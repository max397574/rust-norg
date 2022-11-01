mod parser;
mod tokenizer;

fn main() {
    let x = tokenizer::tokenize("*Hello* {world}!");
    println!("Output is {:#?}", x);
}
