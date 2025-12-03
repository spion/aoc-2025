use anyhow::Result;
use std::{env, io};

fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();

  let mut sum = 0;
  for line in io::stdin().lines() {
    let bank = line?;

    let mut skip_point = 0;

    let mut val = 0;
    for dig in (0..12).rev() {
      let (ix, char) = bank
        .chars()
        .enumerate()
        .skip(skip_point)
        .max_by_key(|(ix, char)| {
          if *ix < bank.len() - dig {
            (*char, 0 - *ix as i64)
          } else {
            ('0', 0 - *ix as i64)
          }
        })
        .unwrap();

      skip_point = ix + 1;
      val = (val * 10) + (char as u64 - '0' as u64);
    }

    println!("{} ;;; {}", bank, val);
    sum += val;
  }
  println!("{}", sum);

  Ok(())
}

//pt2 173065197311341 too low
//pt2 173065202451341
