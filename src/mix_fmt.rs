use crate::mix_types::*;

// Format a MIX program (a raw byte string) as
// row-by-row string listing of MIX words. Format
// each word using the fmt_word function below.
pub fn fmt_words(in_bytes: &Vec<u8>) -> String {
    let mut ix: usize = 0;
    let mut fmt_str = String::new();
    while ix + 3 < in_bytes.len() {
        let w = word_from(&in_bytes[ix..ix+3]);
        fmt_str.push_str(&fmt_word(w));
        fmt_str.push('\n');
        ix += 4;
    }
    fmt_str
}

pub fn fmt_word(w: MixWord) -> String {
    return String::new();
}
