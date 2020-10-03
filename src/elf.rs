pub struct ParsedElf {
    pub filename: String,
    pub contents: Vec<u8>,
}

impl ParsedElf {
    pub fn from_bytes(filename: &String, buf: Vec<u8>) -> ParsedElf {
        // check header

        ParsedElf {
            filename: filename.clone(),
            contents: buf,
        }
    }
}

