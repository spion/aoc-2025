use anyhow::{Result, anyhow};
use itertools::Itertools;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::io;

use rayon::prelude::*;
use std::env;
use std::ops::RangeInclusive;

#[derive(Debug)]
struct Coord {
  x: i64,
  y: i64,
}

impl TryFrom<&str> for Coord {
  type Error = anyhow::Error;

  fn try_from(value: &str) -> Result<Self> {
    let (x, y) = value.split_once(",").ok_or(anyhow!("Invalid coordinate"))?;
    let c = Coord {
      x: x.parse()?,
      y: y.parse()?,
    };
    Ok(c)
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Fill {
  None,
  Filled,
  Empty,
}

fn flood(a: &mut Vec<Vec<Fill>>, x: usize, y: usize, f: Fill) {
  if y >= a.len() {
    return;
  }
  if x >= a[y].len() {
    return;
  }
  if a[y][x] == Fill::None {
    a[y][x] = f;
    flood(a, x + 1, y, f);
    flood(a, x, y + 1, f);
    if x > 0 {
      flood(a, x - 1, y, f);
    }
    if y > 0 {
      flood(a, x, y - 1, f);
    }
  }
}

// fn show(a: &Vec<Vec<Fill>>) {
//   for y in 0..a.len() {
//     for x in 0..a[y].len() {
//       let c = match a[y][x] {
//         Fill::Filled => 'x',
//         Fill::None => 'O',
//         Fill::Empty => '.',
//       };
//       print!("{}", c);
//     }
//     println!("");
//   }
// }

fn rng<T: Ord + Copy>(one: T, two: T) -> RangeInclusive<T> {
  min(one, two)..=max(one, two)
}

fn main() -> Result<()> {
  let args: Vec<String> = env::args().collect();
  let solution_part = args.get(1).map(|x| x.as_str()).unwrap_or("pt1");

  let coords =
    io::stdin().lines().map(|line| line?.as_str().try_into()).collect::<Result<Vec<Coord>>>()?;

  if solution_part == "pt1" {
    let area_max = coords
      .iter()
      .enumerate()
      .flat_map(|(i, c1)| coords.iter().skip(i + 1).map(|c2| (c1, c2)).collect::<Vec<_>>())
      .map(|(c1, c2)| ((c1.x - c2.x).abs() + 1) * ((c1.y - c2.y).abs() + 1))
      .max()
      .ok_or(anyhow!("No coordinates found"))?;

    println!("{}", area_max);
  } else {
    // Lets make a tiny map from the big map!
    // Every unique x of a red tile is a new position in x coordinates
    // Similarly, every unique y is a new position in y coordinates
    // We offset by 1 to leave spaces around the border of the new map.
    let tiny_x = coords
      .iter()
      .map(|c| c.x)
      .unique()
      .sorted()
      .enumerate()
      .map(|(ix, x)| (x, ix + 1))
      .collect::<HashMap<_, _>>();
    let tiny_y = coords
      .iter()
      .map(|c| c.y)
      .unique()
      .sorted()
      .enumerate()
      .map(|(ix, x)| (x, ix + 1))
      .collect::<HashMap<_, _>>();

    let max_x = tiny_x.values().max().unwrap();
    let max_y = tiny_y.values().max().unwrap();

    // Translate the coordinates to the tiny coordinate system
    let tiny_coords = coords
      .iter()
      .map(|c| (tiny_x.get(&c.x).unwrap(), tiny_y.get(&c.y).unwrap()))
      .collect::<Vec<_>>();

    // Make a cute tiny map!
    // In my testing this turns out to be around 250x250, no issues with that size
    let mut tiny_map =
      (0..*max_y + 2).map(|_| vec![Fill::None; *max_x as usize + 2]).collect::<Vec<_>>();

    // Prepare to draw the tiny lines.
    let lines =
      tiny_coords.iter().zip(tiny_coords.iter().skip(1).chain(tiny_coords.iter().take(1)));

    // Fill the lines
    for ((sx, sy), (ex, ey)) in lines {
      if sx == ex {
        for y in rng(**sy, **ey) {
          tiny_map[y][**sx] = Fill::Filled
        }
      } else if sy == ey {
        for x in rng(**sx, **ex) {
          tiny_map[**sy][x] = Fill::Filled
        }
      } else {
        // We assume all lines are along the x axis or the y axis, panic if not
        panic!(
          "Unexpected coordinate alignment {} {}; {} {}",
          sx, sy, ex, ey
        );
      }
    }
    //show(&tiny_map);

    // Flood-fill from the top left corner with emptyness. The flood stops at the drawn borders
    // Thanks to our slightly larger map, it should be able to go around the edges and reach every
    // bit of empty space.
    flood(&mut tiny_map, 0, 0, Fill::Empty);
    //show(&tiny_map);

    // For every possible pair of red coordinates
    let area_max = coords
      .par_iter()
      .enumerate()
      .flat_map(|(i, c1)| coords.iter().skip(i + 1).map(|c2| (c1, c2)).collect::<Vec<_>>())
      .filter(|(c1, c2)| {
        // can we use this pair? translate to tiny coordinate system
        let tx1 = tiny_x.get(&c1.x).unwrap();
        let tx2 = tiny_x.get(&c2.x).unwrap();
        let ty1 = tiny_y.get(&c1.y).unwrap();
        let ty2 = tiny_y.get(&c2.y).unwrap();

        // then check if the entire area of this rectangle is non-empty
        // if a single tile was reached by the flood-fill, early return.
        for y in rng(*ty1, *ty2) {
          for x in rng(*tx1, *tx2) {
            if tiny_map[y][x] == Fill::Empty {
              return false;
            }
          }
        }
        return true;
      })
      //.inspect(|cs| println!("{:?}", cs))
      .map(|(c1, c2)| ((c1.x - c2.x).abs() + 1) * ((c1.y - c2.y).abs() + 1))
      .max()
      .ok_or(anyhow!("No coordinates found"))?;

    println!("{:?}", area_max);
  }

  Ok(())
}
// 1613269140 too low
// 1613305596
