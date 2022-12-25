# Rust Learning
Simple repo to tackle understanding rust



## Rust Book

*Questions*

Section 1.2: So what are differences between functions and macros?
`println!()` versus `println()`

Section 2.0: so how does rand work with thread_rng
`let secret_number = rand::thread_rng().gen_range(1..=100);` 

Section 2.0: why does match on guess not need error catching for compiler `match guess.cmp(&secret_number)`