use arboard::Clipboard;
use bevy::input::keyboard::Key;
use crate::input::control::get_front_ctrl;
use crate::text_field::{Change, Select, TextField};
use crate::tool::split_text;
use super::input::InformType;

#[derive(PartialEq, Eq, Debug)]
pub(crate) enum KeyType {
    BackSpace,
    CtrlBackSpace,
    Space,
    Copy,
    Paste,
    Cut,
    AllSelect,
    Undo,
    Redo,
    Text(String),
}

pub fn get_text_inform_type(key: Key, add_key: &mut Option<InformType>, is_ctrl: bool, is_shift: bool) {
    match key {
        Key::Space => {
            *add_key = Some(InformType::KeyType(KeyType::Space));
        }
        Key::Backspace => {
            if is_ctrl {
                *add_key = Some(InformType::KeyType(KeyType::CtrlBackSpace));
            } else {
                *add_key = Some(InformType::KeyType(KeyType::BackSpace));
            }
        }
        Key::Character(msg) => {
            let s_msg = msg.to_string();
            if is_ctrl {
                set_ctrl_key(add_key, s_msg, is_shift);
            } else {
                *add_key = Some(InformType::KeyType(KeyType::Text(s_msg)));
            }
        }
        Key::Enter => {
            *add_key = Some(InformType::KeyType(KeyType::Text("\n".to_string())));
        }
        Key::Tab => {
            *add_key = Some(InformType::KeyType(KeyType::Text("\t".to_string())));
        }
        _ => {}
    }
}

fn set_ctrl_key(add_key: &mut Option<InformType>, s_msg: String, is_shift: bool) {
    let msg = s_msg.to_uppercase();
    if msg == "V" {
        *add_key = Some(InformType::KeyType(KeyType::Paste))
    } else if msg == "C" {
        *add_key = Some(InformType::KeyType(KeyType::Copy))
    } else if msg == "X" {
        *add_key = Some(InformType::KeyType(KeyType::Cut))
    } else if msg == "A" {
        *add_key = Some(InformType::KeyType(KeyType::AllSelect))
    }else if msg == "Y" || (msg == "Z" && is_shift){
        *add_key = Some(InformType::KeyType(KeyType::Redo))
    }else if msg == "Z" {
        *add_key = Some(InformType::KeyType(KeyType::Undo))
    }
}

pub fn set_text_list(key: &KeyType, text_list: &mut [String; 3], text_field: &mut TextField) {
    let mut reset_select = true;
    match key {
        KeyType::Text(text) => {
            text_list[0] += &text;
            text_field.command.push(Change{select: text_field.select,before:text_list[1].clone(),after:text.clone()});
        }
        KeyType::Space => {
            text_list[0] += &" ";
            text_field.command.push(Change{select: text_field.select,before:text_list[1].clone(),after:" ".to_string()});
        }
        KeyType::BackSpace => {
            let mut pop = String::new();
            if text_field.select.is_close() {
                pop = if let Some(c) = text_list[0].pop(){
                    c.to_string()
                }else { String::new() };
            }
            else{pop = text_list[1].clone();}
            text_field.command.push(Change{select: text_field.select,before:pop,after:"".to_string()});
        }
        KeyType::CtrlBackSpace => {
            let list = get_front_ctrl(text_list[0].clone() + &text_list[1]);
            text_list[0] = list[0].clone();
            text_field.command.push(Change{select: text_field.select,before:text_list[1].clone(),after:"".to_string()});
        }
        KeyType::Paste => {
            if let Ok(mut clip) = Clipboard::new() {
                if let Ok(text) = clip.get_text(){
                    text_list[0] += &text;
                    text_field.command.push(Change{select: text_field.select,before:text_list[1].clone(),after:text.clone()});
                }
            }
        }
        KeyType::Copy => {
            if let Ok(mut clip) = Clipboard::new() {
                let _ =clip.set_text(text_list[1].clone());
                reset_select = false;
            }
        }
        KeyType::Cut => {
            if let Ok(mut clip) = Clipboard::new() {
                let _ =clip.set_text(text_list[1].clone());
                text_field.command.push(Change{select: text_field.select,before:text_list[1].clone(),after:"".to_string()});
            }
        }
        KeyType::AllSelect => {
            text_list[1] = text_list[0].clone() + &text_list[1] + &text_list[2];
            text_list[0].clear();
            text_list[2].clear();
            reset_select = false;
        }
        KeyType::Undo => {
            for(_,command) in text_field.command.undo(){
                println!("Undo {:?}",command);
                let new_list = split_text(text_field.text.clone(),Select(command.select.0,command.select.0,command.select.2));
                println!("Undo List: {:?}",new_list);
            }
        }
        KeyType::Redo => {
            for(_,command) in text_field.command.redo(){
                println!("Redo {:?}",command);
            }
        }
    }
    if reset_select {
        text_list[1] = "".to_string();
    }
}
