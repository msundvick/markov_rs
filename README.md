# markov_rs

[![markov_rs](https://github.com/ichi-h/markov_rs/actions/workflows/markov_rs.yml/badge.svg)](https://github.com/ichi-h/markov_rs/actions/workflows/markov_rs.yml)
[![Crates.io](https://img.shields.io/crates/v/markov_rs)](https://crates.io/crates/markov_rs)
[![docs.rs](https://img.shields.io/docsrs/markov_rs)](https://docs.rs/markov_rs)
[![Crates.io](https://img.shields.io/crates/l/markov_rs)](LICENSE-APACHE)

A simple and fast Markov chain generator in Rust.

By using the Walker's Alias Method, a weighted random sampling algorithm, the model can generate elements very quickly. The benchmark for this crate and the Markov chain using the Cumulative Sum Method is as follows.

|       Algorithm       | Building model | Generation |
| :-------------------: | :------------: | :--------: |
| Walker's Alias Method |   117.15 ms    | 9.2078 ms  |
| Cumulative Sum Method |   8.3087 ms    | 168.02 ms  |

- Building model
  - The time to create a Markov chain model from a stored array of random values from 0 to 999 with a length of 10000.
- Generation
  - The time to generate 100,000 elements from the above model.
- Environment
  - OS: Windows 11 Home Insider Preview
  - CPU: Intel(R) Core(TM) i7-6700HQ CPU @ 2.60GHz
  - RAM: 16.0 GB
  - Rust version: rustc 1.56.1
  - Source code: [benchmark.rs](benches/benchmark.rs)

For details about the Walker's Alias Method, see [ichi-h / weighted_rand](https://github.com/ichi-h/weighted_rand).

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
    let text = [
        "I", "think", "that", "that", "that", "that", "that", "boy", "wrote", "is", "wrong",
    ];
    let mut model = MarkovChain::from(&text);
    for _ in 0..20 {
        print!("{} ", model.next());
    }
}
```

## Planned

- Improve performance of model building.
- Support for Nth-order Markov chain.

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
