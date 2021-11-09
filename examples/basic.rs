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
