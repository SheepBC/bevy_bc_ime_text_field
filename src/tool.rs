


use crate::text_field::Select;


pub fn splite_text(text:String,splite:Select) -> [String; 3]{
    let front = text.split_chars_at(splite.0 as usize);
    let other = front.1.split_chars_at((splite.1-splite.0) as usize);

    [front.0.to_string(),other.0.to_string(),other.1.to_string()]
}

pub trait ToolString {
    fn split_chars_at(&self, index: usize) -> (String, String);

    fn front_pop(&mut self) -> Option<char>;

    fn size(&self) -> usize;
}

impl ToolString for String {
    fn split_chars_at(&self, index: usize) -> (String, String) {
        let char_index = self.char_indices().nth(index).map(|(i, _)| i).unwrap_or(self.len());
        let (first, rest) = self.split_at(char_index);
        (first.to_string(), rest.to_string())
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
}