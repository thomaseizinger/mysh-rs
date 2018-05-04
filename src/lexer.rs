use std::collections::LinkedList;
use std::vec;

lazy_static! {
    static ref SYMBOLS: vec::Vec<&'static str> = {
        let mut m = vec!["&&", ";", "&", "|", ">", ">>", "<", "<<", "||"];

        // sort for longest match rule
        m.sort_by(|a, b| b.cmp(a));
        m
    };
}

fn starts_with_symbol(line: &str) -> Option<&'static str> {
    SYMBOLS
        .iter()
        .find(|&&sym| line.starts_with(sym))
        .map(|sym| *sym)
}

#[derive(PartialEq, Debug)]
enum Token<'a> {
    WhiteSpace,
    Symbols(&'static str), // i.e ';', '&', etc
    VarString(&'a str),    // string slice representing commands, parameters to commands, etc
}

pub struct LexicalAnalyzer<'a> {
    token_list: LinkedList<Token<'a>>,
}

impl<'a> LexicalAnalyzer<'a> {
    pub fn new() -> LexicalAnalyzer<'a> {
        LexicalAnalyzer {
            token_list: LinkedList::new(),
        }
    }

    pub fn tokenize(&mut self, string: &'a str) {
        let mut it = string.chars().enumerate().peekable();

        let mut start = 0;
        let mut capturing = false;

        loop {
            let mut omit_current_ch_for_capture = true;

            if let Some((i, ch)) = it.next() {
                match ch {
                    '\t' | ' ' => {
                        // ignore if last token was whitespace
                        // if self.token_list.back() != Some(&Token::WhiteSpace) {
                        self.token_list.push_back(Token::WhiteSpace);
                        // }
                    }
                    '"' | '\'' => {
                        // extract string literal in between quotes
                        let c = it.position(|(_, _ch)| _ch == ch).unwrap();
                        let (start, end) = (i + 1, i + c + 1);
                        self.token_list
                            .push_back(Token::VarString(&string[start..end]));
                    }
                    _ => {
                        let remaining_str = &string[i..];
                        if let Some(s) = starts_with_symbol(remaining_str) {
                            self.token_list.push_back(Token::Symbols(s));
                            for _ in 0..s.len() {
                                it.next();
                            }
                        } else {
                            omit_current_ch_for_capture = false;
                            if !capturing {
                                capturing = true;
                                start = i;
                            }
                        }
                    }
                }

                if capturing && omit_current_ch_for_capture {
                    capturing = false;
                    let end = i;
                    if let Some(previous_token) = self.token_list.pop_back() {
                        self.token_list
                            .push_back(Token::VarString(&string[start..end]));
                        self.token_list.push_back(previous_token);
                    } else {
                        self.token_list
                            .push_back(Token::VarString(&string[start..end]));
                    }
                }
                if capturing && it.peek().is_none() {
                    capturing = false;
                    self.token_list
                        .push_back(Token::VarString(&string[start..]));
                }
            } else {
                break;
            }
        }
    }
}

#[test]
fn test_analyzer() {
    let string = "Hello World, you are \"sometimes\" 'ok' && sometimes not!!!";
    println!("String: {}", string);
    let mut lexer = LexicalAnalyzer::new();
    lexer.tokenize(&string);
    println!("Token list: {:?}", lexer.token_list);
}

#[test]
fn test_symbol_presence() {
    let string = "<< Help";
    match starts_with_symbol(string) {
        Some(x) => println!("{}", x),
        None => (),
    }
}
