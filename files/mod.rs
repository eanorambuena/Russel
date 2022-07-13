use std::fs::File;
use std::io::Read;

pub mod texts;

fn read_a_file(path: String) -> std::io::Result<Vec<u8>> {
    let mut file = (File::open(path))?;

    let mut data = Vec::new();
    (file.read_to_end(&mut data))?;

    return Ok(data);
}

pub fn read(path: String) -> std::io::Result<texts::Text> {
    let data = read_a_file(path).unwrap();
    return Ok(texts::construct_text(data));
}
