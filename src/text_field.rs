use std::time::Instant;

use crate::{selection::TextFieldSelection, text_field_style::TextFieldStyle, tool::split_text};
use bevy::prelude::{Changed, Children, With};
use bevy::text::TextColor;
use bevy::{
    color::Srgba,
    ecs::{
        component::Component,
        entity::Entity,
        observer::Trigger,
        query::Added,
        resource::Resource,
        system::{Commands, Query, Res, ResMut},
    },
    input::{mouse::MouseButton, ButtonInput},
    picking::{
        events::{Out, Over, Pointer},
        Pickable,
    },
    sprite::Sprite,
    text::{Text2d, TextSpan},
    ui::widget::Text,
};

const TRANSPARENT: Srgba = Srgba::new(0.0, 0.0, 0.0, 0.0);

#[derive(Resource)]
pub(crate) struct LastEmoji(pub Option<String>);

#[derive(Debug,Clone)]
enum Change{
    Add(Select,String),
    Delete(Select,String)
}

#[derive(Debug, Component, Clone)]
pub struct TextField {
    pub text: String,
    pub select: Select,
    command: undo_2::Commands<Change>
}

impl Default for TextField {
    fn default() -> Self {
        Self {
            text: String::new(),
            select: Select(0, 0, None),
            command: undo_2::Commands::new()
        }
    }
}

impl TextField {
    pub fn new(
        focus: bool,
    ) -> (
        Self,
        TextFieldSelection,
        TextFieldInfo,
        TextFieldInput,
        Text,
    ) {
        (
            TextField::default(),
            TextFieldSelection::default(),
            TextFieldInfo {
                focus: focus,
                ..Default::default()
            },
            TextFieldInput::default(),
            Text::default(),
        )
    }
    pub fn new2d(
        focus: bool,
    ) -> (
        Self,
        TextFieldSelection,
        TextFieldInfo,
        TextFieldInput,
        Text2d,
        Sprite,
        Pickable,
    ) {
        (
            TextField::default(),
            TextFieldSelection::default(),
            TextFieldInfo {
                focus: focus,
                ..Default::default()
            },
            TextFieldInput::default(),
            Text2d::default(),
            Sprite {
                color: TRANSPARENT.into(),
                ..Default::default()
            },
            Pickable::default(),
        )
    }
}
/// start, end, startPoint
#[derive(Debug, Clone, Copy)]
pub struct Select(pub usize, pub usize, pub Option<usize>);

impl Select {
    pub(crate) fn is_close(&self) -> bool {
        self.0 == self.1
    }

    pub(crate) fn is_open_left(&self) -> bool {
        if let Some(last) = self.2 {
            return last != self.0;
        }
        false
    }

    pub(crate) fn is_open_right(&self) -> bool {
        if let Some(last) = self.2 {
            return last != self.1;
        }
        false
    }
}

#[derive(Component)]
pub(crate) struct SelectChild;

#[derive(Component)]
pub struct TextFieldInfo {
    pub focus: bool,
    pub max_length: Option<usize>,
    pub placeholder: Option<String>,
    pub changeable_focus_with_click: bool,
}

impl Default for TextFieldInfo {
    fn default() -> Self {
        Self {
            focus: true,
            max_length: None,
            placeholder: None,
            changeable_focus_with_click: true,
        }
    }
}

#[derive(Component)]
pub struct Placeholder;

pub fn change_placeholder_state(
    field: Query<(&TextField,&TextFieldInfo,&Children),Changed<TextField>>,
    mut placeholder: Query<&mut TextSpan,With<Placeholder>>

){
    for (field,info,children) in field{

        let placeholder_text = match &info.placeholder {
            Some(text) => text,
            _ => ""
        };

        for child in children{
            if let Ok(mut text) = placeholder.get_mut(*child){
                text.0 = if field.text.len() == 0 {placeholder_text.to_string()} else { "".to_string() };
            }
        }
    }
}

#[derive(Component)]
pub struct TextFieldInput {
    pub is_last_text_ime: bool,
    pub last_change_time: Instant,
}

impl Default for TextFieldInput {
    fn default() -> Self {
        Self {
            is_last_text_ime: false,
            last_change_time: Instant::now(),
        }
    }
}

pub(crate) fn add_text_field_child(
    mut commands: Commands,
    q_add_text_field: Query<(Entity, &TextField, &TextFieldInfo, Option<&TextFieldStyle>), Added<TextField>>,
) {
    for (parent, field,info, op_style) in q_add_text_field.iter() {
        let list = split_text(field.text.clone(), field.select);

        let text_style = match op_style {
            Some(style) => style,
            _ => &TextFieldStyle::default()
        };

        let front = commands
            .spawn((
                TextFieldPosition::Front,
                TextSpan::new(list[0].clone()),
                text_style.get_text_style(),
            ))
            .id();

        let selection = commands
            .spawn((
                TextFieldPosition::Select(String::new()),
                SelectChild,
                TextSpan::new(list[1].clone()),
                text_style.get_select_style(),
            ))
            .id();

        let back = commands
            .spawn((
                TextFieldPosition::Back,
                TextSpan::new(list[2].clone()),
                text_style.get_text_style(),
            ))
            .id();

        let text = if let Some(tx) = &info.placeholder { tx }else { "" };

        let placeholder = commands.spawn((
            TextSpan::new(text),
            text_style.font.clone(),
            TextColor(text_style.placeholder_color),
            Placeholder
        )).id();

        commands
            .entity(parent)
            .add_children(&[front, selection, back, placeholder])
            .observe(change_remove_cursor_over_field)
            .observe(change_add_cursor_over_field);
    }
}

#[derive(Resource)]
pub struct OverField(pub Option<Entity>);

fn change_add_cursor_over_field(
    trigger: Trigger<Pointer<Over>>,
    mut over_field: ResMut<OverField>,
) {
    over_field.0 = Some(trigger.target);
}

fn change_remove_cursor_over_field(
    trigger: Trigger<Pointer<Out>>,
    mut over_field: ResMut<OverField>,
) {
    if let Some(entity) = over_field.0 {
        if entity == trigger.target {
            over_field.0 = None;
        }
    }
}

pub(crate) fn change_focuse(
    mut q_text_field: Query<(&mut TextFieldInfo, Entity)>,
    over_field: Res<OverField>,
    button_input: Res<ButtonInput<MouseButton>>,
) {
    let focus = over_field.0;
    for click in button_input.get_just_pressed() {
        if *click != MouseButton::Left {
            continue;
        }
        for (mut info, entity) in q_text_field.iter_mut() {
            if !info.changeable_focus_with_click {
                continue;
            }
            
            info.focus = if focus == None {
                false
            } else {
                entity == focus.unwrap()
            };
        }
    }
}

#[derive(Component, Debug, PartialEq, Clone)]
pub(crate) enum TextFieldPosition {
    Front,
    Select(String),
    Back,
}

pub(crate) fn add_essential_component(
    mut commands: Commands,
    q_textfield: Query<
        (
            Entity,
            Option<&TextFieldSelection>,
            Option<&TextFieldInfo>,
            Option<&TextFieldInput>,
            Option<&Text>,
            Option<&Text2d>,
            Option<&Sprite>,
            Option<&Pickable>,
        ),
        Added<TextField>,
    >,
) {
    for (entity, op_selection, op_info, op_input, op_text, op_text2d, op_sprite, op_pick) in
        q_textfield.iter()
    {
        let mut entity_commands = commands.get_entity(entity).unwrap();

        if op_selection.is_none() {
            entity_commands.insert(TextFieldSelection::default());
        }
        if op_info.is_none() {
            entity_commands.insert(TextFieldInput::default());
        }
        if op_input.is_none() {
            entity_commands.insert(TextFieldInput::default());
        }
        if op_text.is_none() && op_text2d.is_none() {
            entity_commands.insert(Text::default());
        }
        if !op_text2d.is_none() {
            if op_sprite.is_none() {
                entity_commands.insert(Sprite {
                    color: TRANSPARENT.into(),
                    ..Default::default()
                });
            }
            if op_pick.is_none() {
                entity_commands.insert(Pickable::default());
            }
        }
    }
}
