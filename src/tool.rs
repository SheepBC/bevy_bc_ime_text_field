use crate::text_field::Select;

pub fn split_text(text: String, split: Select) -> [String; 3] {
    let front = text.split_chars_at(split.0);
    let other = front[1].split_chars_at(split.1 - split.0);

    [
        front[0].to_string(),
        other[0].to_string(),
        other[1].to_string(),
    ]
}

pub trait ToolString {
    fn split_chars_at(&self, index: usize) -> [String; 2];

    fn front_pop(&mut self) -> Option<char>;

    fn size(&self) -> usize;

    fn slice(&self, num: usize) -> String;
}

impl ToolString for String {
    fn split_chars_at(&self, index: usize) -> [String; 2] {
        let char_index = self
            .char_indices()
            .nth(index)
            .map(|(i, _)| i)
            .unwrap_or(self.len());
        let (first, rest) = self.split_at(char_index);
        [first.to_string(), rest.to_string()]
    }

    fn front_pop(&mut self) -> Option<char> {
        self.chars().next().map(|first_char| {
            *self = self.chars().skip(1).collect();
            Some(first_char)
        })?
    }

    fn size(&self) -> usize {
        self.chars().count()
    }

    fn slice(&self, num: usize) -> String {
        let arr: Vec<String> = self.chars().map(|c| c.to_string()).collect();
        if num > arr.len() {
            return self.clone();
        }
        arr[0..num].join("")
    }
}
