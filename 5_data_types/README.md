# Data Types

There are two kinds of data types in Rust:
- Scalar types: Represent a single value.
- Compound types: Group multiple values together.

## Scalar Types

Here we can find:

### Numbers

#### Integers

```
u8, i8,
u16, i16,
u32, i32,
u64, i64,
u128, i128,
isize, usize
```

- Integers are numbers without a fractional part.
- The prefix `u` means unsigned, and `i` means signed.
- The suffix represent the size in bits (e.g. `u32` means unsigned 32-bit
integer), and ``size`` means the the size of the architecture of the computer
the program is running on (usually used for indexing some sort of collection).
- Can be written in many ways:

```
1, 1_000, 1_000_000 // DECIMAL
0x1, 0x1_000, 0x1_000_000 // HEXADECIMAL
0o1, 0o1_000, 0o1_000_000 // OCTAL
0b1, 0b1_000, 0b1_000_000 // BINARY
b'A', b'\x61' // BYTE (for u8)
```

#### Floating-Point Types

```
f32, f64
```

- Floats are numbers with decimal points.
- Floats are represented in the IEEE-754 floating-point standard.

```rust
let sum = 5 + 10;
let difference = 95.5 - 4.3;
let product = 4 * 30;
let quotient = 56.7 / 32.2;
let floored = 2 / 3;

let remainder = 43 % 5;
```

### Boolean

```
bool
```

- Boolean, as always, are either `true` or `false`.

### Character

```
char
```

- A character.
- Is specified with single quotes (`'a'`).
- Represents a Unicode Scalar Value. i.e. Accepts more than just ASCII.

## Compound Types

### Tuple

```
(<type_a>, <type_b>, <type_c>)
```

- A tuple is a sequence of values separated by commas.
- Have a fixed length.
- The tuple without any values, `()` is a special type that has only one value,
also written `()`. The type is called the unit type and the value is called the
unit value. Expressions implicitly return the unit value if they don’t return
any other value.

### Array
    
```
[<type_a>; <size>]
```

- An array is a sequence of values of a single type.
- Have a fixed length.
- Useful when you want your data allocated on the stack rather than the heap.
- When accessing an element by index, if the index is out of bounds, the program
will panic.
