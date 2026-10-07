use logos::Logos;

#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t\f]+")]
pub enum Token<'s> {
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice())]
    Id(&'s str),

    #[regex(r"x[0-9]+|sp|fp|ra", |lex| {
        let s = lex.slice();
        if s.starts_with('x') {
            s[1..].parse::<u32>().unwrap_or(0)
        } else {
            match s {
                "ra" => 1,
                "sp" => 2,
                "fp" => 8,
                _ => 0,
            }
        }
    })]
    Reg(u32),

    #[regex(r"0[xX][0-9a-fA-F]+", |lex| u32::from_str_radix(&lex.slice()[2..], 16).unwrap())]
    #[regex(r"-?[0-9]+", |lex| lex.slice().parse::<u32>().unwrap())]
    Int(u32),

    #[token(":")]
    Colon,

    #[token(",")]
    Comma,

    #[token("\n")]
    Return,
}

pub fn tokenize(input: &str) -> Result<Vec<Token<'_>>, String> {
    let mut lexer = Token::lexer(input);
    let mut tokens = Vec::new();

    while let Some(token) = lexer.next() {
        match token {
            Ok(t) => tokens.push(t),
            Err(_) => return Err(format!("invalid token: {:?}", lexer.slice())),
        }
    }

    if tokens.last() != Some(&Token::Return) {
        tokens.push(Token::Return);
    }

    Ok(tokens)
}
