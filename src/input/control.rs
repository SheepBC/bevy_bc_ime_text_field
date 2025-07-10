use crate::tool::ToolString;

pub(super) fn get_front_ctrl(text: String) -> [String; 2] {
    let list = get_ctrl_list(text.clone(), true);
    if list.2.is_none() {
        return [String::new(), text];
    }
    match list.2.unwrap() {
        TextDataType::Blank => {
            let front_list = get_front_ctrl(list.0.clone());
            [front_list[0].clone(), front_list[1].clone() + &list.1]
        }
        TextDataType::None => {
            let front_list = get_ctrl_list(list.0.clone(), true);
            if front_list.2.is_none() {
                return [String::new(), text];
            }
            match front_list.2.unwrap() {
                TextDataType::Blank => [list.0, list.1],
                TextDataType::Pause => [front_list.0, front_list.1 + &list.1],
                _ => [list.0, list.1],
            }
        }
        _ => [list.0, list.1],
    }
}

pub(super) fn get_back_ctrl(text: String) -> [String; 2] {
    let list = get_ctrl_list(text.clone(), false);
    if list.2.is_none() {
        return [text, String::new()];
    }
    match list.2.unwrap() {
        TextDataType::None => {
            let back_list = get_ctrl_list(list.1.clone(), false);
            if back_list.2.is_none() {
                return [text, String::new()];
            }
            match back_list.2.unwrap() {
                TextDataType::Blank => [list.0 + &back_list.0, back_list.1],
                TextDataType::None => [text, String::new()],
                _ => [list.0, list.1],
            }
        }
        _ => {
            if list.0 == "" {
                return [text, String::new()];
            }
            [list.0, list.1]
        }
    }
}

fn get_ctrl_list(text: String, is_front: bool) -> (String, String, Option<TextDataType>) {
    let mut t_list: Vec<char> = text.chars().collect();
    if is_front {
        t_list.reverse();
    }
    let mut last_char = TextData(None);
    let mut index = 0;
    for (i, t) in t_list.iter().enumerate() {
        let str_t = t.to_string();
        let mut now_char = TextData(None);

        match str_t.as_str() {
            "\t" => {
                now_char.0 = Some(TextDataType::Tab);
            }
            " " => {
                now_char.0 = Some(TextDataType::Blank);
            }
            "\n" => {
                now_char.0 = Some(TextDataType::NextLine);
            }
            "," => {
                now_char.0 = Some(TextDataType::Pause);
            }
            _ => {
                now_char.0 = Some(TextDataType::None);
            }
        }

        let end_list: Vec<String> = "~!@#$%^&*()_+`{}|[]\\:\";',./<>?"
            .chars()
            .map(|c| c.to_string())
            .collect();
        if end_list.contains(&str_t) {
            now_char.0 = Some(TextDataType::End);
        }

        if let Some(_) = last_char.0 {
            if last_char != now_char {
                let len = t_list.len();
                if is_front {
                    index = len - i;
                } else {
                    index = i;
                }
                break;
            }
        }
        last_char = now_char;
    }
    let rst_list = text.to_string().split_chars_at(index);
    (rst_list[0].clone(), rst_list[1].clone(), last_char.0)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct TextData(Option<TextDataType>);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TextDataType {
    End,
    Tab,
    Blank,
    Pause,
    NextLine,
    None,
}

pub(super) fn get_up_ctrl(mut text: String) -> [String; 2] {
    let pop = text.clone().pop();

    if pop == None {
        return [String::new(), text];
    }

    if let Some(last) = pop {
        if last.to_string() == "\n".to_string() {
            text.pop();
            return [text, "\n".to_string()];
        }
        let list = text.split("\n").collect::<Vec<&str>>();
        let split_list = list.split_last();
        if let Some((last, other)) = split_list {
            let mut front = String::new();
            for t in other {
                front += *t;
                front += "\n";
            }
            return [front, last.to_string()];
        }
    }
    [String::new(), text]
}

pub(super) fn get_down_ctrl(mut text: String) -> [String; 2] {
    let pop = text.clone().front_pop();

    if pop == None {
        return [text, String::new()];
    }

    if let Some(first) = pop {
        if first.to_string() == "\n".to_string() {
            text.front_pop();
            return ["\n".to_string(), text];
        }
        let list = text.split("\n").collect::<Vec<&str>>();
        let split_list = list.split_first();
        if let Some((first, other)) = split_list {
            let mut back = String::new();
            for t in other {
                back += *t;
                back += "\n";
            }
            back.pop();
            let mut add = String::new();
            if back != "" {
                add = "\n".to_string();
            }
            return [first.to_string(), add + &back];
        }
    }
    [text, String::new()]
}
