use std::{cmp::min, time::Instant};

use bevy::{
    ecs::{
        entity::Entity,
        event::EventReader,
        hierarchy::Children,
        query::{Changed, With},
        system::{Commands, Query, Res, ResMut},
    },
    input::{
        ButtonInput,
        keyboard::{KeyCode, KeyboardInput},
    },
    sprite::Sprite,
    text::{Text2d, TextLayoutInfo, TextSpan},
    window::Ime,
};

use crate::{
    LastEmoji,
    event::{EnterEvent, TextEdited},
    text_field::{Select, TextField, TextFieldInfo, TextFieldInput, TextFieldPosition},
    tool::{ToolString, is_emoji, splite_text},
};

use super::{
    select_input::{
        SelectType, get_select_informtype, get_select_shift_informtype, set_select_text_list,
    },
    text_input::{KeyType, get_text_inform_type, set_text_list},
};
//메인 함수
pub(crate) fn update_input(
    mut commands: Commands,
    last_emoji: ResMut<LastEmoji>,
    evr_ime: EventReader<Ime>,
    evr_kbd: EventReader<KeyboardInput>,
    res_kbd: Res<ButtonInput<KeyCode>>,
    mut q_textfield: Query<(Entity, &mut TextField, &TextFieldInfo, &mut TextFieldInput)>,
) {
    let key_list = get_keys(last_emoji, evr_ime, evr_kbd, res_kbd);

    for (entity, mut text_field, field_info, mut input) in q_textfield.iter_mut() {
        if !field_info.focus {
            continue;
        }

        let mut is_text_change = false;
        let mut is_enter = false;

        if !key_list.is_empty() {
            set_text_field(&key_list, &mut text_field, &mut input, field_info);
            is_text_change = true;
            is_enter = key_list.contains(&KeyInform {
                is_ime: false,
                is_finish: true,
                key: InformType::KeyType(KeyType::Text("\n".to_string())),
            });
        }

        //event
        if is_text_change {
            commands.get_entity(entity).unwrap().trigger(TextEdited {
                text_field: text_field.clone(),
                entity: entity,
            });
            input.last_change_time = Instant::now()
        }

        if is_enter {
            commands.get_entity(entity).unwrap().trigger(EnterEvent {
                text_field: text_field.clone(),
                entity: entity,
            });
        }
    }
}

pub(crate) fn reload_text_fields(
    q_field_inform: Query<(&TextField, &Children), Changed<TextField>>,
    mut q_child_text: Query<(&mut TextSpan, &mut TextFieldPosition)>,
) {
    for (text_field, children) in q_field_inform.iter() {
        reload_text_field(text_field, children, &mut q_child_text);
    }
}

pub(crate) fn reload_text_field(
    text_field: &TextField,
    children: &Children,
    q_child_text: &mut Query<(&mut TextSpan, &mut TextFieldPosition)>,
) {
    let text_list = splite_text(text_field.text.clone(), text_field.select);

    for child in children.iter() {
        if let Ok((mut span, mut position)) = q_child_text.get_mut(*child) {
            match *position {
                TextFieldPosition::Front => {
                    **span = text_list[0].clone();
                }
                TextFieldPosition::Select(_) => {
                    if text_list[1].is_empty() {
                        **span = "|".to_string();
                    } else {
                        **span = "|".to_string() + &text_list[1] + &"|";
                    }
                    if text_list[1].chars().count() != 0 {
                        *position = TextFieldPosition::Select(text_list[1].clone());
                    } else {
                        *position = TextFieldPosition::Select(String::new());
                    }
                }
                TextFieldPosition::Back => {
                    **span = text_list[2].clone();
                }
            }
        }
    }
}

pub(crate) fn get_keys(
    mut last_emoji: ResMut<LastEmoji>,
    mut evr_ime: EventReader<Ime>,
    mut evr_kbd: EventReader<KeyboardInput>,
    res_kbd: Res<ButtonInput<KeyCode>>,
) -> Vec<KeyInform> {
    let mut list: Vec<KeyInform> = Vec::new();

    for ime in evr_ime.read() {
        match ime {
            Ime::Commit { value, .. } => {
                if value == &"".to_string() {
                    continue;
                }
                if value.chars().count() > 1 && last_emoji.0 != None {
                    last_emoji.0 = None;
                    continue;
                }

                let first_ch = value.clone().front_pop().unwrap();
                if is_emoji(first_ch) {
                    list.push(KeyInform {
                        is_ime: false,
                        is_finish: true,
                        key: InformType::KeyType(KeyType::Text(value.clone())),
                    });
                } else {
                    if value.chars().count() > 1 {
                        list.push(KeyInform {
                            is_ime: false,
                            is_finish: true,
                            key: InformType::KeyType(KeyType::Text(value.clone())),
                        });
                    } else {
                        list.push(KeyInform {
                            is_ime: true,
                            is_finish: true,
                            key: InformType::KeyType(KeyType::Text(value.clone())),
                        });
                    }
                }
            }
            Ime::Preedit { value, cursor, .. } => {
                if value == &"".to_string() {
                    continue;
                }
                if cursor != &Some((0, 0)) {
                    let mut text = value.clone();

                    if let Some(last) = &last_emoji.0 {
                        text = value.replacen(last, "", 1);
                    }
                    last_emoji.0 = Some(value.to_string());
                    list.push(KeyInform {
                        is_ime: false,
                        is_finish: true,
                        key: InformType::KeyType(KeyType::Text(text.clone())),
                    });
                    continue;
                }

                list.push(KeyInform {
                    is_ime: true,
                    is_finish: false,
                    key: InformType::KeyType(KeyType::Text(value.clone())),
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
            get_text_inform_type(key.logical_key.clone(), &mut add_key, is_pressed_ctrl);
            if is_pressed_shift {
                get_select_shift_informtype(key.logical_key.clone(), &mut add_key, is_pressed_ctrl);
            } else {
                get_select_informtype(key.logical_key.clone(), &mut add_key, is_pressed_ctrl);
            }
            if let Some(key) = add_key {
                list.push(KeyInform {
                    is_ime: false,
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
    let mut text_list = splite_text(text_field.text.clone(), text_field.select.clone());
    for key_inform in key_list {
        if key_inform.is_finish {
            if key_inform.is_ime {
                if !input.is_last_text_ime {
                    continue;
                }

                if let InformType::KeyType(KeyType::Text(text)) = &key_inform.key {
                    text_list[0].pop();
                    text_list[0] += &text;
                    text_list[1] = "".to_string();
                }
            } else {
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
            }
            input.is_last_text_ime = false;
        } else {
            if let InformType::KeyType(KeyType::Text(text)) = &key_inform.key {
                if input.is_last_text_ime {
                    text_list[0].pop();
                    text_list[0] += text;
                } else {
                    text_list[0] += text;
                }
                input.is_last_text_ime = true;
                text_list[1] = "".to_string();
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

    if let Some(num) = field_info.max_lenght {
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

#[derive(PartialEq, Eq)]
pub(crate) struct KeyInform {
    pub(crate) is_ime: bool,
    pub(crate) is_finish: bool,
    key: InformType,
}

#[derive(PartialEq, Eq)]
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

/*
현재
1.키 가져오기
2.리스트 가져오기
3.자식등 변경

변경
1.키가져오기
2.리스트 가져오기

3.이벤트로 자식 변경

*/
