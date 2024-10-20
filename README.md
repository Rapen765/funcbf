# funcbf
Funcbf is a superset of brainfuck with a compiler and preprocessor written in rust.

## Features
The new features in funcbf are:

- ! resets the pointer to 0
- % stores the current pointer in the cell
- ^ dereferences a cell to a pointer position
- : print number in cell (not char)
- ; gets number from user (not char)
- Number before a +, -, <, > repeats the operation
- \* before +, -, <, > replaces the * with the number in cell and repeats the operation.

### Preprocessor
Funcbf contains a preprocessor which allows the definition of functions, namespaces and also importing modules.

- \# name - defines a namespace
- $ func_name code $ - defines a function
- & module_name - imports a module
- @namespace::func_name- executes the function

The default namespace in a file is "main". No namespace can be named "main".
When the preprocessor sees a module import it looks in all of the include dirs for a file "module_name.bf".


