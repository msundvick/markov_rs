use markov_rs::MarkovChain;

use once_cell::sync::Lazy;

use criterion::Criterion;
use criterion::{criterion_group, criterion_main};

const LIST: Lazy<Vec<i32>> = Lazy::new(|| {
    let mut rng = rand::thread_rng();
    (0..10000)
        .map(|_| rng.gen_range(0..1000))
        .collect::<Vec<i32>>()
        .to_vec()
});

fn bench_wam_constructor(c: &mut Criterion) {
    let vector = LIST.to_vec();
    c.bench_function("wam_constructor", |b| b.iter(|| MarkovChain::from(&vector)));
}

fn bench_cdf_constructor(c: &mut Criterion) {
    let vector = LIST.to_vec();
    c.bench_function("cdf_constructor", |b| b.iter(|| MarkovCSM::from(&vector)));
}

fn bench_generate_element_by_wam(c: &mut Criterion) {
    let mut model = MarkovChain::from(&LIST.to_vec());

    let mut result = [0; 100_000];
    let mut rng = rand::thread_rng();

    c.bench_function("generate_element_by_wam", |b| {
        b.iter(|| {
            for r in &mut result {
                *r = *model.next_rng(&mut rng);
            }
        })
    });
}

fn bench_generate_element_by_cdf(c: &mut Criterion) {
    let mut model = MarkovCSM::from(&LIST.to_vec());

    let mut result = [0; 100_000];
    let mut rng = rand::thread_rng();

    c.bench_function("generate_element_by_cdf", |b| {
        b.iter(|| {
            for r in &mut result {
                *r = *model.next_rng(&mut rng);
            }
        })
    });
}

criterion_group!(
    benches,
    bench_wam_constructor,
    bench_cdf_constructor,
    bench_generate_element_by_wam,
    bench_generate_element_by_cdf,
);
criterion_main!(benches);

// ========================================================= //

// Markov chain with Cumulative Distribution Function

use rand::prelude::*;

#[derive(Debug, PartialEq)]
pub struct MarkovCSM<T> {
    elements: Vec<T>,
    cm_dist: Vec<Vec<f32>>,
    pre_index: usize,
}

impl<T> MarkovCSM<T>
where
    T: Clone,
    T: Eq,
    T: Ord,
    T: PartialOrd,
    T: PartialEq,
{
    fn new(elements: Vec<T>, cm_dist: Vec<Vec<f32>>, pre_index: usize) -> MarkovCSM<T> {
        MarkovCSM {
            elements: elements,
            cm_dist: cm_dist,
            pre_index: pre_index,
        }
    }

    pub fn from(elements: &Vec<T>) -> MarkovCSM<T> {
        let mut non_dup_elements = elements.clone();
        non_dup_elements.sort();
        non_dup_elements.dedup();

        let elements_len = non_dup_elements.len();

        let mut state_freq = vec![vec![0; elements_len]; elements_len];
        let mut pre_index: Option<usize> = None;
        for token in elements {
            let cur_index = non_dup_elements
                .iter()
                .position(|t| *token == *t)
                .expect("There is no token that should exist.");
            if let Some(i) = pre_index {
                state_freq[i][cur_index] += 1;
            }
            pre_index = Some(cur_index);
        }

        let mut cm_dist = vec![vec![0.0; elements_len]; elements_len];
        for (i, vector) in state_freq.iter().enumerate() {
            let row_sum = vector.iter().fold(0, |acc, cur| acc + cur);
            let mut cumulative_p = 0.0;
            for (j, count) in vector.iter().enumerate() {
                if row_sum != 0 {
                    cumulative_p = cumulative_p + (*count as f32 / row_sum as f32);
                    cm_dist[i][j] = cumulative_p;
                }
            }
        }

        MarkovCSM::new(non_dup_elements, cm_dist, elements_len)
    }

    pub fn next(&mut self) -> &T {
        let mut rng = rand::thread_rng();
        self.next_rng(&mut rng)
    }

    pub fn next_rng(&mut self, rng: &mut ThreadRng) -> &T {
        let row_index = {
            let mut i;
            loop {
                if self.pre_index != self.elements.len() {
                    i = self.pre_index;
                } else {
                    i = rng.gen_range(0..self.elements.len());
                }
                let row_sum = self.cm_dist[i].iter().fold(0.0, |acc, cur| acc + cur);
                if row_sum == 0.0 {
                    self.initialize();
                } else {
                    break;
                }
            }
            i
        };

        let f = rng.gen::<f32>();
        let cur_index: usize = {
            let mut res = self.cm_dist[row_index].len() - 1;
            for (i, p) in self.cm_dist[row_index].iter().enumerate() {
                if f <= *p {
                    res = i;
                    break;
                }
            }
            res
        };

        self.pre_index = cur_index;
        &self.elements[cur_index]
    }

    pub fn initialize(&mut self) {
        self.pre_index = self.elements.len();
    }
}
