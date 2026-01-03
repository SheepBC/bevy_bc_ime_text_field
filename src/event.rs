use bevy::{
    ecs::{entity::Entity, event::Event},
    input::mouse::MouseButtonInput,
    math::Vec2,
};
use bevy::prelude::EntityEvent;
use crate::text_field::TextField;

#[derive(EntityEvent)]
pub struct TextEdited {
    pub text_field: TextField,
    pub entity: Entity,
}

#[derive(Debug, Event)]
pub struct PickingTextField {
    pub entity: Entity,
    pub text_field: TextField,
    pub cursor_position: Vec2,
    pub cursor_click: MouseButtonInput,
}

#[derive(Debug, Event)]

pub struct ChangedSelect {
    pub entity: Entity,
    pub text_field: TextField,
}

#[derive(EntityEvent)]
pub struct EnterEvent {
    pub text_field: TextField,
    pub entity: Entity,
}
