use anyhow::Result;
use std::cmp::max;
use std::io::{self, Read};

use std::env;

peg::parser! {
  grammar ranges_parser() for str {
    rule range() -> (u64, u64)
      = start:number() "-" end:number() { (start, end) }

    rule number() -> u64
      = n:$(['0'..='9']+) {? n.parse().or(Err("Cant parse u64")) }

    rule range_list() -> Vec<(u64, u64)>
      = l:(range() ** "\n") { l }

    rule number_list() -> Vec<u64>
      = l:(number() ** "\n") { l }

    pub rule data() -> (Vec<(u64, u64)>, Vec<u64>)
      = rl:range_list() "\n"+ nl:number_list() "\n"* { (rl, nl) }
  }
}

fn main() -> Result<()> {
  let mut data = String::new();
  io::stdin().read_to_string(&mut data)?;
  let (ranges, items) = ranges_parser::data(&data)?;

  let args: Vec<String> = env::args().collect();
  let solution_part = args.get(1).map(|x| x.as_str()).unwrap_or("pt1");

  if solution_part == "pt1" {
    let count = items
      .into_iter()
      .filter(|i| ranges.iter().any(|(start, end)| i >= start && i <= end))
      .count();

    println!("{}", count);
  } else {
    let mut ranges = ranges.clone();
    ranges.sort();

    let mut count = 0u64;
    let mut accounted = 0u64;
    for (start, end) in ranges {
      if end < accounted {
        continue;
      }
      count += end - max(start, accounted) + 1;
      accounted = end + 1;
    }

    println!("{}", count);
  }

  Ok(())
}
// 334877939080182 too low
