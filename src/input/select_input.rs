use std::cmp::min;

use bevy::input::keyboard::Key;

use crate::{text_field::TextField, tool::ToolString};

use super::input::InformType;

#[derive(Debug,PartialEq,Eq)]
pub(crate) enum SelectType {
    Up,
    Left,
    Right,
    Down,
    Extend(Direction)
}

#[derive(Debug,PartialEq, Eq)]
pub(crate) enum Direction {
    Up,
    Left,
    Right,
    Down
}

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

pub fn get_select_shift_informtype(key: Key,add_key: &mut Option<InformType>){
    match key {
        Key::ArrowLeft => {
            *add_key = Some(InformType::SelectType(SelectType::Extend(Direction::Left)));
        }
        Key::ArrowRight => {
            *add_key = Some(InformType::SelectType(SelectType::Extend(Direction::Right)));
        }
        Key::ArrowUp => {
            *add_key = Some(InformType::SelectType(SelectType::Extend(Direction::Up)));
        }
        Key::ArrowDown => {
            *add_key = Some(InformType::SelectType(SelectType::Extend(Direction::Down)));
        }
        _ => {}
    }
}

pub fn set_select_text_list(key: &SelectType,text_list: &mut [String; 3],text_field: &mut TextField) -> bool{
    let is_close = text_field.select.is_close();
    if is_close{
        return set_text_list_in_close(key, text_list, text_field);
    }
    else {
        return set_text_list_in_open(key, text_list, text_field);
    }
}

fn set_text_list_in_close(key: &SelectType,text_list: &mut [String; 3],text_field: &mut TextField) -> bool{

    match key {
        SelectType::Left => {
            if text_list[0].is_empty() {
                return true;
            }
            let remove = text_list[0].pop();
            text_list[2] = remove.unwrap().to_string() +&text_list[1] + &text_list[2];
            text_list[1] = "".to_string();
        }
        SelectType::Right => {
            if text_list[2].is_empty() {
                return true;
            }
            let remove = text_list[2].front_pop();
            text_list[0] = text_list[0].clone() + &text_list[1] + &remove.unwrap().to_string();
            text_list[1] = "".to_string();
        }
        SelectType::Up => {
            let change_select_usize = get_up_usize(text_list, text_field.select.0);
            let split_front = text_list[0].split_chars_at(change_select_usize);
            text_list[0] = split_front.0;
            text_list[2] = split_front.1+&text_list[1] + &text_list[2];
            text_list[1] = "".to_string();
        }
        SelectType::Down => {
            let change_select_usize = get_down_usize(text_list, text_field.select.1);
            let split_back = text_list[2].split_chars_at(change_select_usize-text_list[0].size());
            text_list[0] = text_list[0].clone() + &split_back.0;
            text_list[2] = split_back.1;
        }
        SelectType::Extend(direction) => {
            match direction {
                Direction::Left => {
                    if text_list[0].is_empty() {
                        return true;
                    }
                    text_field.select.2 = Some(text_field.select.0);
                    let remove = text_list[0].pop();
                    text_list[1] = remove.unwrap().to_string() + &text_list[1];
                }
                Direction::Right => {
                    if text_list[2].is_empty() {
                        return true;
                    }
                    text_field.select.2 = Some(text_field.select.0);
                    let remove = text_list[2].front_pop();
                    text_list[1] = text_list[1].clone() + &remove.unwrap().to_string();
                }
                Direction::Up => {
                    text_field.select.2 = Some(text_field.select.0);
                    let change_select_usize = get_up_usize(text_list, text_field.select.0);
                    let split_front = text_list[0].split_chars_at(change_select_usize);
                    text_list[0] = split_front.0;
                    text_list[1] = split_front.1 + &text_list[1];
                }
                Direction::Down => {
                    text_field.select.2 = Some(text_field.select.0);
                    let change_select_usize = get_down_usize(text_list, text_field.select.1);
                    let split_back = text_list[2].split_chars_at(change_select_usize-text_list[0].size());
                    text_list[1] = text_list[1].clone() + &split_back.0;
                    text_list[2] = split_back.1;
                }
                
            }
        }
    }

    false
}

fn set_text_list_in_open(key: &SelectType,text_list: &mut [String; 3],text_field: &mut TextField) -> bool{
    match key {
        SelectType::Left => {
            text_list[2] = text_list[1].clone() + &text_list[2];
            text_list[1] = "".to_string();
        }
        SelectType::Right => {
            text_list[0] = text_list[0].clone() + &text_list[1];
            text_list[1] = "".to_string();
        }
        SelectType::Up => {
            let change_select_usize = get_up_usize(text_list, text_field.select.0);
            let split_front = text_list[0].split_chars_at(change_select_usize);
            text_list[0] = split_front.0;
            text_list[2] = split_front.1+&text_list[1] + &text_list[2];
            text_list[1] = "".to_string();
        }
        SelectType::Down => {
            let change_select_usize = get_down_usize(text_list, text_field.select.1);
            let split_back = text_list[2].split_chars_at(change_select_usize-text_list[0].size());
            text_list[0] = text_list[0].clone() + &text_list[1]  + &split_back.0;
            text_list[2] = split_back.1;
            text_list[1] = "".to_string()
        }
        SelectType::Extend(direction) => {
            match direction {
                Direction::Left => {
                    if text_field.select.is_open_left(){
                        if text_list[0].is_empty() {
                            return true;
                        }
                        let remove = text_list[0].pop();
                        text_list[1] = remove.unwrap().to_string() + &text_list[1];
                    }
                    else{
                        let remove = text_list[1].pop();
                        text_list[2] = remove.unwrap().to_string() + &text_list[2];
                    }
                }
                Direction::Right => {
                    if text_field.select.is_open_right(){
                        if text_list[2].is_empty() {
                            return true;
                        }
                        let remove = text_list[2].front_pop();
                        text_list[1] = text_list[1].clone() + &remove.unwrap().to_string();
                    }
                    else{
                        let remove = text_list[1].front_pop();
                        text_list[0] = text_list[0].clone()+&remove.unwrap().to_string();
                    }
                }
                Direction::Up => {
                    if text_field.select.is_open_left(){
                        let change_select_usize = get_up_usize(text_list, text_field.select.0);
                        let split_front = text_list[0].split_chars_at(change_select_usize);
                        text_list[0] = split_front.0;
                        text_list[1] = split_front.1 + &text_list[1];
                    }
                    else {
                        let change_select_usize = get_up_usize(text_list, text_field.select.1);
                        let split_front = text_list[1].split_chars_at(change_select_usize-text_list[0].size());
                        text_list[1] = split_front.0;
                        text_list[2] = split_front.1 + &text_list[2];
                    }
                }
                Direction::Down => {
                    if text_field.select.is_open_right(){
                        let change_select_usize = get_down_usize(text_list, text_field.select.1);
                        let split_back = text_list[2].split_chars_at(change_select_usize-text_list[0].size()-text_list[1].size());
                        text_list[1] = text_list[1].clone() + &split_back.0;
                        text_list[2] = split_back.1;
                    }
                    else {
                        let change_select_usize = get_down_usize(text_list, text_field.select.0);
                        let split_back = text_list[1].split_chars_at(change_select_usize-text_list[0].size());
                        text_list[1] = split_back.1;
                        text_list[0] =text_list[0].clone() + &split_back.0;
                    }
                }
            }
        }
    }
    false
}

fn get_up_usize(text_list: &mut [String; 3],select: usize) -> usize{
    let text = text_list.concat();
    let list: Vec<&str> = text.split("\n").collect();
    let mut last_line_inform: (usize,usize) = (0,0);
    let mut all_size = 0;
    for line in list{
        let size = line.to_string().size();
        all_size += size;
        if select <= all_size{
            if last_line_inform == (0,0){
                return select;
            }
            else {
                let change_select_usize = min(last_line_inform.0 + (select - (all_size-size)), last_line_inform.1);
                return change_select_usize;
            }
        }
        last_line_inform = (all_size-size,all_size);
        all_size += 1;
    }
    select
}

fn get_down_usize(text_list: &mut [String; 3],select: usize) -> usize{
    let text = text_list.concat();
    let list: Vec<&str> = text.split("\n").collect();
    let mut is_now: (bool,usize) = (false,0);
    let mut all_size = 0;

    for line in list{
        let size = line.to_string().size();
        all_size += size;

        if is_now.0{
            let change_select_usize = min((all_size-size)+is_now.1, all_size);
            return change_select_usize;
        }

        if select <= all_size{
            is_now = (true,select-(all_size-size));
        }

        all_size += 1;
    }
    text.size()
}