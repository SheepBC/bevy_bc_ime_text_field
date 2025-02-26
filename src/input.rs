use std::cmp::min;

use bevy::{
    ecs::{entity::Entity, event::EventReader, system::{Commands, Query, ResMut}},
    hierarchy::Children,
    input::keyboard::{Key, KeyboardInput},
    text::TextSpan,
    window::Ime
};

use crate::{cursur::TextCursor, event::TextEdited, text_field::{LastEmoji, Select, TextField, TextFieldPosition}, tool::{splite_text, ToolString}};


pub(crate) fn get_keys(
    mut last_emoji: ResMut<LastEmoji>,
    mut evr_ime: EventReader<Ime>,
    mut evr_kbd: EventReader<KeyboardInput>,
) -> Vec<KeyInform>{
    let mut list: Vec<KeyInform>  = Vec::new();

    for ime in evr_ime.read(){
        //println!("{:?}",ime);

        match ime {
            Ime::Commit { value,.. } => {
                if value == &"".to_string() {break;}
                if value.chars().count() > 1 {
                    last_emoji.0 = None;
                    break;
                }
                list.push(KeyInform {is_ime: true,is_finish:true,key: KeyType::Text(value.clone())});
            }
            Ime::Preedit {  value, cursor,.. } => {
                if value == &"".to_string() {break;}
                if cursor != &Some((0,0)) {
                    let mut text = value.clone();

                    if let Some(last) = &last_emoji.0 {
                        text = value.replacen(last, "", 1);
                    }
                    last_emoji.0 = Some(value.to_string());
                    list.push(KeyInform {is_ime: false,is_finish:true,key: KeyType::Text(text.clone())});
                    break;
                }
                
                list.push(KeyInform {is_ime: true,is_finish:false,key: KeyType::Text(value.clone())});
            }
            _ => {}
        }
    }

    for key in evr_kbd.read(){
        if key.state.is_pressed(){
            match &key.logical_key {
                Key::Space => {
                    list.push(KeyInform {is_ime: false,is_finish:true,key: KeyType::Space});
                }
                Key::Backspace => {
                    list.push(KeyInform {is_ime: false,is_finish:true,key: KeyType::BackSpace});
                }
                Key::Character(msg) => {
                    list.push(KeyInform {is_ime: false,is_finish:true,key: KeyType::Text(msg.to_string())});
                }
                Key::Enter => {
                    list.push(KeyInform {is_ime: false,is_finish:true,key: KeyType::Text("\n".to_string())});
                }
                Key::ArrowLeft => {
                    list.push(KeyInform {is_ime: false,is_finish:true,key: KeyType::Left});
                }
                Key::ArrowRight => {
                    list.push(KeyInform {is_ime: false,is_finish:true,key: KeyType::Right});
                }
                Key::ArrowUp => {
                    list.push(KeyInform {is_ime: false,is_finish:true,key: KeyType::Up});
                }
                Key::ArrowDown => {
                    list.push(KeyInform {is_ime: false,is_finish:true,key: KeyType::Down});
                }
                _ => {}
            }
        }
    }

    list

}

pub(crate) fn input_update(
    mut commands: Commands,
    last_emoji: ResMut<LastEmoji>,
    evr_ime: EventReader<Ime>,
    evr_kbd: EventReader<KeyboardInput>,
    mut q_textfield: Query<(Entity,&mut TextField,&TextCursor,&Children)>,
    mut q_child_text: Query<(&mut TextSpan,&mut TextFieldPosition)>,
) {

    let key_list = get_keys(last_emoji,evr_ime, evr_kbd);

    for (entity,mut text_field,cursor,children) in q_textfield.iter_mut(){
        if !text_field.is_focuse {continue;}

        let mut text_list = [const { String::new() }; 3];
        let mut is_text_change = false;
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

pub(crate) fn get_changed_text_list(
    key_list: &Vec<KeyInform>,
    text_field: &mut TextField,
) -> [String; 3] {

    let mut text_list = splite_text(text_field.text.clone(), text_field.select.clone());

    for key_inform in key_list{

        if key_inform.is_finish{
            if key_inform.is_ime{
                if !text_field.is_before_text_ime {continue;}

                if let KeyType::Text(text) = &key_inform.key{
                    text_list[0].pop();
                    text_list[0] += text;
                }
            }
            else {
                
                match &key_inform.key {
                    KeyType::Text(text) => {
                        text_list[0] += text;
                    }
                    KeyType::Space => {
                        text_list[0] += &" ";
                    }
                    KeyType::BackSpace => {
                        text_list[0].pop();
                    }
                    KeyType::Left => {
                        if text_list[0].is_empty() {break;}
                        let remove = text_list[0].pop();
                        text_list[2] = remove.unwrap().to_string() + &text_list[2];
                    }
                    KeyType::Right => {
                        if text_list[2].is_empty() {break;}
                        let remove = text_list[2].front_pop();
                        text_list[0] = text_list[0].clone() + &remove.unwrap().to_string();
                    }
                    KeyType::Up => {
                        let text = text_list.concat();
                        let list: Vec<&str> = text.split("\n").collect();
                        let mut last_line_inform: (usize,usize) = (0,0);
                        let mut all_size = 0;
                        let select = text_field.select.0;

                        for line in list{
                            let size = line.to_string().size();
                            all_size += size;
                            if select <= all_size{
                                if last_line_inform == (0,0){
                                    break;
                                }
                                else {
                                    let change_select_usize = min(last_line_inform.0 + (select - (all_size-size)), last_line_inform.1);
                                    let split_front = text_list[0].split_chars_at(change_select_usize);
                                    text_list[0] = split_front.0;
                                    text_list[2] = split_front.1 + &text_list[2];
                                    break;
                                }
                            }

                            last_line_inform = (all_size-size,all_size);
                            all_size += 1;
                        }

                    }
                    KeyType::Down => {
                        let text = text_list.concat();
                        let list: Vec<&str> = text.split("\n").collect();
                        let mut is_now: (bool,usize) = (false,0);
                        let mut all_size = 0;
                        let select = text_field.select.0;

                        for line in list{
                            let size = line.to_string().size();
                            all_size += size;

                            if is_now.0{
                                let change_select_usize = min((all_size-size)+is_now.1, all_size);
                                let split_back = text_list[2].split_chars_at(change_select_usize-text_list[0].size());
                                text_list[0] = text_list[0].clone() + &split_back.0;
                                text_list[2] = split_back.1;
                                break; 
                            }

                            if select <= all_size{
                                is_now = (true,select-(all_size-size));
                            }

                            all_size += 1;
                        }

                    }
                }

            }
            text_field.is_before_text_ime= false;
        }
        else {
            if let KeyType::Text(text) = &key_inform.key{

                if text_field.is_before_text_ime{
                    text_list[0].pop();
                    text_list[0] += text;
                }
                else {
                    text_list[0] += text;
                }
                text_field.is_before_text_ime= true;
            }
        }
    }

    text_field.text = text_list.concat();
    let num = text_list[0].chars().count();
    text_field.select = Select(num,num);

    text_list
    
}

#[derive(Debug)]
pub(crate) struct KeyInform{
    pub(crate) is_ime: bool,
    pub(crate) is_finish: bool,
    pub(crate) key: KeyType,
}

#[derive(PartialEq, Eq,Debug)]
pub(crate) enum KeyType {
    BackSpace,
    Space,
    Up,
    Left,
    Right,
    Down,
    Text(String)
}