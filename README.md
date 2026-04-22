# rlox

A small Rust implementation of the Lox language from
[Crafting Interpreters](https://craftinginterpreters.com/).

This project is currently a tree-walk interpreter with a scanner, parser,
AST, environment, and evaluator.

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
- arithmetic and comparison expressions
- equality operators
- unary operators
- variables and assignment
- print statements
- block scopes
- line comments and block comments

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
