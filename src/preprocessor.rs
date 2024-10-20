use crate::{
    parser::{parse, CodeBlock, AST},
    tokenizer::tokenize,
};
use anyhow::anyhow;
use std::{collections::HashMap, path::Path};

pub struct PreprocessorConfig {
    pub include_dirs: Vec<std::path::PathBuf>,
}

fn include_preprocessor(
    code: &CodeBlock,
    preprocessor_config: &PreprocessorConfig,
) -> anyhow::Result<CodeBlock> {
    let mut code_block = Vec::new();
    let mut index: usize = 0;
    while index < code.len() {
        let current_token = code[index].clone();
        match current_token {
            AST::ModuleInclude(module_name) => {
                let mut module_code: String = String::from("");
                for include_dir in preprocessor_config.include_dirs.clone() {
                    module_code = match std::fs::read_to_string(
                        match include_dir.to_str() {
                            Some(dir) => dir,
                            None => continue,
                        }
                        .to_string()
                            + module_name.as_str()
                            + ".bf",
                    ) {
                        Ok(file) => file,
                        Err(_) => continue,
                    }
                }
                if module_code == "" {
                    return Err(anyhow!("Module with name {} wasn't found", module_name));
                }

                let tokens = tokenize(module_code)?;
                let parsed = parse(&tokens)?;
                let preprocessed = include_preprocessor(&parsed, &preprocessor_config)?;
                code_block.extend(preprocessed);
            }

            other => code_block.push(other),
        };
        index += 1;
    }
    Ok(code_block)
}

pub fn preprocessor(
    code: &CodeBlock,
    preprocessor_config: &PreprocessorConfig,
) -> anyhow::Result<CodeBlock> {
    let mut code_block: CodeBlock = CodeBlock::new();
    let code = include_preprocessor(code, &preprocessor_config)?;
    let mut index: usize = 0;
    let mut functions: HashMap<String, CodeBlock> = HashMap::new();
    let mut current_namespace: String = "_main".to_string();
    while index < code.len() {
        let current_token = code[index].clone();
        match current_token {
            AST::FunctionDef(name, func_code) => {
                functions.insert(
                    current_namespace.clone() + &"::".to_string() + &name,
                    func_code,
                );
            }

            AST::NamespaceDef(name) => {
                current_namespace = name;
            }

            AST::FunctionCall(name) => {
                println!("{}", name);
                if let Some(func_code) = functions.get(&name) {
                    code_block.extend(func_code.clone());
                };
            }

            other => code_block.push(other),
        };
        index += 1;
    }
    Ok(code_block)
}
