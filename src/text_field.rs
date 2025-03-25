use std::time::Duration;

use bevy::{
    color::{palettes::css::{GRAY, WHITE}, Color},
    ecs::{component::Component, entity::Entity, query::{Added, Changed, With}, system::{Commands, Query, Resource}},
    hierarchy::{BuildChildren, Children},
    text::{Text2d, TextColor, TextFont, TextSpan},
    time::{Timer, TimerMode},
    ui::widget::Text
};

use crate::cursur::TextCursor;

#[derive(Resource)]
pub(crate) struct LastEmoji(pub Option<String>);

#[derive(Debug,Component,Clone)]
 pub struct TextField{
    pub is_focuse: bool,
    pub text: String,
    pub(crate)  select: Select,
    pub(crate) is_before_text_ime: bool,
}

#[derive(Debug,Clone, Copy)]
pub struct Select(pub usize,pub usize,pub Option<usize>);

impl Select {
    pub(crate) fn is_close(&self) -> bool{
        self.0 == self.1
    }

    pub(crate) fn is_open_left(&self) -> bool{
        if let Some(last) = self.2{
            return last != self.0;
        }
        false
    }

    pub(crate) fn is_open_right(&self) -> bool{
        if let Some(last) = self.2{
            return last != self.1;
        }
        false
    }
}

#[derive(Component)]
struct SelectChild;

impl TextField {
    pub fn new(is_foucs: bool) -> (Self,TextCursor,Text){
        
        (
            TextField { 
                is_focuse: is_foucs,
                text: String::new(),
                select: Select(0,0,None),
                is_before_text_ime: false
            },
            TextCursor {
                is_see: true,
                timer: Timer::new(Duration::from_secs_f32(100000.0), TimerMode::Repeating)
            },
            Text::default()
        )
        
    }
    pub fn new2d(is_foucs: bool) -> (Self,TextCursor,Text2d){
        
        (
            TextField { 
                is_focuse: is_foucs,
                text: String::new(),
                select: Select(0,0,None),
                is_before_text_ime: false 
            },
            TextCursor {
                is_see: false,
                timer: Timer::new(Duration::from_secs_f32(0.5), TimerMode::Repeating)
            },
            Text2d::default()
        )
        
    }
}

pub(crate) fn add_textfield_child(
    mut commands: Commands,
    q_add_textfield: Query<(Entity,Option<&TextStyle>),Added<TextField>> ,
){
    for (parent,op_style) in q_add_textfield.iter(){

        let text_style = match op_style {
            Some(text_style) => {
                text_style
            }
            None => {
                &TextStyle::default()
            }
        };

        let front = commands.spawn((
            TextFieldPosition::Front,
            TextSpan::new(""),
            text_style.get_text_style()
        )).id();

        let selection = commands.spawn((
            TextFieldPosition::Select(String::new()),
            SelectChild,
            TextSpan::new(""),
            text_style.get_select_style()
        )).id();

        let back = commands.spawn((
            TextFieldPosition::Back,
            TextSpan::new(""),
            text_style.get_text_style()
        )).id();

        commands.entity(parent).add_children(&[
            front,
            selection,
            back
        ]);

    }
}

#[derive(Component,Debug,PartialEq)]
pub(crate)  enum TextFieldPosition {
    Front,
    Select(String),
    Back
}

#[derive(Component)]
pub struct TextStyle{
    pub color: Color,
    pub cusor_color: Color,
    pub font: TextFont
}

impl TextStyle{
    fn get_text_style(&self) -> (TextColor,TextFont){
        (
            TextColor(self.color),
            self.font.clone()
        )
    }

    fn get_select_style(&self) -> (TextColor,TextFont){
        (
            TextColor(self.cusor_color),
            self.font.clone()
        )
    }
}

impl Default for TextStyle{
    
    fn default() -> Self {
        Self {
            color: Color::Srgba(WHITE.into()),
            cusor_color: Color::Srgba(GRAY.into()),
            font: TextFont::default()
        }
    }
}

pub(crate) fn text_style_changed(
    fiedl_style: Query<(&Children,&TextStyle),(With<TextField>,Changed<TextStyle>)>,
    mut chile_style: Query<(&mut TextFont,&mut TextColor,&TextFieldPosition)>
) {
    
    for (children,style) in fiedl_style.iter(){

        for child in children.iter(){
            let (mut font,mut color,position) = chile_style.get_mut(*child).unwrap();

            match position {
                TextFieldPosition::Back |
                TextFieldPosition::Front => {
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