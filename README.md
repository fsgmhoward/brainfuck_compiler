# (Simple) Brainfuck Compiler

This is a very simple Brainfuck compiler, targetting
X86-64 machine with libc available.

Full language specification could be obtained from:
https://en.wikipedia.org/wiki/Brainfuck

Memory is allocated through `calloc` and total size allocated
and zeroed is 30,000 bytes. No boundary check is embeded and
undefined behaviour will happen if reading beyond boundaries
(most likely wrong calculation result or segfault will happen).

## Undelying Design

Eventhough it is not strictly necessary for a simple language
like this, this compiler still have regular components like
lexer, parser, ast, ir, abstract asm, etc.

Same type of operation is folded at parse time (e.g. `++++` is
folded to `Arith(+4)`) to reduce generated code size. Unsupported
characters are automatically skipped by the lexer.

Function prologue and epilogue are hard-coded in X86-64 assembly.
Output assembly file is in AT&T syntax for GAS to assemble. The
compiler also calls `cc` to assemble the generated assembly and
link it with libc.

## How to Use
The compiler does not have a argument parser and it only accept
one argument, the input file:
```
cargo run -- sample/hello_world.bf
```
