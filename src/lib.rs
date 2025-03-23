
use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::{query::With,system::Query},
    window::{PrimaryWindow, Window}
};
use cursur::update_text_cursor;
use input::input::update_input;
use text_field::{add_textfield_child, text_style_changed, LastEmoji};

pub mod cursur;
pub mod text_field;
pub(crate) mod tool;
pub mod event;
pub struct ImeTextFieldPlugin;
mod input;


impl Plugin for ImeTextFieldPlugin{

    fn build(&self, app: &mut App) {
        app
        .insert_resource(LastEmoji(None))
        .add_systems(Startup, setup)
        .add_systems(Update, update_input)
        .add_systems(Update, update_text_cursor)
        .add_systems(Update, text_style_changed)
        .add_systems(Update, add_textfield_child);
    }
}

fn setup(
    mut q_window: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = q_window.single_mut();
    println!("{}",window.ime_enabled);
    window.ime_enabled = true;
}
