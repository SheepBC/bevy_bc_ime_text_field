
use bevy::{
    ecs::{entity::Entity, event::EventReader, system::{Commands, Query, Res, ResMut}},
    hierarchy::Children,
    input::{keyboard::{KeyCode, KeyboardInput}, ButtonInput},
    text::TextSpan,
    window::Ime
};

use crate::{
    cursur::TextCursor,
    event::{EnterEvent, TextEdited},
    text_field::{Select, TextField, TextFieldPosition},
    tool::splite_text,
    LastEmoji
};

use super::{
    select_input::{get_select_informtype, get_select_shift_informtype, set_select_text_list, SelectType},
    text_input::{get_text_informtype, set_text_list, KeyType}
};

pub(crate) fn update_input(
    mut commands: Commands,
    last_emoji: ResMut<LastEmoji>,
    evr_ime: EventReader<Ime>,
    evr_kbd: EventReader<KeyboardInput>,
    res_kbd: Res<ButtonInput<KeyCode>>,
    mut q_textfield: Query<(Entity,&mut TextField,&TextCursor,&Children)>,
    mut q_child_text: Query<(&mut TextSpan,&mut TextFieldPosition)>,
) {

    let key_list = get_keys(last_emoji,evr_ime, evr_kbd,res_kbd);

    for (entity,mut text_field,cursor,children) in q_textfield.iter_mut(){
        if !text_field.is_focuse {continue;}

        let mut text_list = [const { String::new() }; 3];
        let mut is_text_change = false;
        let is_enter = key_list.contains(&KeyInform { is_ime: false, is_finish: true, key: InformType::KeyType(KeyType::Text("\n".to_string())) });
        if !key_list.is_empty(){
            text_list = get_changed_text_list(&key_list, &mut text_field);
            is_text_change = true;
        }

        update_text_field(children, &mut q_child_text, is_text_change, text_list, cursor);

        if is_text_change{
            commands.get_entity(entity).unwrap().trigger(TextEdited {
                text_field: text_field.clone(),
                entity: entity
            });
        }

        if is_enter{
            commands.get_entity(entity).unwrap().trigger(EnterEvent{
                text_field: text_field.clone(),
                entity: entity
            });
        }

    }

}

pub(crate) fn update_text_field(
    children: &Children,
    q_child_text: &mut Query<(&mut TextSpan,&mut TextFieldPosition)>,
    is_update: bool,
    text_list: [String; 3],
    cursor: &TextCursor
) {
    for child in children.iter(){

        let (mut span,mut position) = q_child_text.get_mut(*child).unwrap();

        if is_update {
            match *position {
                TextFieldPosition::Front => {
                    **span = text_list[0].clone();
                }
                TextFieldPosition::Select(_) => {
                    **span = text_list[1].clone();
                    if span.0.chars().count() != 0{
                        *position = TextFieldPosition::Select(span.0.clone());
                    }
                    else {
                        *position = TextFieldPosition::Select(String::new());
                    }
                }
                TextFieldPosition::Back => {
                    **span = text_list[2].clone();
                }
            }

        }

        if let TextFieldPosition::Select(select) = position.into_inner() {
            if select.is_empty(){
                if cursor.is_see {
                    **span = "|".to_string();
                }
                else {
                    **span = String::new();
                }
            }   
        }
    }

}

pub(crate) fn get_keys(
    mut last_emoji: ResMut<LastEmoji>,
    mut evr_ime: EventReader<Ime>,
    mut evr_kbd: EventReader<KeyboardInput>,
    res_kbd: Res<ButtonInput<KeyCode>>
) -> Vec<KeyInform>{

    let mut list: Vec<KeyInform> = Vec::new();

    for ime in evr_ime.read(){
        //println!("{:?}",ime);

        match ime {
            Ime::Commit { value,.. } => {
                if value == &"".to_string() {continue;}
                if value.chars().count() > 1 {
                    last_emoji.0 = None;
                    continue;
                }
                list.push(KeyInform {
                    is_ime: true,
                    is_finish:true,
                    key: InformType::KeyType(KeyType::Text(value.clone()))
                });
            }
            Ime::Preedit {  value, cursor,.. } => {
                if value == &"".to_string() {continue;}
                if cursor != &Some((0,0)) {
                    let mut text = value.clone();

                    if let Some(last) = &last_emoji.0 {
                        text = value.replacen(last, "", 1);
                    }
                    last_emoji.0 = Some(value.to_string());
                    list.push(KeyInform {
                        is_ime: false,
                        is_finish:true,
                        key: InformType::KeyType(KeyType::Text(text.clone()))
                    });
                    continue;
                }
                
                list.push(KeyInform {
                    is_ime: true,
                    is_finish:false,
                    key: InformType::KeyType(KeyType::Text(value.clone()))
                });
            }
            _ => {}
        }
    }
    let key_list = evr_kbd.read();
    let is_pressed_shift = res_kbd.any_pressed([KeyCode::ShiftLeft,KeyCode::ShiftRight]);
    let is_pressed_ctrl = res_kbd.any_pressed([KeyCode::ControlLeft,KeyCode::ControlRight]);
    for key in key_list{
        if key.state.is_pressed(){
            let mut add_key:Option<InformType> = None;
            get_text_informtype(key.logical_key.clone(), &mut add_key,is_pressed_ctrl);
            if is_pressed_shift{
                get_select_shift_informtype(key.logical_key.clone(), &mut add_key,is_pressed_ctrl);
            }
            else {
                get_select_informtype(key.logical_key.clone(), &mut add_key,is_pressed_ctrl);   
            }
            if let Some(key) = add_key{
                list.push(KeyInform { 
                    is_ime: false,
                    is_finish: true,
                    key: key
                });
            }
        }
    }

    list
}

pub(crate) fn get_changed_text_list(
    key_list: &Vec<KeyInform>,
    text_field: &mut TextField,
) -> [String; 3] {

    let mut text_list = splite_text(text_field.text.clone(), text_field.select.clone());

    for key_inform in key_list{

        if key_inform.is_finish{
            if key_inform.is_ime{
                if !text_field.is_before_text_ime {continue;}

                if let InformType::KeyType(KeyType::Text(text)) = &key_inform.key{
                    text_list[0].pop();
                    text_list[0] += &text;
                    text_list[1] = "".to_string();
                }
            }
            else {
                
                match &key_inform.key {
                    InformType::KeyType(key) => {
                        set_text_list(key, &mut text_list);
                    }
                    InformType::SelectType(key) => {
                        if set_select_text_list(key,&mut text_list,text_field){
                            break;
                        }
                    }
                }

            }
            text_field.is_before_text_ime= false;
        }
        else {
            if let InformType::KeyType(KeyType::Text(text)) = &key_inform.key{

                if text_field.is_before_text_ime{
                    text_list[0].pop();
                    text_list[0] += text;
                }
                else {
                    text_list[0] += text;
                }
                text_field.is_before_text_ime= true;
                text_list[1] = "".to_string();
            }
        }
    }

    text_field.text = text_list.concat();
    let num = text_list[0].chars().count();
    let select_num = text_list[1].chars().count();
    let last_select = if select_num == 0{
        None
    }else {
        text_field.select.2
    };
    text_field.select = Select(num,num+select_num,last_select);

    text_list
    
}

#[derive(PartialEq, Eq)]
pub(crate) struct KeyInform{
    pub(crate) is_ime: bool,
    pub(crate) is_finish: bool,
    key: InformType,
}

#[derive(PartialEq, Eq)]
pub(crate) enum InformType{
    KeyType(KeyType),
    SelectType(SelectType)
}