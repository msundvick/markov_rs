# markov_rs

A simple Markov chain generator in Rust.

By using Walker's Alias Method, a weighted random sampling algorithm, the model can generate elements very quickly.

## Usage

Add this to your Cargo.toml:

```toml
[dependencies]
markov_rs = "0.1"
```

## Example

```rust
use markov_rs::MarkovChain;

fn main() {
    let text = vec![
        "I", "think", "that", "that", "that", "that", "that", "boy", "wrote", "is", "wrong",
    ];

    let mut model = MarkovChain::from(&text);

    for _ in 0..20 {
        print!("{} ", model.next());
    }
}
```

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
