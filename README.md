# Bevy_BC_ime_text_field

A simple IME-compatible text field plugin for **Bevy** (Windows only).  
Supports both UI and 2D text input.

## ✨ Features

- IME (Input Method Editor) text input support (Windows)
- 2D & UI-compatible text fields

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
bevy_bc_ime_text_field = "0.0.1"
```
| `bevy` Version | `bevy_bc_ime_text_field` Version |
| -------------- | -------------------------------- | 
| `0.16`         | `0.0.1`                          |


# 🚀 Example
```rust
use bevy::color::palettes::css::PINK;
use bevy::prelude::*;
use bevy_bc_ime_text_field::*;
use bevy_bc_ime_text_field::text_field::*;
use bevy_bc_ime_text_field::text_field_style::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(ImeTextFieldPlugin)//✅required
    .add_systems(Startup,setup)
    .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
){
    
    commands.spawn(Camera2d);

    //✅collects
    commands.spawn(
        TextField::new2d(true),
    );

    commands.spawn(
        TextField::new(true)
    );

    commands.spawn((
        TextField::new2d(false),
        TextFieldStyle {
            color: PINK.into(),
            ..Default::default()
        }
    ));

    commands.spawn((
        TextField::default(),//✅required
        TextFieldInfo::default(),
        TextFieldStyle::default(),
        TextFieldInput::default(),

        //text
        Text::default(),

        //text2D
        /*
        Text2d::default(), //✅required
        Sprite::default(),
        Pickable::default(),
         */
    ));

    //❌incorrect
    commands.spawn((
        TextField::new(true),
        TextFieldInfo::default(),
    ));

}


```