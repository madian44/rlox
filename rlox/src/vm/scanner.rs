use std::{iter::Peekable, str::CharIndices};

use crate::{
    vm::token::{Token, TokenType},
    Region,
};

macro_rules! two_char_token {
    ($scanner:expr, $char_to_match:literal, $token_type:expr, $alt_token_type:expr) => {{
        if $scanner.match_next_char($char_to_match) {
            $scanner.make_token($token_type)
        } else {
            $scanner.make_token($alt_token_type)
        }
    }};
}

// NOTE
// * line numbers and line offsets are zero based
// * lexemes are from start char to one after the last character

pub struct CharacterCache {
    cached_char: Option<(usize, char)>
}

impl CharacterCache {
    fn new() -> Self {
        Self {
            cached_char: None
        }
    }

    fn is_some(&self) -> bool {
        self.cached_char.is_some()
    }

    fn is_none(&self) -> bool {
        self.cached_char.is_none()
    }

    fn store(&mut self, new_value: Option<(usize, char)>) {
        self.cached_char = new_value;
    }

    fn consume(&mut self) -> Option<(usize, char)> {
        let tmp = self.cached_char;
        self.cached_char = None;
        tmp
    }

    fn as_ref(&self) -> Option<&(usize, char)> {
        self.cached_char.as_ref()
    }

    fn map_to_char(&self) -> Option<char> {
        self.cached_char.map(|(_, c)| c)
    }
}

pub struct Scanner<'a> {
    token_start_line_number: u16,
    token_start_line_offset: u16,
    current_line_number: u16,
    current_line_offset: i16,
    start_of_token: usize,
    current: usize,
    source: &'a str,
    char_indices: Peekable<CharIndices<'a>>,
    character_cache: CharacterCache
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            token_start_line_number: 0,
            token_start_line_offset: 0,
            current_line_number: 0,
            current_line_offset: -1,
            start_of_token: 0,
            current: 0,
            source,
            char_indices: source.char_indices().peekable(),
            character_cache: CharacterCache::new()
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_white_space();

        if self.is_at_end() {
            self.start_of_token();
            return self.make_token(TokenType::Eof);
        }

        let c = self.advance(false).unwrap();
        self.start_of_token();
        if is_alpha(Some(c)) {
            return self.make_identifier_token()
        }
        if is_digit(Some(c)) {
            return self.make_number_token()
        }
        match c {
            '(' => return self.make_token(TokenType::LeftParen),
            ')' => return self.make_token(TokenType::RightParen),
            '{' => return self.make_token(TokenType::LeftBrace),
            '}' => return self.make_token(TokenType::RightBrace),
            ';' => return self.make_token(TokenType::Semicolon),
            ',' => return self.make_token(TokenType::Comma),
            '.' => return self.make_token(TokenType::Dot),
            '-' => return self.make_token(TokenType::Minus),
            '+' => return self.make_token(TokenType::Plus),
            '/' => return self.make_token(TokenType::Slash),
            '*' => return self.make_token(TokenType::Star),
            '!' => return two_char_token!(self, '=', TokenType::BangEqual, TokenType::Bang),
            '=' => return two_char_token!(self, '=', TokenType::EqualEqual, TokenType::Equal),
            '<' => return two_char_token!(self, '=', TokenType::LessEqual, TokenType::Less),
            '>' => return two_char_token!(self, '=', TokenType::GreaterEqual, TokenType::Greater),
            '"' => return self.make_string_token(),
            _ => (),
        }

        return self.make_error_token("Unexpected character");
    }

    fn start_of_token(&mut self) {
        self.start_of_token = self.current;
        self.token_start_line_number = self.current_line_number;
        self.token_start_line_offset = u16::try_from(self.current_line_offset).unwrap_or(0);
    }

    fn new_line(&mut self) {
        self.current_line_number += 1;
        self.current_line_offset = -1;
    }

    fn make_token(&self, token_type: TokenType) -> Token {
        let include_lexeme = token_type != TokenType::Eof ;
        Token::new(
            token_type,
            if include_lexeme {&self.source[self.start_of_token..(self.current+1)]} else {""},
            Region::new(
                self.token_start_line_number,
                self.token_start_line_offset,
                self.current_line_number,
                u16::try_from(self.current_line_offset).unwrap_or(0)+1
            ),
        )
    }

    fn make_error_token(&self, message: &'static str) -> Token {
        Token::new(
            TokenType::Error,
            message,
            Region::new(
                self.token_start_line_number,
                self.token_start_line_offset,
                self.current_line_number,
                u16::try_from(self.current_line_offset).unwrap_or(0)+1
            ),
        )
    }

    fn make_string_token(&mut self) -> Token {
        while !self.is_at_end() {
            let next_char = self.peek_next_char(false);
            match next_char {
                Some('"') => break,
                Some('\n') => {self.advance(false); self.new_line();},
                _ => {self.advance(false);},
            }
        }
        if self.is_at_end() {
            self.make_error_token("Unterminated string")
        } else {
            self.advance(false);
            self.make_token(TokenType::String)
        }
    }

    fn make_number_token(&mut self) -> Token {
        while is_digit(self.peek_next_char(false)) {
            self.advance(false);
        }
        if let Some('.') = self.peek_next_char(false) {
            if is_digit(self.peek_next_next_char()) {
                self.advance(false);

                while is_digit(self.peek_next_char(false)) {
                    self.advance(false);
                }
            }
        }

        self.make_token(TokenType::Number)
    }

    fn make_identifier_token(&mut self) -> Token {
        while is_alpha(self.peek_next_char(false)) || is_digit(self.peek_next_char(false)){
            self.advance(false);
        }

        self.make_token(identifier_type(
            &self.source[self.start_of_token..(self.current+1)]
        ))
    }

    fn advance(&mut self, ignore_cache: bool) -> Option<char> {
        let next_char = if !ignore_cache && self.character_cache.is_some() {
            self.character_cache.consume()
        } else {
            self.char_indices.next()
        };
        if let Some((i, c)) = next_char {
            self.current_line_offset += 1;
            self.current = i;
            Some(c)
        } else {
            None
        }
    }

    fn is_at_end(&mut self) -> bool {
        self.character_cache.is_none() && self.char_indices.peek().is_none()
    }

    fn match_next_char(&mut self, char_to_match: char) -> bool {
        let next_char = if self.character_cache.is_some() {
            self.character_cache.as_ref()
        } else {
            self.char_indices.peek()
        };
        if let Some((_, c)) = next_char {
            if *c == char_to_match {
                self.advance(false);
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn peek_next_next_char(&mut self) -> Option<char> {
        self.character_cache.store(self.char_indices.next());
        self.peek_next_char(true)
    }

    fn peek_next_char(&mut self, ignore_cache: bool) -> Option<char> {
        if !ignore_cache && self.character_cache.is_some() {
            self.character_cache.map_to_char()
        } else {
            self.char_indices.peek().map(|(_, c)| *c)
        }
    }

    fn skip_white_space(&mut self) {
        loop {
            match self.peek_next_char(false) {
                Some(' ') | Some('\r') | Some('\t') => {
                    self.advance(false);
                }
                Some('\n') => {
                    self.advance(false);
                    self.new_line();
                }
                //** Need to peek the next character **
                Some('/') => {
                    if let Some('/') = self.peek_next_next_char() {
                        self.consume_line()
                    }
                    if self.character_cache.is_some() {
                        return;
                    }
                }
                _ => return,
            }
        }
    }

    fn consume_line(&mut self) {
        loop {
            match self.peek_next_char(false) {
                None => return,
                Some('\n') => return,
                _ => { self.advance(false); }
            }
        }

    }
}

fn is_digit(c: Option<char>) -> bool {
    c.map_or(false, |v| v.is_ascii_digit() )
}

fn is_alpha(c: Option<char>) -> bool {
    c.map_or(false, |v| v.is_ascii_lowercase() || v.is_ascii_uppercase() || v == '_')
}

fn identifier_type(lexeme: &str) -> TokenType {
    match &lexeme[0..1] {
        "a" => check_keyword(lexeme, 1, "nd", TokenType::And),
        "c" => check_keyword(lexeme, 1, "lass", TokenType::Class),
        "e" => check_keyword(lexeme, 1, "lse", TokenType::Else),
        "f" if lexeme.len() > 1 => {
                match &lexeme[1..2] {
                    "a" => check_keyword(lexeme, 2, "lse", TokenType::False),
                    "o" => check_keyword(lexeme, 2, "r", TokenType::For),
                    "u" => check_keyword(lexeme, 2, "n", TokenType::Fun),
                    _ => TokenType::Identifier
                }
            }
        "i" => check_keyword(lexeme, 1, "f", TokenType::If),
        "n" => check_keyword(lexeme, 1, "il", TokenType::Nil),
        "o" => check_keyword(lexeme, 1, "r", TokenType::Or),
        "p" => check_keyword(lexeme, 1, "rint", TokenType::Print),
        "r" => check_keyword(lexeme, 1, "eturn", TokenType::Return),
        "s" => check_keyword(lexeme, 1, "uper", TokenType::Super),
        "t" if lexeme.len() > 1 => {
            match &lexeme[1..2] {
                "h" => check_keyword(lexeme, 2, "is", TokenType::This),
                "r" => check_keyword(lexeme, 2, "ue", TokenType::True),
                _ => TokenType::Identifier
            }
        }
        "v" => check_keyword(lexeme, 1, "ar", TokenType::Var),
        "w" => check_keyword(lexeme, 1, "hile", TokenType::While),
        _ => TokenType::Identifier
        
    }
}

fn check_keyword(lexeme: &str, start: usize, rest: &str, token_type: TokenType) -> TokenType {
    if (lexeme.len() == start + rest.len()) && &lexeme[start..start+rest.len()] == rest {
        token_type
    } else {
        TokenType::Identifier
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokens() {
        let tests = vec![
            ("", make_token(TokenType::Eof, "", 0, 0, 0, 1)),
            ("//comment
             !
             ", make_token(TokenType::Bang, "!", 1, 13, 1, 14)),
            ("!", make_token(TokenType::Bang, "!", 0, 0, 0, 1)),
            ("!=", make_token(TokenType::BangEqual, "!=", 0, 0, 0, 2)),
            ("/", make_token(TokenType::Slash, "/", 0, 0, 0, 1)),
            ("//", make_token(TokenType::Eof, "", 0, 1, 0, 2)),
            ("//comment", make_token(TokenType::Eof, "", 0, 8, 0, 9)),
            (" .", make_token(TokenType::Dot, ".", 0, 1, 0, 2)),
            ("! =", make_token(TokenType::Bang, "!", 0, 0, 0, 1)),
            (" !=", make_token(TokenType::BangEqual, "!=", 0, 1, 0, 3)),
            (" ! =", make_token(TokenType::Bang, "!", 0, 1, 0, 2)),
            ("\"hello\"", make_token(TokenType::String, "\"hello\"", 0, 0, 0, 7)),
            ("\"hello
            \"", make_token(TokenType::String, "\"hello\n            \"", 0, 0, 1, 13)),
            ("\"hello", make_token(TokenType::Error, "Unterminated string", 0, 0, 0, 6)),
            ("10", make_token(TokenType::Number, "10", 0, 0, 0, 2)),
            ("10.", make_token(TokenType::Number, "10", 0, 0, 0, 2)),
            ("10.1", make_token(TokenType::Number, "10.1", 0, 0, 0, 4)),
            ("10.1.", make_token(TokenType::Number, "10.1", 0, 0, 0, 4)),
            ("10.1.0", make_token(TokenType::Number, "10.1", 0, 0, 0, 4)),
            (".0", make_token(TokenType::Dot, ".", 0, 0, 0, 1)),
            ("hello", make_token(TokenType::Identifier, "hello", 0, 0, 0, 5)),
            ("h3llo", make_token(TokenType::Identifier, "h3llo", 0, 0, 0, 5)),
            ("4ello", make_token(TokenType::Number, "4", 0, 0, 0, 1)),
            ("a", make_token(TokenType::Identifier, "a", 0, 0, 0, 1)),
            ("and", make_token(TokenType::And, "and", 0, 0, 0, 3)),
            ("andy", make_token(TokenType::Identifier, "andy", 0, 0, 0, 4)),
            ("f", make_token(TokenType::Identifier, "f", 0, 0, 0, 1)),
            ("for", make_token(TokenType::For, "for", 0, 0, 0, 3)),
            ("fore", make_token(TokenType::Identifier, "fore", 0, 0, 0, 4)),
        ];

        for (src, expected) in tests {
            let mut scanner = Scanner::new(src);
            let token = scanner.scan_token();

            assert_eq!(token, expected, "Unexpected token {expected:?} for source '{src}'")
        }
    }

    fn make_token(token_type: TokenType, lexeme: &str, start_line: u16, start_char: u16, end_line: u16, end_char: u16) -> Token {
        Token::new(token_type, lexeme, Region::new(start_line, start_char, end_line, end_char))
    }

}
