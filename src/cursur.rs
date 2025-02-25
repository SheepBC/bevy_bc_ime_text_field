use bevy::{
    ecs::{component::Component, entity::Entity, event::EventReader, observer::Trigger, query::With, system::{Commands, Query, Res,Single}}, hierarchy::Children, input::{mouse::MouseButtonInput, ButtonState}, math::Vec2, render::camera::Camera, text::{TextFont, TextLayoutInfo, TextSpan}, time::{Time, Timer}, transform::components::{GlobalTransform, Transform}, window::{PrimaryWindow, Window}
};

use crate::{event::{ChangedSelect, PickingTextField}, input::update_text_field, text_field::{Select, TextField, TextFieldPosition}, tool::{is_in_box, splite_text}};


#[derive(Component)]
pub struct TextCursor{
    pub is_see: bool,
    pub timer: Timer
}

pub(crate) fn update_text_cursor(
    time: Res<Time>,
    mut q_cursors:Query<&mut TextCursor>
) {
    for mut text_cursor in q_cursors.iter_mut(){
        text_cursor.timer.tick(time.delta());

        if text_cursor.timer.finished() {
            text_cursor.is_see = !text_cursor.is_see;
        }
    }
}

pub(crate) fn update_picking_text_field(
    mut commands: Commands,
    mut evr_mouse: EventReader<MouseButtonInput>,
    q_text_field: Query<(&TextField,Entity,&TextLayoutInfo,&Transform)>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    s_camera: Single<(&Camera,&GlobalTransform)>
) {

    let (camera,camera_global_transform) = s_camera.into_inner();

    for mouse in evr_mouse.read(){
        if let Some(cusror_window_position) = q_windows.single().cursor_position(){
            let cursor_pos = camera.viewport_to_world_2d(camera_global_transform, cusror_window_position).unwrap();

            for (field,entity,layout,transporm) in q_text_field.iter(){
                if is_in_box( Vec2 { x: transporm.translation.x, y: transporm.translation.y }, layout.size,cursor_pos){
                    commands.get_entity(entity).unwrap().trigger(PickingTextField {
                        entity: entity,
                        text_field: field.clone(),
                        cursor_position: cursor_pos,
                        cusrsor_click: *mouse
                    });
                }
            }
        }
    }
}

pub(crate) fn clicked_change_select(
    trigger:Trigger<PickingTextField>,
    mut commands: Commands,
    q_layout: Query<&TextLayoutInfo>,
    q_trans: Query<&Transform>,
    q_style: Query<&TextFont>,
    mut q_field: Query<&mut TextField>,
    q_cursor: Query<&TextCursor>
) {
    if trigger.cusrsor_click.state == ButtonState::Pressed {return;}

    let trans = q_trans.get(trigger.entity).unwrap();
    let field_pos = Vec2 {x: trans.translation.x, y: trans.translation.y};
    let pos_glyph = q_layout.get(trigger.entity).unwrap();
    let cursor_field_pos = trigger.cursor_position - field_pos + pos_glyph.size/2.0;

    let cursor = q_cursor.get(trigger.entity).unwrap();
    let heigh = q_style.get(trigger.entity).unwrap().font_size;

    for (index,glyph) in pos_glyph.glyphs.clone().iter().enumerate(){

        if is_in_box(glyph.position, Vec2 { x: glyph.size.x, y: heigh }, cursor_field_pos){

            let mut field = q_field.get_mut(trigger.entity).unwrap();
            
            let mut change_select = index;
            if cursor.is_see && field.select.clone().is_close(){
                if index < field.select.0 {
                    change_select += 1;
                }
            }
            field.select = Select(change_select,change_select);
            commands.get_entity(trigger.entity).unwrap().trigger(ChangedSelect {
                entity: trigger.entity,
                text_fiedl: field.into_inner().clone()
            });
        }

    }
}

pub(crate) fn changed_select(
    trigger: Trigger<ChangedSelect>,
    q_text_field: Query<(&Children,&TextCursor)>,
    mut q_child_text: Query<(&mut TextSpan,&mut TextFieldPosition)>,
) {
    let (children,cursor) = q_text_field.get(trigger.entity).unwrap();

    let text_list = splite_text(trigger.text_fiedl.text.clone(), trigger.text_fiedl.select);

    update_text_field(
        children,
        &mut q_child_text,
        true,
        text_list,
        cursor
    );
}