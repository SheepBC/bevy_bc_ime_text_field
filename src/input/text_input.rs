use arboard::Clipboard;
use bevy::input::keyboard::Key;
use crate::input::control::get_front_ctrl;
use crate::text_field::TextField;

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

pub fn get_text_inform_type(key: Key, add_key: &mut Option<InformType>, is_ctrl: bool) {
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
                set_ctrl_key(add_key, s_msg);
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

fn set_ctrl_key(add_key: &mut Option<InformType>, s_msg: String) {
    let msg = s_msg.to_uppercase();
    if msg == "V" {
        *add_key = Some(InformType::KeyType(KeyType::Paste))
    } else if msg == "C" {
        *add_key = Some(InformType::KeyType(KeyType::Copy))
    } else if msg == "X" {
        *add_key = Some(InformType::KeyType(KeyType::Cut))
    } else if msg == "A" {
        *add_key = Some(InformType::KeyType(KeyType::AllSelect))
    }
}

pub fn set_text_list(key: &KeyType, text_list: &mut [String; 3], text_field: &TextField) {
    let mut reset_select = true;
    match key {
        KeyType::Text(text) => {
            text_list[0] += &text;
        }
        KeyType::Space => {
            text_list[0] += &" ";
        }
        KeyType::BackSpace => {
            if text_field.select.is_close() {
                text_list[0].pop();
            }
        }
        KeyType::CtrlBackSpace => {
            let list = get_front_ctrl(text_list[0].clone() + &text_list[1]);
            text_list[0] = list[0].clone();
        }
        KeyType::Paste => {
            if let Ok(mut clip) = Clipboard::new() {
                if let Ok(text) = clip.get_text(){
                    text_list[0] += &text;
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
            }
        }
        KeyType::AllSelect => {
            text_list[1] = text_list[0].clone() + &text_list[1] + &text_list[2];
            text_list[0].clear();
            text_list[2].clear();
            reset_select = false;
        }
        KeyType::Undo => {

        }
        KeyType::Redo => {

        }
    }
    if reset_select {
        text_list[1] = "".to_string();
    }
}
