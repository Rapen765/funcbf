use crate::tokenizer::{Token, Tokens};
use anyhow::anyhow;

/// The codeblock type is just a vector of commands
pub type CodeBlock = Vec<AST>;

/// Enum containing all commands
#[derive(Debug, Clone)]
pub enum AST {
    Add(i8),
    Sub(i8),
    Next(i32),
    Prev(i32),
    LoopStart,
    LoopEnd,
    PrintChar,
    ReadChar,
    PrintInt,
    ReadInt,
    StorePointer,
    Jump,
    JumpToZero,
    FunctionDef(String, CodeBlock),
    NamespaceDef(String),
    ModuleInclude(String),
    FunctionCall(String),
}

/// Parses the code and returns a vector of commands.
///
/// # Args:
///     tokens: vector of input tokens (Tokens)
///
/// # Returns:
///     A result containing vector of commands (CodeBlock)
pub fn parse(tokens: &Tokens) -> anyhow::Result<CodeBlock> {
    let mut code_block: CodeBlock = CodeBlock::new();
    let mut index: usize = 0;
    let mut repeat: i32 = 1;
    while index < tokens.len() {
        let current_token = tokens[index].clone();
        match current_token {
            Token::Add => {
                code_block.push(AST::Add(repeat as i8));
                repeat = 1
            }
            Token::Sub => {
                code_block.push(AST::Sub(repeat as i8));
                repeat = 1
            }
            Token::Next => {
                code_block.push(AST::Next(repeat));
                repeat = 1
            }
            Token::Prev => {
                code_block.push(AST::Prev(repeat));
                repeat = 1
            }
            Token::LBracket => code_block.push(AST::LoopStart),
            Token::RBracket => code_block.push(AST::LoopEnd),
            Token::Repeat(n) => repeat = n,
            Token::RepeatCell => repeat = -1,
            Token::StorePointer => code_block.push(AST::StorePointer),
            Token::Jump => code_block.push(AST::Jump),
            Token::JumpToZero => code_block.push(AST::JumpToZero),

            Token::IncludeSymbol => {
                index += 1;
                if let Some(Token::Identifier(string)) = tokens.get(index) {
                    code_block.push(AST::ModuleInclude(string.clone()));
                } else {
                    return Err(anyhow!("Expected identifier"));
                }
            }
            Token::FunctionDefSymbol => {
                index += 1;
                if let Some(Token::Identifier(string)) = tokens.get(index) {
                    index += 1;
                    let mut closing_index = index;
                    for i in index..tokens.len() {
                        if let Token::FunctionDefSymbol = tokens[i] {
                            closing_index = i;
                            break;
                        }
                    }

                    if closing_index == index {
                        return Err(anyhow!("Expected end of function"));
                    }
                    let code = parse(&tokens[index..closing_index].to_vec())?;
                    index = closing_index;
                    code_block.push(AST::FunctionDef(string.clone(), code));
                } else {
                    return Err(anyhow!("Expected identifier"));
                }
            }
            Token::FunctionCallSymbol => {
                index += 1;
                if let Some(Token::Identifier(namespace_name)) = tokens.get(index) {
                    index += 1;
                    if let Some(Token::DoubleColon) = tokens.get(index) {
                        index += 1;
                        if let Some(Token::Identifier(func_name)) = tokens.get(index) {
                            code_block.push(AST::FunctionCall(
                                namespace_name.to_string() + &"::".to_string() + func_name,
                            ));
                        } else {
                            return Err(anyhow!("Expected function name"));
                        }
                    } else {
                        index -= 1;
                        code_block.push(AST::FunctionCall(namespace_name.to_string()));
                    }
                }
            }
            Token::PrintChar => code_block.push(AST::PrintChar),
            Token::ReadChar => code_block.push(AST::ReadChar),
            Token::PrintInt => code_block.push(AST::PrintInt),
            Token::ReadInt => code_block.push(AST::ReadInt),
            Token::NamespaceSymbol => {
                index += 1;
                if let Some(Token::Identifier(name)) = tokens.get(index) {
                    code_block.push(AST::NamespaceDef(name.to_string()));
                };
            }

            _ => {}
        };
        index += 1;
    }
    Ok(code_block)
}
