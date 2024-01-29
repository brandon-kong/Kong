# konglang

A simple programming language written in Rust.

## Usage

```bash
cargo run --release
```

### Syntax

#### Variables

```kong
let x = 1;
let y = 2;
let z = x + y;
```

You can also declare variables without initializing them.

```kong
let x: int;
```

#### Types

Kong supports the following primitive types:

```kong
let x: int = 1;
let y: float = 1.0;
let z: bool = true;
let w: string = "hello";
```

Variables can be declared without a type, and the compiler will infer the type.

```kong
let x = 1;
```

In the above example, `x` will be inferred to be an `int`.

#### Control Flow

##### If

```kong
let x = 1;

if x == 1 {
  print("x is 1");
} else if x == 2 {
  print("x is 2");
} else {
  print("x is neither 1 nor 2");
}
```

##### Pattern matching

```kong

let x = 1;

match x {
  1: print("x is 1"),
  2: print("x is 2"),
  _: print("x is neither 1 nor 2"),
}
```

##### Loops

```kong

let x = 1;

while x < 10 {
  print(x);
  x = x + 1;
}

for i in 0..10 {
  print(i);
}
```

#### Functions

```kong
global fn add(x: int, y: int) -> int {
  return x + y;
}

let z = add(1, 2);
```

#### Structs

```kong

struct Point {
  x: int,
  y: int,
}

let p = Point { x: 1, y: 2 };
```

#### Enums

```kong

enum Option<T> {
  Some(T),
  None,
}

let x = Option::Some(1);
```

#### Modules

```kong

mod foo {
  fn bar() {
    print("bar");
  }
}

foo::bar();
```

#### Imports

```kong

import foo::bar;

bar();
```

#### Comments

```kong

// This is a comment
```

#### Printing

```kong

print("hello");
```

#### Errors

```kong

error("Something went wrong");
```

#### Casting

```kong

let x: int = 1;

let y: float = x as float;
```