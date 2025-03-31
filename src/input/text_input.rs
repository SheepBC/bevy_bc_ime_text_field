use bevy::input::keyboard::Key;

use super::input::InformType;

#[derive(PartialEq, Eq,Debug)]
pub(crate) enum KeyType {
    BackSpace,
    Space,
    Text(String)
}

pub fn get_text_informtype(key: Key,add_key: &mut Option<InformType>){
    match key {
        Key::Space => {
            *add_key = Some(InformType::KeyType(KeyType::Space));
        }
        Key::Backspace => {
            *add_key = Some(InformType::KeyType(KeyType::BackSpace));
        }
        Key::Character(msg) => {
            *add_key = Some(InformType::KeyType(KeyType::Text(msg.to_string())));
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

pub fn set_text_list(key: &KeyType,text_list: &mut [String; 3]){
    match key {
        KeyType::Text(text) => {
            text_list[0] += &text;
        }
        KeyType::Space => {
            text_list[0] += &" ";
        }
        KeyType::BackSpace => {
            text_list[0].pop();
        }
    }
    text_list[1] = "".to_string();
}