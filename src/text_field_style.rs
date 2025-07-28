use bevy::{
    color::{
        Color,
        palettes::css::{GRAY, WHITE},
    },
    ecs::{
        component::Component,
        hierarchy::Children,
        query::{Changed, With},
        system::Query,
    },
    text::{TextColor, TextFont},
};

use crate::text_field::{TextField, TextFieldPosition};

#[derive(Component)]
pub struct TextFieldStyle {
    pub color: Color,
    pub select_color: Color,
    pub placeholder_color: Color,
    pub font: TextFont,
}

impl TextFieldStyle {
    pub fn get_text_style(&self) -> (TextColor, TextFont) {
        (TextColor(self.color), self.font.clone())
    }

    pub fn get_select_style(&self) -> (TextColor, TextFont) {
        (TextColor(self.select_color), self.font.clone())
    }
}

impl Default for TextFieldStyle {
    fn default() -> Self {
        Self {
            color: Color::Srgba(WHITE.into()),
            select_color: Color::Srgba(GRAY.into()),
            placeholder_color: Color::Srgba(GRAY.into()),
            font: TextFont::default(),
        }
    }
}

pub(crate) fn text_style_changed(
     field_style: Query<(&Children, &TextFieldStyle), (With<TextField>, Changed<TextFieldStyle>)>,
    mut chile_style: Query<(&mut TextFont, &mut TextColor, &TextFieldPosition)>,
) {
    for (children, style) in  field_style.iter() {
        for child in children.iter() {
            let (mut font, mut color, position) = chile_style.get_mut(*child).unwrap();

            match position {
                TextFieldPosition::Back | TextFieldPosition::Front => {
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
