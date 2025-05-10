use std::cmp::min;

use crate::{text_field::TextField, tool::ToolString};

use super::control::{get_back_ctrl, get_down_ctrl, get_front_ctrl, get_up_ctrl};


pub(super) fn set_left_area_ctrl(text_list: &mut [String; 3],changed_list: [String; 2]){
    text_list[0] = changed_list[0].clone();
    text_list[2].insert_str(0, &(changed_list[1].clone() + &text_list[1]));
}

pub(super) fn set_left_area(text_list: &mut [String; 3],add_text: String){
    text_list[2].insert_str(0, &(add_text + &text_list[1]));
}

pub(super) fn set_right_area_ctrl(text_list: &mut [String; 3],changed_list: [String; 2]){
    text_list[2] = changed_list[1].clone();
    text_list[0].push_str(&(text_list[1].clone() + &changed_list[0]));
}

pub(super) fn set_right_area(text_list: &mut [String; 3],add_text: String){
    text_list[0].push_str(&(text_list[1].clone() + &add_text));
}

pub(super) fn set_left_extend(text_list: &mut [String; 3],is_ctrl: &bool){
    if *is_ctrl{
        let list = get_front_ctrl(text_list[0].clone());
        text_list[0] = list[0].clone();
        text_list[1].insert_str(0, &list[1]);
    }
    else if let Some(remove) = text_list[0].pop() {
        text_list[1].insert(0, remove);
    }
}

pub(super) fn set_right_extend(text_list: &mut [String; 3],is_ctrl: &bool){
    if *is_ctrl{
        let list = get_back_ctrl(text_list[2].clone());
        text_list[1].push_str(&list[0]);
        text_list[2] = list[1].clone();
    }
    else if let Some(remove) = text_list[2].front_pop(){
        text_list[1].push(remove);
    }
}

pub(super) fn set_up_extend(text_list: &mut [String; 3],text_field:&mut TextField,is_ctrl: &bool){
    let list: [String; 2] = if *is_ctrl{
        get_up_ctrl(text_list[0].clone())
    }
    else {
        let change_select_usize = get_up_usize(text_list, text_field.select.0);
        text_list[0].split_chars_at(change_select_usize)
    };
    text_list[0] = list[0].to_string();
    text_list[1].insert_str(0, &list[1]);
}

pub(super) fn set_down_extend(text_list: &mut [String; 3],text_field:&mut TextField,is_ctrl: &bool){
    let list = if *is_ctrl{
        get_down_ctrl(text_list[2].clone())
    }
    else {
        let change_select_usize = get_down_usize(text_list, text_field.select.1);
        text_list[2].split_chars_at(change_select_usize-text_list[0].size()-text_list[1].size())
    };
    text_list[2] = list[1].to_string();
    text_list[1].push_str(&list[0]);
}

pub(super) fn get_up_usize(text_list: &mut [String; 3],select: usize) -> usize{
    let text = text_list.concat();
    let list: Vec<&str> = text.lines().collect();
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
pub(super) fn get_down_usize(text_list: &mut [String; 3],select: usize) -> usize{
    let text = text_list.concat();
    let list: Vec<&str> = text.lines().collect();
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