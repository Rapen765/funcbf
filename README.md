# funcbf
Funcbf is a superset of brainfuck with a compiler and preprocessor written in rust.

## Features
The new features in funcbf are:

The new features in Funcbf are:

- **`!`**: Resets the pointer to 0.
- **`%`**: Stores the current pointer in the cell.
- **`^`**: Dereferences a cell to a pointer position.
- **`:`**: Prints the number in a cell (not a character).
- **`;`**: Gets a number from user input (not a character).
- **Number before `+`, `-`, `<`, `>`**: Repeats the operation the specified number of times.
- **`*` before `+`, `-`, `<`, `>`**: Replaces the `*` with the number in the cell and repeats the operation.

## Preprocessor

Funcbf contains a preprocessor that allows the definition of functions, namespaces, and the importing of modules.

- **`# name`**: Defines a namespace.
- **`$ func_name code $`**: Defines a function.
- **`& module_name`**: Imports a module.
- **`@namespace::func_name`**: Executes the function.

The default namespace in a file is "main". No namespace can be named "main". When the preprocessor sees a module import, it looks in all of the include directories for a file named `module_name.bf`.


