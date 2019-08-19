use std::str::FromStr;
use crate::mix_types::*;
use crate::mix_util;
use crate::arch_util;

#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: &'static str,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Parsing error: {}", self.message)
    }
}

impl std::error::Error for ParseError {
    fn description(&self) -> &str { &self.message }
}

// Read in a MIX program as a
// row-by-row string listing of MIX words and convert it to
// a sequence of raw bytes (in little-endian word order). Parse
// each word using the parse_word function below.

pub fn parse_words(in_words: &String) -> Result<Vec<u8>, ParseError> {
    let mut bytes: Vec<u8> = vec![];
    for s in in_words.lines() {
        match parse_word(s) {
            Ok(w) => {
                bytes.extend(arch_util::u32to8s(w).iter());
            },
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok(bytes)
}

// Parse a line of the form returned by fmt_word() into a MixWord.
// Return an error if the line cannot be parsed.
// Lines should have the form
// <sign> <b1> <b2> <b3> <b4> <b5>
// for example
// + 20 42 10 0 5
// All bytes specified must be between 0 and MIX_BYTE_MAX.

pub fn parse_word(s: &str) -> Result<MixWord, ParseError> {
    let sign: u8;
    let mut bytes: [u8; 5] = [0; 5];
    let toks: Vec<&str> = s.split_whitespace().collect();
    if toks.len() < 6 {
        return Err(ParseError {
            message: "Too few tokens! MIX words have 5 bytes and a sign.",
        });
    } else if toks.len() > 6 {
        return Err(ParseError {
            message: "Too many tokens! MIX words have 5 bytes and a sign.",
        });
    }
    match toks[0] {
        "+" => {
            sign = 0;
        },
        "-" => {
            sign = 1;
        }
        _ => {
            return Err(ParseError {
                message: "Invalid sign. Sign must be '+' or '-'",
            });
        }
    }
    for i in 0..5 {
        match u8::from_str(toks[i+1]) {
            Ok(b) => {
                if b <= MIX_BYTE_MAX {
                    bytes[i] = b;
                } else {
                    return Err(ParseError {
                        message: "Invalid byte token. Byte must fit \
                        into a MIX byte.",
                    });
                }
            },
            Err(_e) => {
                return Err(ParseError {
                    message: "Invalid byte token. Byte must be a \
                    small nonnegative number.",
                });
            },
        }
    }
    Ok(mix_util::word_from_bytes(sign, bytes[0], bytes[1],
                                 bytes[2], bytes[3], bytes[4]))
}
