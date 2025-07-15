use std::fs::File;
use std::io::prelude::*;
use std::env;
use regex::Regex;

struct Token{
    name: &'static str,
    pattern: Regex
}

#[derive(Debug)]
enum Tokens {
    Identifier,
    Constant,
    IntKeyword,
    ReturnKeyword,
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    Semicolon

}


impl Tokens {
    pub fn get(&self) -> Token {
        match self {
            Tokens::Constant => Token {
                name: "Constant",
                pattern: Regex::new(r"^[0-9]+\b").unwrap(),
            },
            Tokens::Identifier => Token {
                name: "Identifier",
                pattern: Regex::new(r"^[a-zA-Z_]\w*\b").unwrap(),
            },
            Tokens::IntKeyword => Token {
                name: "Int keyword",
                pattern: Regex::new(r"^int\b").unwrap(),
            },
            Tokens::ReturnKeyword => Token {
                name: "Return keyword",
                pattern: Regex::new(r"^return\b").unwrap(),
            },
            Tokens::OpenParenthesis => Token {
                name: "Open parenthesis",
                pattern: Regex::new(r"^\(").unwrap(),
            },
            Tokens::CloseParenthesis => Token {
                name: "Close parenthesis",
                pattern: Regex::new(r"^\)").unwrap(),
            },
            Tokens::OpenBrace => Token {
                name: "Open brace",
                pattern: Regex::new(r"^\{").unwrap(),
            },
            Tokens::CloseBrace => Token {
                name: "Close brace",
                pattern: Regex::new(r"^\}").unwrap(),
            },
            Tokens::Semicolon => Token {
                name: "Semicolon",
                pattern: Regex::new(r"^;").unwrap(),
            },
        }
    }
}



fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut file = File::open(filename).expect("file not fount");
    
    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("file read error");

    println!("text: {}", contents);
    let mut tokens: Vec<Tokens> = vec![];
    while !contents.is_empty(){
        contents = tokenizeInput(&contents, &mut tokens).to_string();
    }

    for token in tokens.iter_mut(){
        println!("{:?}", token);
    }

    }

fn tokenizeInput<'a>(text: &'a str, tokens: &mut Vec<Tokens>) -> &'a str {

    let text = text.trim_start();
    let mut best_match: (usize, Option<Tokens>) = (0, None);
    for token in [
        Tokens::Constant,
        Tokens::Identifier,
        Tokens::IntKeyword,
        Tokens::ReturnKeyword,
        Tokens::OpenParenthesis,
        Tokens::CloseParenthesis,
        Tokens::OpenBrace,
        Tokens::CloseBrace,
        Tokens::Semicolon,
    ] {
        let t = token.get();
        if let Some(mat) = t.pattern.find(text) {
            if mat.start() == 0 {
                if mat.end() > best_match.0 {
                    
                    best_match = (mat.end(), Some(token));
                    println!("{}", best_match.0);
                }
            }
        
        }
    }
    if let Some(tok) = best_match.1 {
        tokens.push(tok);
        &text[best_match.0..] // Move forward
    } else {
            // No match found — stop or handle error
        println!("Unrecognized token near: {}", text);
        &text 
 } 

 }   
