use std::{env, fs};

fn main() {
  // read lines from first argument file
  let args: Vec<String> = env::args().collect();
  let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
  let lines = contents.lines();
}
