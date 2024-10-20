use crate::parser::AST;
use crate::parser::CodeBlock;

pub fn generate_c(code: &CodeBlock) -> anyhow::Result<String> {
    let mut output = std::fs::read_to_string("base.c")?;

    let mut index = 0;
    while index < code.len() {
        match code[index] {
            AST::Add(n) => if n >= 0 {output.push_str(&format!("*ptr+={};", n))} else {output.push_str("*ptr*=2;")},
            AST::Sub(n) => if n >= 0 {output.push_str(&format!("*ptr-={};", n))} else {output.push_str("*ptr=0;")},
            AST::Next(n) => if n >= 0 {output.push_str(&format!("ptr = (ptr - memory + {}) % MEMORY_SIZE + memory;", n))} else {output.push_str("ptr = (ptr - memory + *ptr) % MEMORY_SIZE + memory;")},
            AST::Prev(n) => if n >= 0 {output.push_str(&format!("ptr = (ptr - memory - {} + MEMORY_SIZE) % MEMORY_SIZE + memory;", n))} else {output.push_str("ptr = (ptr - memory - *ptr + MEMORY_SIZE) % MEMORY_SIZE + memory;")},
            AST::LoopStart => output.push_str("while(*ptr){"),
            AST::LoopEnd => output.push_str("}"),
            AST::PrintChar => output.push_str("putchar(*ptr);"),
            AST::ReadChar => output.push_str("*ptr = getchar();"),
            AST::PrintInt => output.push_str("printf(\"%d\\n\", *ptr);"),
            AST::ReadInt => output.push_str("scanf(\"%d\", ptr);"),
            AST::StorePointer => output.push_str("*ptr = ptr;"),
            AST::Jump => output.push_str("ptr = *ptr % MEMORY_SIZE + memory;"),
            AST::JumpToZero => output.push_str("ptr = memory;"),

            _ => {}

        };
        index += 1;
    };
    output.push('}');
    Ok(output)
}
