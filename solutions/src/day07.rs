use anyhow::Result;
use itertools::Itertools;
use std::io;

use std::env;

#[derive(Debug, PartialEq)]
enum Point {
  Space,
  Splitter,
  Start,
}

impl From<char> for Point {
  fn from(value: char) -> Self {
    match value {
      '^' => Self::Splitter,
      'S' => Self::Start,
      _ => Self::Space, // cheating
    }
  }
}

fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();
  let solution_part = args.get(1).map(|x| x.as_str()).unwrap_or("pt1");

  if solution_part == "pt1" {
    let mut solution = 0;
    let mut current_beams: Vec<usize> = vec![];
    for line in io::stdin().lines() {
      let pts: Vec<Point> = line?.chars().map(|c| c.into()).collect();
      if current_beams.len() == 0 {
        println!("Initializing");
        current_beams = pts
          .iter()
          .enumerate()
          .filter(|(_, pt)| **pt == Point::Start)
          .map(|(ix, _)| ix)
          .collect();

        println!("Beaming {:?}", current_beams);
      } else {
        let (split_points, continued_beams): (Vec<usize>, Vec<usize>) = current_beams
          .into_iter()
          .partition(|ix| pts[*ix] == Point::Splitter);

        solution += split_points.len();

        current_beams = split_points
          .iter()
          .flat_map(|sp| vec![sp - 1, sp + 1])
          .filter(|sp| *sp > 0 && *sp < pts.len())
          .chain(continued_beams.into_iter())
          .unique()
          .collect();
      }
    }
    println!("{}", solution);
  } else {
    let mut beam_counts = vec![];
    for line in io::stdin().lines() {
      let pts: Vec<Point> = line?.chars().map(|c| c.into()).collect();
      if beam_counts.len() == 0 {
        beam_counts = vec![0; pts.len()];
        println!("Initializing");
        for ix in pts
          .iter()
          .enumerate()
          .filter(|(_, pt)| **pt == Point::Start)
          .map(|(ix, _)| ix)
        {
          beam_counts[ix] = 1;
        }
      } else {
        let adders = {
          beam_counts
            .iter()
            .zip(pts.into_iter())
            .enumerate()
            .flat_map(|(ix, (bc, pt))| {
              if pt == Point::Splitter {
                vec![(ix - 1, *bc), (ix + 1, *bc), (ix, 0 - *bc)]
              } else {
                vec![]
              }
            })
            .collect::<Vec<_>>()
        };

        for (ix, c) in adders.into_iter() {
          beam_counts[ix] += c
        }
      }
      //println!("Beaming next {:?}", beam_counts);
    }
    println!("{}", beam_counts.iter().sum::<usize>());
  }

  Ok(())
}
