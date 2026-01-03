use crate::input::input::reload_text_field;
use crate::text_field::{TextField, TextFieldPosition};
use bevy::color::palettes::tailwind::{BLUE_200, BLUE_800};
use bevy::prelude::{ParamSet, TextBackgroundColor};
use bevy::text::TextSpan;
use bevy::{
    color::{
        palettes::css::{GRAY, WHITE},
        Color,
    },
    ecs::{
        component::Component,
        hierarchy::Children,
        query::Changed,
        system::Query,
    },
    text::{TextColor, TextFont},
};
use bevy::ui::BackgroundColor;

#[derive(Component)]
pub struct TextFieldStyle {
    pub color: Color,
    pub background : Color,
    pub select_color: Color,
    pub select_background : Color,
    pub placeholder_color: Color,
    pub password_style: bool,
    pub font: TextFont,
}

impl TextFieldStyle {
    pub fn get_text_style(&self) -> (TextColor, TextFont, TextBackgroundColor) {
        (TextColor(self.color), self.font.clone(),TextBackgroundColor(self.background))
    }

    pub fn get_select_style(&self) -> (TextColor, TextFont,TextBackgroundColor) {
        (TextColor(self.select_color), self.font.clone(),TextBackgroundColor(self.select_background))
    }
}

impl Default for TextFieldStyle {
    fn default() -> Self {
        Self {
            color: Color::Srgba(WHITE.into()),
            background: Color::NONE,
            select_color: Color::Srgba(GRAY.into()),
            select_background: Color::Srgba(BLUE_200.into()),
            placeholder_color: Color::Srgba(GRAY.into()),
            password_style: false,
            font: TextFont::default(),
        }
    }
}

pub(crate) fn text_style_changed(
    field_style: Query<(&TextField, &Children, &TextFieldStyle),Changed<TextFieldStyle>>,
    mut parm: ParamSet<(
        Query<(&mut TextFont,&mut TextSpan ,&mut TextColor, &mut TextBackgroundColor, &TextFieldPosition)>,
        Query<(&mut TextSpan, &mut TextFieldPosition)>
    )>
) {
    for (field,children, style) in  field_style.iter() {
        for child in children.iter() {
            if let Ok((mut font, mut span,mut color, mut back, position)) = parm.p0().get_mut(*child){

                match position {
                    TextFieldPosition::Back | TextFieldPosition::Front => {
                        let list = style.get_text_style();
                        *font = list.1;
                        *color = list.0;
                        *back = list.2;
                    }
                    TextFieldPosition::Select(_) => {
                        let list = style.get_select_style();
                        *font = list.1;
                        *color = list.0;
                        *back = list.2;
                    }
                }

                if style.password_style{
                    span.0 = change_passwd(span.0.clone());
                }else {
                    reload_text_field(field,children,style,&mut parm.p1());
                }
            }
        }
    }
}

pub fn change_passwd(text: String) -> String{
    text.chars().map(|_| '•').collect()
}

