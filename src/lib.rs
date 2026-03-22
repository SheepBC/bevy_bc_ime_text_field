#![cfg(windows)]
//! # bevy_bc_ime_text_field
//!
//! [![crates.io](https://img.shields.io/crates/v/bevy_bc_ime_text_field)](https://crates.io/crates/bevy_bc_ime_text_field)
//! [![docs.rs](https://docs.rs/bevy_bc_ime_text_field/badge.svg)](https://docs.rs/bevy_bc_ime_text_field)
//!
//! A simple IME-compatible text field plugin for **Bevy** (Windows only).
//! Supports both UI and 2D text input, with full Korean/Japanese/Chinese IME support.
//!
//! ## ✨ Features
//!
//! - IME support (Windows 10 & 11)
//! - Works with **2D** (`Text2d`) and **UI** (`Text`) text fields
//! - **Undo / Redo** (`Ctrl+Z` / `Ctrl+Y`)
//! - Text **selection** (mouse & keyboard)
//! - **Copy / Paste / Cut** (`Ctrl+C` / `Ctrl+V` / `Ctrl+X`)
//! - **Select All** (`Ctrl+A`)
//! - **Password** style masking
//! - Max length limit
//! - Events: [`TextEdited`], [`EnterEvent`]
//!
//! ## 📦 Installation
//!
//! ```toml
//! [dependencies]
//! bevy_bc_ime_text_field = "0.1"
//! ```
//!
//! ### Version Compatibility
//!
//! | `bevy` | `bevy_bc_ime_text_field` |
//! |--------|--------------------------|
//! | `0.16` | `0.0.1` ~ `0.0.5`        |
//! | `0.18` | `0.1` ~                  |
//!
//! ## 🚀 Quick Start
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
//!         .add_plugins(ImeTextFieldPlugin) // ✅ Required
//!         .add_systems(Startup, setup)
//!         .run();
//! }
//!
//! fn setup(mut commands: Commands) {
//!     commands.spawn(Camera2d);
//!
//!     // 2D text field
//!     commands.spawn(TextField::new2d(true));
//!
//!     // UI text field
//!     commands.spawn(TextField::new(true));
//!
//!     // With custom style
//!     commands.spawn((
//!         TextField::new2d(false),
//!         TextFieldStyle {
//!             color: PINK.into(),
//!             ..Default::default()
//!         },
//!     ));
//!
//!     // Manual setup
//!     commands.spawn((
//!         TextField::default(),     // ✅ Required
//!         TextFieldInfo::default(),
//!         TextFieldStyle::default(),
//!         TextFieldInput::default(),
//!         Text::default(),          // UI mode
//!         // Text2d::default(),     // 2D mode
//!         // Sprite::default(),
//!         // Pickable::default(),
//!     ));
//! }
//! ```
//!
//! ## 🔔 Events
//!
//! Two events are triggered directly on the [`TextField`] entity:
//!
//! - [`TextEdited`] — fires whenever the text changes
//! - [`EnterEvent`] — fires when Enter is pressed
//!
//! ## ⌨️ Keyboard Shortcuts
//!
//! | Shortcut | Action |
//! |----------|--------|
//! | `Ctrl+Z` | Undo |
//! | `Ctrl+Y` / `Ctrl+Shift+Z` | Redo |
//! | `Ctrl+C` | Copy |
//! | `Ctrl+V` | Paste |
//! | `Ctrl+X` | Cut |
//! | `Ctrl+A` | Select All |
//! | `Ctrl+Backspace` | Delete word |
//! | `Shift+Arrow` | Extend selection |
//!
//! ## ⚠️ Notes
//!
//! - Windows only
//! - Do **not** add [`TextFieldInfo`] manually when using `new()` / `new2d()`
//!
//! ## 📄 License
//!
//! Licensed under either of:
//!
//! - MIT License
//! - Apache License, Version 2.0
//!
//! at your option.
use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::{query::With, system::Query},
    window::{PrimaryWindow, Window},
};
use bevy::app::PostUpdate;
use bevy::text::{Strikethrough, StrikethroughColor, Underline, UnderlineColor};
use input::input::{change_sprite_size, reload_text_fields, update_input};
use selection::{update_cursor, update_text_cursor_timer};
use text_field::{
    LastEmoji, OverField, add_essential_component, add_text_field_child, change_focuse,
};
use text_field_style::text_style_changed;
use crate::text_field::{change_placeholder_state};
use crate::text_field_style::{text_deco, text_remove_deco};

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
            .add_systems(Update, add_text_field_child)
            .add_systems(Update, change_placeholder_state)
            .add_systems(Update, update_cursor)
            .add_systems(Update, change_sprite_size)
            .add_systems(Update, change_focuse)
            .add_systems(Update, add_essential_component)
            .add_systems(Update, reload_text_fields)
            .add_systems(PostUpdate,text_deco::<Underline>)
            .add_systems(PostUpdate,text_deco::<UnderlineColor>)
            .add_systems(PostUpdate,text_deco::<Strikethrough>)
            .add_systems(PostUpdate,text_deco::<StrikethroughColor>)
            .add_systems(Update,text_remove_deco::<Underline>)
            .add_systems(Update,text_remove_deco::<UnderlineColor>)
            .add_systems(Update,text_remove_deco::<Strikethrough>)
            .add_systems(Update,text_remove_deco::<StrikethroughColor>);
    }
}

fn setup(mut q_window: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = q_window.single_mut().unwrap();
    window.ime_enabled = true;
}
