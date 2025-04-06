use bevy::input::keyboard::Key;

use crate::{
    text_field::TextField,
    tool::ToolString
};

use super::{
    control::{get_back_ctrl, get_down_ctrl, get_front_ctrl, get_up_ctrl},
    input::InformType,
    select_list::{
        get_down_usize,
        get_up_usize,
        set_down_extend,
        set_left_area,
        set_left_area_ctrl,
        set_left_extend,
        set_right_area,
        set_right_area_ctrl,
        set_right_extend,
        set_up_extend
    }
};

#[derive(Debug,PartialEq,Eq)]
pub(crate) enum SelectType {
    Up(bool),
    Left(bool),
    Right(bool),
    Down(bool),
    Extend(Direction,bool)
}

#[derive(Debug,PartialEq, Eq)]
pub(crate) enum Direction {
    Up,
    Left,
    Right,
    Down
}

pub fn get_select_informtype(key: Key,add_key: &mut Option<InformType>,is_pressed_crtrl: bool){
    match key {
        Key::ArrowLeft => {
            *add_key = Some(InformType::SelectType(SelectType::Left(is_pressed_crtrl)));
        }
        Key::ArrowRight => {
            *add_key = Some(InformType::SelectType(SelectType::Right(is_pressed_crtrl)));
        }
        Key::ArrowUp => {
            *add_key = Some(InformType::SelectType(SelectType::Up(is_pressed_crtrl)));
        }
        Key::ArrowDown => {
            *add_key = Some(InformType::SelectType(SelectType::Down(is_pressed_crtrl)));
        }
        _ => {}
    }
}

pub fn get_select_shift_informtype(key: Key,add_key: &mut Option<InformType>,is_pressed_ctrl: bool){
    match key {
        Key::ArrowLeft => {
            *add_key = Some(InformType::SelectType(SelectType::Extend(Direction::Left,is_pressed_ctrl)));
        }
        Key::ArrowRight => {
            *add_key = Some(InformType::SelectType(SelectType::Extend(Direction::Right,is_pressed_ctrl)));
        }
        Key::ArrowUp => {
            *add_key = Some(InformType::SelectType(SelectType::Extend(Direction::Up,is_pressed_ctrl)));
        }
        Key::ArrowDown => {
            *add_key = Some(InformType::SelectType(SelectType::Extend(Direction::Down,is_pressed_ctrl)));
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
    let mut is_extend = false;
    match key {
        SelectType::Left(is_ctrl) => {
            if text_list[0].is_empty() {
                return true;
            }
            if *is_ctrl{
                let list = get_front_ctrl(text_list[0].clone());
                set_left_area_ctrl(text_list,list);
            }
            else {
                let remove = text_list[0].pop();
                set_left_area(text_list, remove.unwrap().to_string());
            }
            
        }
        SelectType::Right(is_ctrl) => {
            if text_list[2].is_empty() {
                return true;
            }
            if *is_ctrl{
                let list = get_back_ctrl(text_list[2].clone());
                set_right_area_ctrl(text_list,list);
            }
            else {
                let remove = text_list[2].front_pop();
                set_right_area(text_list, remove.unwrap().to_string());
            }
        }
        SelectType::Up(is_ctrl) => {
            if *is_ctrl{
                if text_list[0] == ""{
                    return true;
                }
                let list = get_up_ctrl(text_list[0].clone());
                set_left_area_ctrl(text_list, list);
            }
            else {
                let change_select_usize = get_up_usize(text_list, text_field.select.0);
                let split_front = text_list[0].split_chars_at(change_select_usize);
                text_list[0] = split_front[0].clone();
                set_left_area(text_list, split_front[1].clone());
            }
        }
        SelectType::Down(is_ctrl) => {
            if *is_ctrl{
                if text_list[2] == ""{
                    return true;
                }
                let list = get_down_ctrl(text_list[2].clone());
                set_right_area_ctrl(text_list, list);
            }
            else {
                let change_select_usize = get_down_usize(text_list, text_field.select.1);
                let split_back = text_list[2].split_chars_at(change_select_usize-text_list[0].size());
                text_list[2] = split_back[1].clone();
                set_right_area(text_list, split_back[0].clone());
            }
        }
        SelectType::Extend(direction,is_ctrl) => {
            is_extend = true;
            match direction {
                Direction::Left => {
                    if text_list[0].is_empty() {
                        return true;
                    }
                    set_left_extend(text_list, is_ctrl);
                }
                Direction::Right => {
                    if text_list[2].is_empty() {
                        return true;
                    }
                    set_right_extend(text_list, is_ctrl);
                }
                Direction::Up => {
                    set_up_extend(text_list, text_field,is_ctrl);
                }
                Direction::Down => {
                    set_down_extend(text_list, text_field, is_ctrl);
                }
                
            }
        }
    }
    if is_extend{
        text_field.select.2 = Some(text_field.select.0);
    }
    else {
        text_list[1] = "".to_string();
    }

    false
}

fn set_text_list_in_open(key: &SelectType,text_list: &mut [String; 3],text_field: &mut TextField) -> bool{
    let mut is_extend = false;
    match key {
        SelectType::Left(is_ctrl) => {
            if *is_ctrl{
                let list = get_front_ctrl(text_list[0].clone());
                set_left_area_ctrl(text_list,list);
            }
            else {
                set_left_area(text_list, String::new());  
            }
        }
        SelectType::Right(is_ctrl) => {
            if *is_ctrl{
                let list = get_back_ctrl(text_list[2].clone());
                set_right_area_ctrl(text_list,list);
            }
            else{
                set_right_area(text_list,String::new());
            }
        }
        SelectType::Up(is_ctrl) => {
            if *is_ctrl{
                if text_list[0] == ""{
                    return true;
                }
                let list = get_up_ctrl(text_list[0].clone());
                set_left_area_ctrl(text_list, list);
            }
            else{
                let change_select_usize = get_up_usize(text_list, text_field.select.0);
                let split_front = text_list[0].split_chars_at(change_select_usize);
                text_list[0] = split_front[0].clone();
                set_left_area(text_list, split_front[1].clone());
            }
        }
        SelectType::Down(is_ctrl) => {
            if *is_ctrl{
                if text_list[2] == ""{
                    return true;
                }
                let list = get_down_ctrl(text_list[2].clone());
                set_right_area_ctrl(text_list, list);
            }
            else {
                let change_select_usize = get_down_usize(text_list, text_field.select.1);
                let split_back = text_list[2].split_chars_at(change_select_usize-text_list[0].size());
                text_list[2] = split_back[1].clone();
                set_right_area(text_list, split_back[0].clone());
            }
        }
        SelectType::Extend(direction,is_ctrl) => {
            is_extend = true;
            match direction {
                Direction::Left => {
                    if text_field.select.is_open_left(){
                        if text_list[0].is_empty() {
                            return true;
                        }
                        set_left_extend(text_list, is_ctrl);
                    }
                    else{
                        if *is_ctrl{
                            let list = get_front_ctrl(text_list[1].clone());
                            text_list[1] = list[0].clone();
                            text_list[2] = list[1].clone() + &text_list[2];
                        }
                        else{
                            let remove = text_list[1].pop();
                            text_list[2] = remove.unwrap().to_string() + &text_list[2];
                        }
                    }
                }
                Direction::Right => {
                    if text_field.select.is_open_right(){
                        if text_list[2].is_empty() {
                            return true;
                        }
                        set_right_extend(text_list, is_ctrl);
                    }
                    else{
                        if *is_ctrl{
                            let list = get_back_ctrl(text_list[1].clone());
                            text_list[1] = list[1].clone();
                            text_list[0] = text_list[0].clone() + &list[0];
                        }
                        else{
                            let remove = text_list[1].front_pop();
                            text_list[0] = text_list[0].clone()+&remove.unwrap().to_string();
                        }
                    }
                }
                Direction::Up => {
                    if text_field.select.is_open_left(){
                        set_up_extend(text_list, text_field, is_ctrl);
                    }
                    else {
                        let list = if *is_ctrl{
                            get_up_ctrl(text_list[1].clone())
                        }
                        else {
                            let change_select_usize = get_up_usize(text_list, text_field.select.1);
                            text_list[1].split_chars_at(change_select_usize-text_list[0].size())
                        };
                        text_list[1] = list[0].to_string();
                        text_list[2] = list[1].to_string() + &text_list[2];
                    }
                }
                Direction::Down => {
                    if text_field.select.is_open_right(){
                        set_down_extend(text_list, text_field, is_ctrl);
                    }
                    else {
                        let list = if *is_ctrl{
                            get_down_ctrl(text_list[1].clone())
                        }
                        else {
                            let change_select_usize = get_down_usize(text_list, text_field.select.0);
                            text_list[1].split_chars_at(change_select_usize-text_list[0].size())
                        };
                        text_list[1] = list[1].to_string();
                        text_list[0] = text_list[0].clone() + &list[0];
                    }
                }
            }
        }
    }
    if !is_extend{
        text_list[1] = "".to_string();
    }
    false
}