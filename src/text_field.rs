use std::time::Instant;

use bevy::{
    color::{palettes::css::{GRAY, WHITE}, Color, Srgba},
    ecs::{
        component::Component, 
        entity::Entity, 
        hierarchy::Children, 
        observer::Trigger, 
        query::{Added, Changed, With}, 
        resource::Resource, 
        system::{Commands, Query, Res, ResMut}
    }, 
    input::{mouse::MouseButton, ButtonInput}, 
    picking::{events::{Out, Over, Pointer}, Pickable}, 
    sprite::Sprite, 
    text::{Text2d, TextColor, TextFont, TextSpan}, 
    ui::widget::Text
};

use crate::cursur::TextCursor;

#[derive(Resource)]
pub(crate) struct LastEmoji(pub Option<String>);

#[derive(Debug,Component,Clone)]
 pub struct TextField{
    pub is_focuse: bool,
    pub text: String,
    pub max_text: Option<usize>,
    pub select: Select,
    pub is_before_text_ime: bool,
    pub last_change_time: Instant
}

impl Default for TextField{
    fn default() -> Self {
        Self {
            is_focuse: false,
            text: String::new(),
            max_text: None,
            select: Select(0, 0, None),
            is_before_text_ime: false,
            last_change_time: Instant::now(),
        }
    }
}

#[derive(Debug,Clone, Copy)]
pub struct Select(pub usize,pub usize,pub Option<usize>);

impl Select {
    pub(crate) fn is_close(&self) -> bool{
        self.0 == self.1
    }

    pub(crate) fn is_open_left(&self) -> bool{
        if let Some(last) = self.2{
            return last != self.0;
        }
        false
    }

    pub(crate) fn is_open_right(&self) -> bool{
        if let Some(last) = self.2{
            return last != self.1;
        }
        false
    }
}

#[derive(Component)]
pub(crate) struct SelectChild;

impl TextField {
    pub fn new(is_foucs: bool) -> (Self,TextCursor,Text){
        
        (
            TextField { 
                is_focuse: is_foucs,
                ..Default::default()
            },
            TextCursor::default(),
            Text::default()
        )
        
    }
    pub fn new2d(is_foucs: bool) -> (Self,TextCursor,Text2d,Sprite,Pickable){
        
        (
            TextField { 
                is_focuse: is_foucs,
                ..Default::default()
            },
            TextCursor::default(),
            Text2d::default(),
            Sprite {
                color: Srgba::new(0.0, 0.0, 0.0, 0.0).into(),
                ..Default::default()
            },
            Pickable::default()
        )
        
    }
}

pub(crate) fn add_textfield_child(
    mut commands: Commands,
    q_add_textfield: Query<(Entity,Option<&TextStyle>),Added<TextField>> ,
){
    for (parent,op_style) in q_add_textfield.iter(){

        let text_style = match op_style {
            Some(text_style) => {
                text_style
            }
            None => {
                &TextStyle::default()
            }
        };

        let front = commands.spawn((
            TextFieldPosition::Front,
            TextSpan::new(""),
            text_style.get_text_style()
        )).id();

        let selection = commands.spawn((
            TextFieldPosition::Select(String::new()),
            SelectChild,
            TextSpan::new(""),
            text_style.get_select_style()
        )).id();

        let back = commands.spawn((
            TextFieldPosition::Back,
            TextSpan::new(""),
            text_style.get_text_style()
        )).id();

        commands.entity(parent)
        .add_children(&[
            front,
            selection,
            back
        ])
        .observe(change_remove_cursur_over_field)
        .observe(change_add_cursur_over_field);

    }
}

#[derive(Resource)]
pub struct OverField(pub Option<Entity>);

fn change_add_cursur_over_field(
    trigger: Trigger<Pointer<Over>>,
    mut over_field: ResMut<OverField>
) {
    over_field.0 = Some(trigger.target);
    println!("Changed!!");
}

fn change_remove_cursur_over_field(
    trigger: Trigger<Pointer<Out>>,
    mut over_field: ResMut<OverField>
) {
    if let Some(entity) = over_field.0{
        if entity == trigger.target{
            over_field.0 = None;
            println!("Out")
        }
    }
}

pub(crate) fn change_focuse(
    mut q_text_field: Query<(&mut TextField,Entity)>,
    over_field: Res<OverField>,
    button_input: Res<ButtonInput<MouseButton>>
) {
    let focus = over_field.0;
    for click in button_input.get_just_pressed(){
        if *click != MouseButton::Left {
            continue;
        }
        for (mut field, entity) in q_text_field.iter_mut(){
            if focus == None {
                field.is_focuse = false;
            }
            else {
                field.is_focuse = entity == focus.unwrap();
            }
        }
    }
}

#[derive(Component,Debug,PartialEq,Clone)]
pub(crate) enum TextFieldPosition {
    Front,
    Select(String),
    Back
}

#[derive(Component)]
pub struct TextStyle{
    pub color: Color,
    pub cusor_color: Color,
    pub font: TextFont
}

impl TextStyle{
    fn get_text_style(&self) -> (TextColor,TextFont){
        (
            TextColor(self.color),
            self.font.clone()
        )
    }

    fn get_select_style(&self) -> (TextColor,TextFont){
        (
            TextColor(self.cusor_color),
            self.font.clone()
        )
    }
}

impl Default for TextStyle{
    
    fn default() -> Self {
        Self {
            color: Color::Srgba(WHITE.into()),
            cusor_color: Color::Srgba(GRAY.into()),
            font: TextFont::default()
        }
    }
}

pub(crate) fn text_style_changed(
    fiedl_style: Query<(&Children,&TextStyle),(With<TextField>,Changed<TextStyle>)>,
    mut chile_style: Query<(&mut TextFont,&mut TextColor,&TextFieldPosition)>
) {
    
    for (children,style) in fiedl_style.iter(){

        for child in children.iter(){
            let (mut font,mut color,position) = chile_style.get_mut(*child).unwrap();

            match position {
                TextFieldPosition::Back |
                TextFieldPosition::Front => {
                    let list = style.get_text_style();
                    *font = list.1;
                    *color = list.0;
                }
                TextFieldPosition::Select(_) => {
                    let list = style.get_select_style();
                    *font = list.1;
                    *color = list.0;
                }
            }
            
        }

    }

}