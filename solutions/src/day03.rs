use anyhow::Result;
use std::{env, io};

fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();
  let solution_part = args.get(1).map(|x| x.as_str()).unwrap_or("pt1");

  let mut sum = 0;
  for line in io::stdin().lines() {
    let bank = line?;

    let (first_ix, first_char) = bank
      .chars()
      .enumerate()
      .max_by_key(|(ix, char)| {
        if *ix < bank.len() - 1 {
          (*char, 0 - *ix)
        } else {
          ('0', 0 - *ix)
        }
      })
      .unwrap();
    let second_char = bank.chars().skip(first_ix + 1).max().unwrap();

    let val = (first_char as u64 - '0' as u64) * 10 + (second_char as u64 - '0' as u64);

    println!(
      "{};{}; {}; {} ;;; {}",
      bank.chars().max().unwrap(),
      first_char,
      first_ix,
      bank,
      val
    );
    sum += val;
  }
  println!("{}", sum);

  Ok(())
}

// 17254 too low
