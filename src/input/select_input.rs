use std::cmp::min;

use bevy::input::keyboard::Key;

use crate::{text_field::TextField, tool::ToolString};

use super::input::{InformType, SelectType};



pub fn get_select_informtype(key: Key,add_key: &mut Option<InformType>){
    match key {
        Key::ArrowLeft => {
            *add_key = Some(InformType::SelectType(SelectType::Left));
        }
        Key::ArrowRight => {
            *add_key = Some(InformType::SelectType(SelectType::Right));
        }
        Key::ArrowUp => {
            *add_key = Some(InformType::SelectType(SelectType::Up));
        }
        Key::ArrowDown => {
            *add_key = Some(InformType::SelectType(SelectType::Down));
        }
        _ => {}
    }
}

pub fn set_select_text_list(key: &SelectType,text_list: &mut [String; 3],text_field: &mut TextField) -> bool{
    match *key {
        SelectType::Left => {
            if text_list[0].is_empty() {
                return true;
            }
            let remove = text_list[0].pop();
            text_list[2] = remove.unwrap().to_string() + &text_list[2];
        }
        SelectType::Right => {
            if text_list[2].is_empty() {
                return true;
            }
            let remove = text_list[2].front_pop();
            text_list[0] = text_list[0].clone() + &remove.unwrap().to_string();
        }
        SelectType::Up => {
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
        SelectType::Down => {
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

    false
}