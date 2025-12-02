use anyhow::Result;
use std::{env, io};

peg::parser! {
  grammar ranges_parser() for str {
    rule range() -> (u64, u64)
      = start:number() "-" end:number() { (start, end) }
    rule number() -> u64
      = n:$(['0'..='9']+) {? n.parse().or(Err("Cant parse u64")) }

    pub rule list() -> Vec<(u64, u64)>
      = l:(range() ** ",") { l }
  }
}

fn digits(num: u64) -> u64 {
  if num == 1 {
    return 1;
  }
  (num as f64).log(10.0f64).ceil() as u64
}

fn add_zeros(num: u64, zeroes: u32) -> u64 {
  num * 10u64.pow(zeroes)
}

fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();
  let solution_part = args.get(1).map(|x| x.as_str()).unwrap_or("pt1");

  let mut input = String::new();
  io::stdin().read_line(&mut input)?;
  input = input.trim().to_string();

  let ranges = ranges_parser::list(&input)?;

  let mut sum: u64 = 0;

  for (start, end) in ranges.iter() {
    for item in *start..=*end {
      let item_digits = digits(item);

      let piece_sizes = if solution_part == "pt1" {
        (item_digits / 2)..=(item_digits / 2)
      } else {
        1..=(item_digits / 2)
      };

      for piece_size in piece_sizes {
        if item_digits % piece_size == 0 {
          let last_piece = item % 10u64.pow(piece_size as u32);
          let mut reconstructed = 0;
          let mut reconstructions = item_digits / piece_size;
          while reconstructions > 0 {
            reconstructed = add_zeros(reconstructed, piece_size as u32) + last_piece;
            reconstructions -= 1;
          }
          if reconstructed == item {
            sum += reconstructed;
            break;
          }
        }
      }
    }
  }

  println!("{}", sum);

  Ok(())
}
