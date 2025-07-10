use bevy::{
    ecs::{entity::Entity, event::Event},
    input::mouse::MouseButtonInput,
    math::Vec2,
};

use crate::text_field::TextField;

#[derive(Event)]
pub struct TextEdited {
    pub text_field: TextField,
    pub entity: Entity,
}

#[derive(Debug, Event)]
pub struct PickingTextField {
    pub entity: Entity,
    pub text_field: TextField,
    pub cursor_position: Vec2,
    pub cusrsor_click: MouseButtonInput,
}

#[derive(Debug, Event)]

pub struct ChangedSelect {
    pub entity: Entity,
    pub text_fiedl: TextField,
}

#[derive(Event)]
pub struct EnterEvent {
    pub text_field: TextField,
    pub entity: Entity,
}
