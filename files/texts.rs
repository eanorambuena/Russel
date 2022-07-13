// Text is a string of characters struct

#[derive(Clone)]
pub struct Text {
    pub array: Vec<u8>,
    pub content: String,
}

pub fn construct_text(data: Vec<u8>) -> Text {
    let content = String::from_utf8(data.clone()).unwrap();
    return Text {
        array: data,
        content: content,
    };
}

// Split a string into a vector of strings, separated by a given character

pub fn _split(text: String, ch: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let splited_text = text.split(ch);
    let mut sub_string;

    for sub_text in splited_text {
        sub_string = sub_text.to_string();
        result.push(sub_string.clone());
    }
    return result;
}
