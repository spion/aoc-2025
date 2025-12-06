use anyhow::Result;
use std::io::{self, Read};

use std::env;

#[derive(Debug)]
enum Operator {
  Add,
  Mul,
}

impl From<&str> for Operator {
  fn from(value: &str) -> Self {
    match value {
      "+" => Self::Add,
      _ => Self::Mul, // cheating
    }
  }
}

peg::parser! {
  grammar problem() for str {

    rule _() = quiet!{[' ' | '\t']*}

    rule number() -> u64
      = n:$(['0'..='9']+) {? n.parse().or(Err("Cant parse u64")) }

    rule oper() -> Operator
      = op:$("*" / "+") { op.into() }


    rule number_row() -> Vec<u64>
      = _ l:(number() ++ (" " _)) _ { l }

    pub rule pt1() -> (Vec<Vec<u64>>, Vec<Operator>)
      = l:(number_row() ** ("\n")) "\n" _ o:(oper() ** (" "+)) _ "\n"* { (l, o) }


    rule pt2_problem() -> (Vec<u64>, Operator)
      = _ l:(number() ++ (" " _)) _ o:oper() { (l, o) }

    pub rule pt2() -> Vec<(Vec<u64>, Operator)>
      = l:(pt2_problem() ++ " ") _ { l }

  }
}

fn main() -> Result<()> {
  let mut data = String::new();
  io::stdin().read_to_string(&mut data)?;

  let args: Vec<String> = env::args().collect();
  let solution_part = args.get(1).map(|x| x.as_str()).unwrap_or("pt1");

  if solution_part == "pt1" {
    let (rows, opers) = problem::pt1(&data)?;
    let solution: u64 = (0..opers.len())
      .map(|k| {
        let items = rows.iter().map(|r| r[k]);
        let oper = &opers[k];
        match oper {
          Operator::Add => items.sum::<u64>(),
          Operator::Mul => items.product::<u64>(),
        }
      })
      .sum();
    println!("{}", solution);
  } else {
    let char_grid = data
      .split("\n")
      .map(|l| l.chars().collect::<Vec<_>>())
      .collect::<Vec<_>>();

    let lines = char_grid.len();
    let longest_len = char_grid.iter().max_by_key(|v| v.len()).unwrap().len();
    let fixed_data: String = (0..longest_len)
      .rev()
      .map(|col| {
        (0..lines)
          .map(|l| char_grid[l].get(col).unwrap_or(&' '))
          .collect::<String>()
      })
      .collect();

    let problems = problem::pt2(&fixed_data)?;
    let solution: u64 = problems
      .into_iter()
      .map(|(items, oper)| match oper {
        Operator::Add => items.iter().sum::<u64>(),
        Operator::Mul => items.iter().product::<u64>(),
      })
      .sum();

    println!("{}", solution);
  }

  Ok(())
}
