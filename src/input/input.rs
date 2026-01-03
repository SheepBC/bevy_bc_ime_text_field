use bevy::prelude::{MessageReader, Text2d};
use std::{cmp::min, time::Instant};

use bevy::{
    ecs::{
        entity::Entity,
        hierarchy::Children,
        query::{Changed, With},
        system::{Commands, Query, Res, ResMut},
    },
    input::{
        keyboard::{KeyCode, KeyboardInput},
        ButtonInput,
    },
    sprite::Sprite,
    text::{ TextLayoutInfo, TextSpan},
    window::Ime,
};

use crate::{
    event::{EnterEvent, TextEdited},
    text_field::{Select, TextField, TextFieldInfo, TextFieldInput, TextFieldPosition},
    tool::{split_text, ToolString},
    LastEmoji,
};
use crate::text_field_style::{change_passwd, TextFieldStyle};
use super::{
    select_input::{
        get_select_informtype, get_select_shift_informtype, set_select_text_list, SelectType,
    },
    text_input::{get_text_inform_type, set_text_list, KeyType},
};
//메인 함수
pub(crate) fn update_input(
    mut commands: Commands,
    last_emoji: ResMut<LastEmoji>,
    evr_ime: MessageReader<Ime>,
    evr_kbd: MessageReader<KeyboardInput>,
    res_kbd: Res<ButtonInput<KeyCode>>,
    mut q_text_field: Query<(Entity, &mut TextField, &TextFieldInfo, &mut TextFieldInput)>,
) {
    let key_list = get_keys(last_emoji, evr_ime, evr_kbd, res_kbd);

    for (entity, mut text_field, field_info, mut input) in q_text_field.iter_mut() {
        if !field_info.focus {
            continue;
        }

        let mut is_text_change = false;
        let mut is_enter = false;

        if !key_list.is_empty() {
            set_text_field(&key_list, &mut text_field, &mut input, field_info);
            is_text_change = true;
            is_enter = key_list.contains(&KeyInform {
                is_ime: None,
                is_finish: true,
                key: InformType::KeyType(KeyType::Text("\n".to_string())),
            });
        }

        //event
        if is_text_change {
            commands.get_entity(entity).unwrap().trigger(|entity| {TextEdited {
                text_field: text_field.clone(),
                entity: entity,
            }});
            input.last_change_time = Instant::now()
        }

        if is_enter {
            commands.get_entity(entity).unwrap().trigger(|entity| {EnterEvent {
                text_field: text_field.clone(),
                entity: entity,
            }});
        }
    }
}

pub(crate) fn reload_text_fields(
    q_field_inform: Query<(&TextField,&TextFieldStyle, &Children), Changed<TextField>>,
    mut q_child_text: Query<(&mut TextSpan, &mut TextFieldPosition)>,
) {
    for (text_field,style, children) in q_field_inform.iter() {
        reload_text_field(text_field, children,style, &mut q_child_text);
    }
}

pub(crate) fn reload_text_field(
    text_field: &TextField,
    children: &Children,
    style: &TextFieldStyle,
    q_child_text: &mut Query<(&mut TextSpan, &mut TextFieldPosition)>,
) {
    let text_list = split_text(text_field.text.clone(), text_field.select);

    for child in children.iter() {
        if let Ok((mut span, mut position)) = q_child_text.get_mut(*child) {
            match *position {
                TextFieldPosition::Front => {
                    if style.password_style {
                        **span = change_passwd(text_list[0].clone());
                    }else {
                        **span = text_list[0].clone();   
                    }
                }
                TextFieldPosition::Select(_) => {
                    if text_list[1].is_empty() {
                        **span = "|".to_string();
                    } else {
                        if style.password_style {
                            **span = "|".to_string() + &change_passwd(text_list[1].clone()) + &"|";
                        }else {
                            **span = "|".to_string() + &text_list[1] + &"|";
                        }
                    }
                    if text_list[1].chars().count() != 0 {
                        *position = TextFieldPosition::Select(text_list[1].clone());
                    } else {
                        *position = TextFieldPosition::Select(String::new());
                    }
                }
                TextFieldPosition::Back => {
                    if style.password_style {
                        **span = change_passwd(text_list[2].clone());
                    }else {
                        **span = text_list[2].clone();
                    }
                }
            }
        }
    }
}

pub(crate) fn get_keys(
    mut last_emoji: ResMut<LastEmoji>,
    mut evr_ime: MessageReader<Ime>,
    mut evr_kbd: MessageReader<KeyboardInput>,
    res_kbd: Res<ButtonInput<KeyCode>>,
) -> Vec<KeyInform> {
    let mut list: Vec<KeyInform> = Vec::new();

    for ime in evr_ime.read() {
        match ime {
            Ime::Commit {value,.. } => {

                list.push(KeyInform {
                    is_ime: None,
                    is_finish: true,
                    key: InformType::KeyType(KeyType::Text(value.clone()))
                });

            }
            Ime::Preedit {value,cursor,..} => {
                if value.is_empty() && !cursor.is_none() {
                    continue;
                }

                let num = if let Some(last) = last_emoji.0.clone() { last.size() }else { 1 };

                last_emoji.0 = if let Some((a,b)) = cursor.clone() {
                    if a+b == 0 {
                        None
                    }
                    else {
                        Some(value.clone())
                    }
                } else {
                    None
                };

                list.push(KeyInform {
                    is_ime: Some(num),
                    is_finish: false,
                    key: InformType::KeyType(KeyType::Text(value.clone()))
                });

            }
            _ => {}
        }
    }
    let key_list = evr_kbd.read();
    let is_pressed_shift = res_kbd.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    let is_pressed_ctrl = res_kbd.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);
    for key in key_list {
        if key.state.is_pressed() {
            let mut add_key: Option<InformType> = None;
            get_text_inform_type(key.logical_key.clone(), &mut add_key, is_pressed_ctrl,is_pressed_shift);
            if is_pressed_shift {
                get_select_shift_informtype(key.logical_key.clone(), &mut add_key, is_pressed_ctrl);
            } else {
                get_select_informtype(key.logical_key.clone(), &mut add_key, is_pressed_ctrl);
            }
            if let Some(key) = add_key {
                list.push(KeyInform {
                    is_ime: None,
                    is_finish: true,
                    key: key,
                });
            }
        }
    }

    list
}

pub(crate) fn set_text_field(
    key_list: &Vec<KeyInform>,
    text_field: &mut TextField,
    input: &mut TextFieldInput,
    field_info: &TextFieldInfo,
) {
    let mut text_list = split_text(text_field.text.clone(), text_field.select.clone());
    for key_inform in key_list {

        if key_inform.is_finish {
            match &key_inform.key {
                InformType::KeyType(key) => {
                    set_text_list(key, &mut text_list, text_field);
                }
                InformType::SelectType(key) => {
                    if set_select_text_list(key, &mut text_list, text_field) {
                        break;
                    }
                }
            }
            input.is_last_text_ime = false;
        }
        else {
            if let Some(mut num) = key_inform.is_ime {
                if !input.is_last_text_ime {
                    num = 0;
                }

                if let InformType::KeyType(KeyType::Text(text)) = &key_inform.key {
                    for _ in 0..num{
                        text_list[0].pop();
                    }
                    text_list[0] += &text;
                    text_list[1] = "".to_string();
                }

                input.is_last_text_ime = true;
            }
        }

        
    }

    let mut change_text = text_list.concat();
    let mut select_start_num = text_list[0].chars().count();
    let mut select_num = text_list[1].chars().count();
    let last_select = if select_num == 0 {
        None
    } else {
        text_field.select.2
    };

    if let Some(num) = field_info.max_length {
        if num < change_text.chars().collect::<Vec<char>>().len() {
            change_text = change_text.slice(num);
            input.is_last_text_ime = false;
            select_start_num = min(num, select_start_num);
            select_num = min(num, select_start_num + select_num) - select_start_num;
        }
    }
    text_field.text = change_text;

    text_field.select = Select(select_start_num, select_start_num + select_num, last_select);
}

#[derive(PartialEq, Eq, Debug)]
pub(crate) struct KeyInform {
    pub(crate) is_ime: Option<usize>,
    pub(crate) is_finish: bool,
    key: InformType,
}

#[derive(PartialEq, Eq,Debug)]
pub(crate) enum InformType {
    KeyType(KeyType),
    SelectType(SelectType),
}

pub(crate) fn change_sprite_size(
    mut q_text2d: Query<
        (&mut Sprite, &TextLayoutInfo),
        (With<Text2d>, With<TextField>, Changed<TextLayoutInfo>),
    >,
) {
    for (mut sprite, info) in q_text2d.iter_mut() {
        sprite.custom_size = Some(info.size);
    }
}

