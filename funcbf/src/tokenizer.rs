pub type Tokens = Vec<Token>;
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Add,
    Sub,
    Next,
    Prev,
    LBracket,
    RBracket,
    PrintChar,
    ReadChar,
    PrintInt,
    ReadInt,
    Jump,
    JumpToZero,
    StorePointer,
    RepeatCell,
    FunctionDefSymbol,
    FunctionCallSymbol,
    NamespaceSymbol,
    DoubleColon,
    IncludeSymbol,
    Repeat(i32),
    Identifier(String),
}

pub fn tokenize(code: String) -> anyhow::Result<Tokens> {
    let mut tokens: Tokens = Tokens::new();

    // Tokenizing loop
    let mut index: usize = 0;
    while index < code.len() {
        while get_char_in_string(&code, index).is_whitespace() {
            index += 1;
        }

        match get_char_in_string(&code, index) {
            '+' => tokens.push(Token::Add),
            '-' => tokens.push(Token::Sub),
            '>' => tokens.push(Token::Next),
            '<' => tokens.push(Token::Prev),
            '[' => tokens.push(Token::LBracket),
            ']' => tokens.push(Token::RBracket),
            '.' => tokens.push(Token::PrintChar),
            ',' => tokens.push(Token::ReadChar),
            ';' => tokens.push(Token::ReadInt),
            '$' => tokens.push(Token::FunctionDefSymbol),
            '@' => tokens.push(Token::FunctionCallSymbol),
            '*' => tokens.push(Token::RepeatCell),
            '^' => tokens.push(Token::Jump),
            '%' => tokens.push(Token::StorePointer),
            '!' => tokens.push(Token::JumpToZero),
            ':' => {
                index += 1;
                if get_char_in_string(&code, index) == ':' {
                    tokens.push(Token::DoubleColon);
                    index += 1;
                    continue;
                }
                index -= 1;
                tokens.push(Token::PrintInt);
            }
            '#' => tokens.push(Token::NamespaceSymbol),
            '&' => tokens.push(Token::IncludeSymbol),
            other => {
                if other.is_alphabetic() || other == '_' {
                    let mut string: String = other.to_string();
                    index += 1;
                    let mut current_char: char = get_char_in_string(&code, index);
                    while current_char.is_alphabetic() || current_char == '_' {
                        string.push(current_char);
                        index += 1;
                        current_char = get_char_in_string(&code, index);
                    }
                    tokens.push(Token::Identifier(string));
                    index -= 1;
                } else if other.is_digit(10) {
                    let mut num_string: String = other.to_string();
                    index += 1;
                    let mut current_char: char = get_char_in_string(&code, index);
                    while current_char.is_digit(10) {
                        num_string.push(current_char);
                        index += 1;
                        current_char = get_char_in_string(&code, index);
                    }
                    tokens.push(Token::Repeat(num_string.parse()?));
                    index -= 1;
                }
            }
        };
        index += 1;
    }

    Ok(tokens)
}

/// Get char at a index <index> of string <string>

pub fn get_char_in_string(string: &String, index: usize) -> char {
    let chars = string.chars();
    if index < chars.clone().collect::<Vec<_>>().len() {
        chars.collect::<Vec<_>>()[index]
    } else {
        '\0'
    }
}
