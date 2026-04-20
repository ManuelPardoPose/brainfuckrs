use brainfuckrs::{interpreter::Simulator, test_programs::HELLO_WORLD_PROGRAM1, tokenizer::tokenize};

fn main() {
    let tokens = tokenize(HELLO_WORLD_PROGRAM1.to_string());
    let mut simulator = Simulator::new(tokens.clone());
    simulator.simulate();
}
