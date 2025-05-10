use bevy::input::keyboard::Key;
use clipboard_win::{get_clipboard_string, set_clipboard_string};

use crate::text_field::TextField;

use super::input::InformType;

#[derive(PartialEq, Eq,Debug)]
pub(crate) enum KeyType {
    BackSpace,
    Space,
    Copy,
    Paste,
    Cut,
    AllSelect,
    Text(String)
}

pub fn get_text_informtype(key: Key,add_key: &mut Option<InformType>,is_ctrl: bool){
    match key {
        Key::Space => {
            *add_key = Some(InformType::KeyType(KeyType::Space));
        }
        Key::Backspace => {
            *add_key = Some(InformType::KeyType(KeyType::BackSpace));
        }
        Key::Character(msg) => {
            let s_msg = msg.to_string();
            if is_ctrl{
                set_ctrl_key(add_key, s_msg);
            }
            else{
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

fn set_ctrl_key(add_key: &mut Option<InformType>,s_msg: String){
    let msg = s_msg.to_uppercase();
    if msg == "V"{
        *add_key = Some(InformType::KeyType(KeyType::Paste))
    }
    else if msg == "C"{
        *add_key = Some(InformType::KeyType(KeyType::Copy))
    }
    else if msg == "X"{
        *add_key = Some(InformType::KeyType(KeyType::Cut))
    }
    else if msg == "A"{
        *add_key = Some(InformType::KeyType(KeyType::AllSelect))
    }
}

pub fn set_text_list(key: &KeyType,text_list: &mut [String; 3],text_field: &TextField){
    let mut reset_select = true;
    match key {
        KeyType::Text(text) => {
            text_list[0] += &text;
        }
        KeyType::Space => {
            text_list[0] += &" ";
        }
        KeyType::BackSpace => {
            if text_field.select.is_close(){
                text_list[0].pop();
            }
        }
        KeyType::Paste => {
            let paste_text = get_clipboard_string();
            if let Ok(text) = paste_text{
                text_list[0] += &text;
            }
        }
        KeyType::Copy => {
            let _ = set_clipboard_string(text_list[1].as_str());
            reset_select = false;
        }
        KeyType::Cut => {
            let _ = set_clipboard_string(text_list[1].as_str());
        }
        KeyType::AllSelect => {
            text_list[1] = text_list[0].clone() + &text_list[1] + &text_list[2];
            text_list[0].clear();
            text_list[2].clear();
            reset_select = false;
        }
    }
    if reset_select {
        text_list[1] = "".to_string();
    }
}