use core::fmt;

pub enum Token {
    Plus,         // +
    Minus,        // -
    Grater,       // >
    Lesser,       // <
    Period,       // .
    Comma,        // ,
    LeftBracket,  // [
    RightBracket, // ]

    TERM,
}

// for println
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Grater => write!(f, ">"),
            Token::Lesser => write!(f, "<"),
            Token::Period => write!(f, "."),
            Token::Comma => write!(f, ","),
            Token::LeftBracket => write!(f, "["),
            Token::RightBracket => write!(f, "]"),
            Token::TERM => write!(f, "TERM"),
        }
    }
}

pub fn letter_to_token(letter: char) -> Token {
    if letter == '+' {
        return Token::Plus;
    } else if letter == '-' {
        return Token::Minus;
    } else if letter == '>' {
        return Token::Grater;
    } else if letter == '<' {
        return Token::Lesser;
    } else if letter == '.' {
        return Token::Period;
    } else if letter == ',' {
        return Token::Comma;
    } else if letter == '[' {
        return Token::LeftBracket;
    } else if letter == ']' {
        return Token::RightBracket;
    }

    Token::TERM
}
