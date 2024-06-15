# uroboros 🐉

> Uroboros is an ancient symbol depicting a serpent or dragon eating its own tail, this time uroboros is a rust package for making parsers

## Overview

Uroboros is a **Rust** package for making complex parsers with parser combinators and classical state machine LL parsers with **cfg** (context-free grammars). For now this is my personal project, for learning parsing and **Rust**, so do not expect readable and reliable code 🤡

*Name is highly inspired by [github.com/mame/quine-relay](https://github.com/mame/quine-relay)*

> uroboros is a not a new python fork ? 

Cool looking widgets 
<img src="https://img.shields.io/github/license/Maksasj/uroboros" alt="license">
<img src="https://img.shields.io/github/v/release/Maksasj/uroboros" alt="version">
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/Maksasj/uroboros/rust)

### Links
1. Source code avaiable at [github.com/Maksasj/uroboros](https://github.com/Maksasj/uroboros)
2. **Backus-Naur form** wiki [wikipedia.org/wiki/Backus–Naur_form](https://en.wikipedia.org/wiki/Backus%E2%80%93Naur_form)

### Features
- Macros for writing **Backus-Naur form** style grammars 
- **LL grammar parser** Left-to-right, leftmost derivation
- Parser combinators such as **Token**, **Or**, **Many**

## Example
As example we have this simple grammar written in **Backus-Naur form**:

```bash
<expr> := <term> '=' <term>
<term> := <term> '+' <num> | <num>
<num> := 'x' | 'y' | ...
```

With **uroboros** you can write previous grammar in **Rust** like this(Note `Token` is a enum, that represents all terminals)
 
```rust
let _: Grammar<Token> = gram![
    ("expr" => ("term", Token::Equal, "term")),
    ("term" => ("term", Token::Plus, "num") | ("num")),
    ("num" => (Token::VariableLiteral))
];
```

Now you can use this structure in parsing, or modify gramma it self for example removing left recursion with **remove_left_recursion()** method. 

## License
uroboros is free, open source library. All code in this repository is licensed under
- MIT License ([LICENSE.md](https://github.com/Maksasj/uroboros/blob/master/LICENSE.md) or https://opensource.org/license/mit/)