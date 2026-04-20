#[derive(Debug, Clone, Copy)]
pub enum Token {
    MoveRight,
    MoveLeft,
    Increment,
    Decrement,
    Output,
    Input,
    LoopStart,
    LoopEnd,
}

pub fn tokenize(input: String) -> Vec<Token> {
    input
        .chars()
        .filter_map(|s| match s {
            '>' => Some(Token::MoveRight),
            '<' => Some(Token::MoveLeft),
            '+' => Some(Token::Increment),
            '-' => Some(Token::Decrement),
            '.' => Some(Token::Output),
            ',' => Some(Token::Input),
            '[' => Some(Token::LoopStart),
            ']' => Some(Token::LoopEnd),
            _ => None,
        })
        .collect()
}
