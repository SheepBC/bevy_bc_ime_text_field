use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::{query::With, system::Query},
    window::{PrimaryWindow, Window},
};
use input::input::{change_sprite_size, reload_text_fields, update_input};
use selection::{update_cursor, update_text_cursor_timer};
use text_field::{
    LastEmoji, OverField, add_essential_component, add_textfield_child, change_focuse,
};
use text_field_style::text_style_changed;

pub mod event;
pub mod selection;
pub mod text_field;
pub mod text_field_style;
pub(crate) mod tool;
pub struct ImeTextFieldPlugin;
mod input;

impl Plugin for ImeTextFieldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LastEmoji(None))
            .insert_resource(OverField(None))
            .add_systems(Startup, setup)
            .add_systems(Update, update_input)
            .add_systems(Update, update_text_cursor_timer)
            .add_systems(Update, text_style_changed)
            .add_systems(Update, add_textfield_child)
            .add_systems(Update, update_cursor)
            .add_systems(Update, change_sprite_size)
            .add_systems(Update, change_focuse)
            .add_systems(Update, add_essential_component)
            .add_systems(Update, reload_text_fields);
    }
}

fn setup(mut q_window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = q_window.single_mut().unwrap();
    window.ime_enabled = true;
}
