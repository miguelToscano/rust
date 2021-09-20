## Variables and mutability

Declare and immutable variable
```rust
let x = 5
```

Declares a mutable variable
```rust
let mut x = 5
```

Declares a constant (Rust’s naming convention for constants is to use all uppercase)
```rust
const MAX_VALUE: u32 = 5
```

## Shadowing

You can declare a new variable with the same name as a previous variable. Rustaceans say that the first variable is shadowed by the second, which means that the second variable’s value is what appears when the variable is used.

```rust
let x = 5;
let x = x + 1;
let x = x * 2;
```

Shadowing is different from marking a variable as mut, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

## Data Types
Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so it knows how to work with that data.

-  Integer type
 ```rust
let x: u32 = 1 // f32
 ```

- Floating point type
 ```rust
let x: f32 = 3.0; // f32
 ```

- Boolean type
 ```rust
let t = true;
let f: bool = false; // with explicit type annotation
 ```

- Character type
 ```rust
let c = 'z';
let z = 'ℤ';
let heart_eyed_cat = '😻';
 ```

- Tuple type
 ```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let (x, y, z) = tup; // DESTRUCTURING

let x: (i32, f64, u8) = (500, 6.4, 1);
let five_hundred = x.0;
let six_point_four = x.1;
let one = x.2;
 ```

- Array type
```rust
let a = [1, 2, 3, 4, 5];
let first = a[0];
let second = a[1];

let a: [i32; 5] = [1, 2, 3, 4, 5]; // Array of 5 i32 type elements
let a = [3; 5]; // Same as let a = [3, 3, 3, 3, 3]
```
