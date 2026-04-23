# rlox

A small Rust implementation of the Lox language from
[Crafting Interpreters](https://craftinginterpreters.com/).

This project is currently a tree-walk interpreter with a scanner, parser,
AST, lexical environments, callable values, and evaluator.

It is still a work in progress, so many parts of the full Lox language are
not implemented yet.

## Usage

Start the REPL:

```sh
cargo run
```

Run a Lox file:

```sh
cargo run -- test.lox
```

## REPL

In the REPL, bare expressions are evaluated and printed automatically:

```lox
> 1 + 2
3
> var a = 10;
> a
10
> a = 20
20
```

The REPL also supports multi-line input. It keeps reading when a line ends
with `\`, when a block is not closed yet, or when a string literal is still
open. Nested blocks are shown with automatic indentation:

```lox
> {
|   var message = "hello";
|   print message;
| }
hello
```

Line continuation can be used for long expressions:

```lox
> 1 + \
| 2 + \
| 3
6
```

## Supported Features

- numbers, strings, booleans, and nil
- arithmetic, comparison, equality, unary, and logical expressions
- variables and assignment
- print statements and block scopes
- if / else statements, while loops, and for loops
- break, continue, and return control flow
- functions and closures
- native functions, including `clock()`
- line comments and block comments

`return`, `break`, and `continue` are checked while parsing. For example,
top-level `return` and loop control outside of a loop are reported before
evaluation starts.

## Not Yet Implemented

- classes and instances
- methods and initializers
- inheritance
- lists or maps
- static resolution pass

## Example

```lox
var a = "global a";
var b = "global b";

{
  var a = "inner a";
  print a;
  print b;
}

print a;
```

## Functions

```lox
fun add(a, b) {
  return a + b;
}

print add(2, 3);
```

Functions close over the environment where they are declared:

```lox
var message = "hello";

fun sayMessage() {
  print message;
}

sayMessage();
```

## Loops

```lox
for (var i = 0; i < 5; i = i + 1) {
  if (i == 2) continue;
  if (i == 4) break;
  print i;
}
```
