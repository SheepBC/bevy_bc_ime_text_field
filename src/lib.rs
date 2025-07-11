//! # bevy_bc_ime_text_field
//!
//! A simple IME-compatible text field plugin for **Bevy** (Windows only).
//! Supports both UI and 2D text input.
//!
//! ---
//!
//! ## ✨ Features
//!
//! - IME (Input Method Editor) text input support (Windows)
//! - 2D & UI-compatible text fields
//!
//! ---
//!
//! ## 📦 Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! bevy_bc_ime_text_field = "0.0.1"
//! ```
//!
//! | `bevy` Version | `bevy_bc_ime_text_field` Version |
//! |----------------|----------------------------------|
//! | `0.16`         | `0.0.1`                          |
//!
//! ---
//!
//! ## 🚀 Example
//!
//! ```rust
//! use bevy::color::palettes::css::PINK;
//! use bevy::prelude::*;
//! use bevy_bc_ime_text_field::*;
//! use bevy_bc_ime_text_field::text_field::*;
//! use bevy_bc_ime_text_field::text_field_style::*;
//!
//! fn main() {
//!     App::new()
//!         .add_plugins(DefaultPlugins)
//!         .add_plugins(ImeTextFieldPlugin) // ✅ required
//!         .add_systems(Startup, setup)
//!         .run();
//! }
//!
//! fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//!
//!     commands.spawn(Camera2d);
//!
//!     // ✅ basic 2D text field
//!     commands.spawn(TextField::new2d(true));
//!
//!     // ✅ basic UI text field
//!     commands.spawn(TextField::new(true));
//!
//!     // ✅ styled 2D text field
//!     commands.spawn((
//!         TextField::new2d(false),
//!         TextFieldStyle {
//!             color: PINK.into(),
//!             ..Default::default()
//!         }
//!     ));
//!
//!     // ✅ full manual setup
//!     commands.spawn((
//!         TextField::default(),          // ✅ required
//!         TextFieldInfo::default(),
//!         TextFieldStyle::default(),
//!         TextFieldInput::default(),
//!         Text::default(),
//!         /*
//!         Text2d::default(),            // ✅ required
//!         Sprite::default(),
//!         Pickable::default(),
//!         */
//!     ));
//!
//!     // ❌ incorrect (missing required components)
//!     commands.spawn((
//!         TextField::new(true),
//!         TextFieldInfo::default(),
//!     ));
//! }
//! ```
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
