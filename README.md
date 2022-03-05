# astr

A compile time known string

| Unsized | Const Length | Dynamic |
|--- | --- | --- |
| \[T\] (slice) | \[T; LEN\] (array) | Vec\<T\> |
| str | **AStr\<LEN\>** | String|

## Example

```rust
use astr::{AStr, astr};
// use the macro to infer the length of the string
let s = astr!("Hello World!");
assert_eq!(s, "Hello World!");

// also works in const context
const S1: &'static AStr<12> = astr!("Hello World!");
// the type is also copy and sized so you can derefernce it
const S2: AStr<12> = *astr!("Hello World!");
assert_eq!(S1, S2);

// use try_from to convert a String
let source_string = String::from("Hello World!");
let s2 = AStr::<12>::try_from(source_string).unwrap();
assert_eq!(s2, "Hello World!");
```
