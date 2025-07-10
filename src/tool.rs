use crate::text_field::Select;

pub fn splite_text(text: String, splite: Select) -> [String; 3] {
    let front = text.split_chars_at(splite.0 as usize);
    let other = front[1].split_chars_at((splite.1 - splite.0) as usize);

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

pub(crate) fn is_emoji(c: char) -> bool {
    matches!(c as u32,
        0x1F600..=0x1F64F | // 감정
        0x1F300..=0x1F5FF | // 기호와 객체
        0x1F680..=0x1F6FF | // 교통과 장소
        0x2600..=0x26FF   | // 기타 기호
        0x2700..=0x27BF   | // 딩뱃
        0x1F900..=0x1F9FF | // 추가 이모지
        0x1FA70..=0x1FAFF   // 최근 이모지
    )
}
