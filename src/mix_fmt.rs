use crate::mix_types::*;
use crate::mix_util;

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
    let sign = if mix_util::get_bytes(w, 0, 0) != 0 { '-' } else { '+' };
    let b1 = mix_util::get_bytes(w, 1, 1);
    let b2 = mix_util::get_bytes(w, 2, 2);
    let b3 = mix_util::get_bytes(w, 3, 3);
    let b4 = mix_util::get_bytes(w, 4, 4);
    let b5 = mix_util::get_bytes(w, 5, 5);
    format!(" {} {:2} {:2} {:2} {:2} {:2}", sign, b1, b2, b3, b4, b5)
}
