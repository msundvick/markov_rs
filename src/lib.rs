//! # markov_rs
//!
//! A simple and fast Markov chain generator in Rust.
//!
//! By using Walker's Alias Method, a weighted random sampling
//! algorithm, the model can generate elements very quickly.
//!
//! ## Example
//!
//! ```rust
//! use markov_rs::MarkovChain;
//!
//! fn main() {
//!     let text = [
//!         "I", "think", "that", "that", "that", "that", "that", "boy", "wrote", "is", "wrong",
//!     ];
//!     let mut model = MarkovChain::from(&text);
//!     for _ in 0..20 {
//!         print!("{} ", model.next());
//!     }
//! }
//! ```
//!

use rand::prelude::*;
use serde::{Deserialize, Serialize};
use weighted_rand::builder::*;
use weighted_rand::table::WalkerTable;

/// Markov model structure
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct MarkovChain<T> {
    /// The set of possible states of the model.
    state_space: Vec<T>,

    /// The transition probability table by Walker's Alias Method.
    wa_table: Vec<WalkerTable>,

    /// The index of the state generated by the previous
    /// [`next()`](#method.next) method. The initial value is the
    /// length of `state_space`.
    prev_index: usize,
}

impl<T> MarkovChain<T>
where
    T: Clone,
    T: Eq,
    T: Ord,
    T: PartialOrd,
    T: PartialEq,
{
    /// Creates a new instance of [`MarkovChain`].
    fn new(state_space: Vec<T>, wa_table: Vec<WalkerTable>, prev_index: usize) -> MarkovChain<T> {
        MarkovChain {
            state_space: state_space,
            wa_table: wa_table,
            prev_index: prev_index,
        }
    }

    /// Builds a new model from [`&[T]`].
    ///
    /// `T` must implement [`Clone`], [`Eq`], [`Ord`], [`PartialOrd`]
    /// and [`PartialEq`] traits.
    pub fn from(elements: &[T]) -> MarkovChain<T> {
        let mut state_space = elements.to_vec();
        state_space.sort();
        state_space.dedup();

        let space_len = state_space.len();

        let mut freq_table = vec![vec![0; space_len]; space_len];
        let mut prev_index: Option<usize> = None;
        for element in elements {
            let cur_index = state_space
                .iter()
                .position(|state| *element == *state)
                .expect("There is no state that should exist.");
            if let Some(i) = prev_index {
                freq_table[i][cur_index] += 1;
            }
            prev_index = Some(cur_index);
        }

        let mut wa_table = Vec::with_capacity(space_len);
        for row in freq_table {
            let builder = WalkerTableBuilder::new(&row);
            wa_table.push(builder.build());
        }

        MarkovChain::new(state_space, wa_table, space_len)
    }

    /// Returns a next possible state.
    ///
    /// The first state will be determined randomly, and the next
    /// one will be chosen by its state space.
    ///
    /// If you want to initialize the chain of states, use
    /// [`initialize()`](#method.initialize) methods.
    pub fn next(&mut self) -> &T {
        let mut rng = rand::thread_rng();
        self.next_rng(&mut rng)
    }

    /// Returns a next possible state using an external [`ThreadRng`].
    pub fn next_rng<R: Rng>(&mut self, rng: &mut R) -> &T {
        let row = {
            if self.prev_index == self.state_space.len() {
                self.prev_index = rng.gen_range(0..self.state_space.len());
            }
            self.prev_index
        };
        let elem_index = self.wa_table[row].next_rng(rng);

        self.prev_index = elem_index;
        &self.state_space[elem_index]
    }

    /// Initializes `prev_index` with the length of `state_space`.
    pub fn initialize(&mut self) {
        self.prev_index = self.state_space.len();
    }
}

#[cfg(test)]
mod markov_test {
    use crate::MarkovChain;
    use weighted_rand::table::WalkerTable;

    const TEXT: [&str; 11] = [
        "I", "think", "that", "that", "that", "that", "that", "boy", "wrote", "is", "wrong",
    ];

    #[test]
    fn make_markov_model() {
        let actual = MarkovChain::from(&TEXT);

        let expected = MarkovChain {
            state_space: vec!["I", "boy", "is", "that", "think", "wrong", "wrote"],
            wa_table: vec![
                WalkerTable::new(
                    vec![4, 4, 4, 4, 4, 4, 4],
                    vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
                ),
                WalkerTable::new(
                    vec![6, 6, 6, 6, 6, 6, 6],
                    vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
                ),
                WalkerTable::new(
                    vec![5, 5, 5, 5, 5, 5, 5],
                    vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
                ),
                WalkerTable::new(
                    vec![3, 1, 3, 1, 3, 3, 3],
                    vec![1.0, 1.0, 1.0, 0.4, 1.0, 1.0, 1.0],
                ),
                WalkerTable::new(
                    vec![3, 3, 3, 3, 3, 3, 3],
                    vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
                ),
                WalkerTable::new(
                    vec![0, 0, 0, 0, 0, 0, 0],
                    vec![0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
                ),
                WalkerTable::new(
                    vec![2, 2, 2, 2, 2, 2, 2],
                    vec![1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0],
                ),
            ],
            prev_index: 7,
        };

        assert_eq!(actual, expected)
    }

    #[test]
    fn generate_element() {
        let mut model = MarkovChain::from(&TEXT);
        let element = model.next();

        let include = TEXT
            .iter()
            .fold(false, |acc, cur| if acc { acc } else { element == cur });

        assert!(include)
    }

    #[test]
    fn initialize() {
        let mut model = MarkovChain::from(&TEXT);

        model.next();
        let before = model.prev_index;
        model.initialize();
        let after = model.prev_index;

        assert!(before != after);
        assert_eq!(after, 7);
    }
}
