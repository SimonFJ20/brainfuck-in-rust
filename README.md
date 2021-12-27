
# brainfuck-in-rust

Efficient [Brainfuck](https://en.wikipedia.org/wiki/Brainfuck) Interpreter in Rust

"Efficient" because:
1. [Rust is fast](https://www.rust-lang.org/)
2. It parses the text to instructions before interpreting

The VM's "memory" is refered to as 'the stack', acknowledging that 'the stack' isn't a [Stack](https://en.wikipedia.org/wiki/Stack_(abstract_data_type)) but a variable [Array](https://en.wikipedia.org/wiki/Array_data_type).

But not as efficient as it could be, because the VM's stack is a `Vec`, so the VM may need to reallocate the entire stack when the stack grows.

**This interpreter has no error handling, and will only assume your program to be correct.**

## [Parser](src/parser.rs)

Converts brainfuck
```brainfuck
+-<>[].,
```
into instructions
```
0   add
1   sub
2   left
3   right
4   jz  5
5   jmp 4
6   output
7   input 
```

### Text to tokens

```rust
enum Token {
    ADD,
    SUB,
    LEFT,
    RIGHT,
    BEGIN,
    END,
    INPUT,
    OUTPUT,
}
```

It starts by converting text into `Token`s, ignoring non-Brainfuck characters in the process.

### Pair braces

```rust
struct BracePair {
    begin: usize,
    end: usize,
}
```

This step takes the `Token`s and finds all the `[` and `]` and then returns them in `BracePairs`s.

### Tokens to instructions

```rust
enum Ops {
    EXIT,
    LEFT,   RIGHT,
    ADD,    SUB,
    INPUT,  OUTPUT,
    JZ,     // (j)ump to program[value] if stack[sp] is equal to (z)ero
    JMP,    // jump to program[value]
}
struct Instruction {
    op: Ops,
    value: usize,
}
```

This is the final step of the parser. It takes the `Token`s and `BracePairs`s and converts them into `Instruction`s with respective operation (`Ops`) and value. 

The value is used for the destination of `JZ` and `JMP`.

Value could also be used to compile multiples of the same value, eg. `++++++` into `add  6`, which could save ram usage and increase performance.

## VM

```rust
enum Ops {
    EXIT,
    LEFT,   RIGHT,
    ADD,    SUB,
    INPUT,  OUTPUT,
    JZ,     // (j)ump to program[value] if stack[sp] is equal to (z)ero
    JMP,    // jump to program[value]
}
struct Instruction {
    op: Ops,
    value: usize,
}

```
This is the "Virtual Machine"/Runtime enviroment of the interpreter. This takes a program consisting of `Instruction`s and executes them.

### Context

```rust
struct Context {                // instances shortened to 'ctx'
    program: Vec<Instruction>,
    pc: usize,                  // (p)rogram (c)ounter, keeps track which instruction we're at
    stack: Vec<u8>,
    sp: usize,                  // (s)tack (p)ointer, keeps track of the position in the stack
}
```

The VM uses a executing-"context" to keep track of the program and stack.

### Stack

The stack is a `Vec` of bytes. 

Brainfuck specifies the stack to have a minimum size of `3 * 10^5 <=> 30,000`, but the VM uses a dynamic `Vec` for the stack, therefore it doesn't allocate ram it doesn't need, and it can grow to infinity size, limited by hardware of course.

Though a dynamic `Vec`, each time the stack grows, the VM might have to reallocate and move the entire stack in ram.

### Execution

The VM runs through each `Instruction` of the program, matches the operation (`Ops`) and executes the corresponding function.

It increments the program counter (`ctx.pc`) after each iteration.

It starts with the program counter (`ctx.pc`) and stack pointer (`ctx.sp`) at `0`.

It runs until it reaches an `Ops::END`-instruction.

### Instructions

#### EXIT

Instructs the VM to exit the program.

Operation       `EXIT`

Prominent at [`src/vm.rs:89:37`](https://github.com/SimonFJ20/brainfuck-in-rust/blob/main/src/vm.rs#L89).



#### ADD

Increments the value of the current position of the stack, or resetting it to zero if it is equal to `255 <=> 0xFF`.

`ctx.stack[ctx.sp] < 255 ? ctx.stack[ctx.sp] + 1 : 0`

| Name | Value |
|------|-------|
| Brainfuck     | `+` |
| Token         | `ADD` |
| Operation     | `ADD` |
| VM function   | `add` |

#### SUB

Decrements the value of the current position of the stack, or resetting it to `255 <=> 0xFF` if it is equal to `0`.

`ctx.stack[ctx.sp] = ctx.stack[ctx.sp] > 0 ? ctx.stack[ctx.sp] - 1 : 255`

| Name | Value |
|------|-------|
| Brainfuck     | `-`   |
| Token         | `SUB` |
| Operation     | `SUB` |
| VM function   | `sub` |

#### LEFT

Decrements the stack pointer, or setting it to the stack length, in case it is equal to `0`.

`ctx.sp = ctx.sp > 0 ? ctx.sp - 1 : length(ctx.stack)`

| Name | Value |
|------|-------|
| Brainfuck     | `<`    |
| Token         | `LEFT` |
| Operation     | `LEFT` |
| VM function   | `left` |

#### RIGHT

Increments the stack pointer, or setting it to `0`, in case it is equal to the stack length.

`ctx.sp = ctx.sp < length(ctx.stack) ? ctx.sp + 1 : 0`

| Name | Value |
|------|-------|
| Brainfuck     | `>`    |
| Token         | `RIGHT` |
| Operation     | `RIGHT` |
| VM function   | `right` |

#### INPUT

Reads a raw byte from `STDIN` and writes it to the stack location value.

`ctx.stack[ctx.sp] = getchar(stdin)`

It somewhat matches the [ASCII](https://en.wikipedia.org/wiki/ASCII) specification.

| Name | Value |
|------|-------|
| Brainfuck     | `,`    |
| Token         | `INPUT` |
| Operation     | `INPUT` |
| VM function   | `input` |

#### OUTPUT

Writes a raw byte to `STDOUT`, reading it from the stack location value.

`putchar(STDOUT, ctx.stack[ctx.sp])`

The implementation uses the Rust [`print!`](https://doc.rust-lang.org/std/macro.print.html) macro.

It somewhat matches the [ASCII](https://en.wikipedia.org/wiki/ASCII) specification.

| Name | Value |
|------|-------|
| Brainfuck     | `.`    |
| Token         | `OUTPUT` |
| Operation     | `OUTPUT` |
| VM function   | `output` |

#### JZ

Short for '(j)ump to instruction if stack location value is equal to (z)ero'

`ctx.stack[ctx.sp] == 0 ? ctx.pc = ctx.program[ctx.pc].value`

Jumps to the matching `]`, skipping the `]`, because the program counter (`ctx.pc`) gets incremented.

| Name | Value |
|------|-------|
| Brainfuck     | `[`    |
| Token         | `BEGIN` |
| Operation     | `JZ` |
| VM function   | `jz` |

#### JMP

Short for 'jump'

`ctx.pc = ctx.program[ctx.pc].value`

Jumps to the instruction just before the matching `[` for the program counter (`ctx.pc`) then to be incremented, for the VM to then run the `[`.

| Name | Value |
|------|-------|
| Brainfuck     | `]`    |
| Token         | `END` |
| Operation     | `JMP` |
| VM function   | `jmp` |


