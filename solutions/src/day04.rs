use anyhow::Result;
use std::io;

#[derive(PartialEq)]
enum MapPoint {
  Empty,
  ToiletRoll,
}

impl From<char> for MapPoint {
  fn from(value: char) -> Self {
    if value == '@' {
      Self::ToiletRoll
    } else {
      Self::Empty
    }
  }
}

type SolutionMap = Vec<Vec<MapPoint>>;

fn count_toilet(point: &MapPoint) -> usize {
  match point {
    MapPoint::Empty => 0,
    MapPoint::ToiletRoll => 1,
  }
}

fn count_near_toilet_rolls(ix: usize, iy: usize, map: &SolutionMap) -> usize {
  (-1i32..=1i32)
    .flat_map(|x| (-1i32..=1i32).map(move |y| (x, y)))
    .filter(|(x, y)| *x != 0 || *y != 0)
    .map(|(x, y)| (ix as i32 + x, iy as i32 + y))
    // .inspect(|(x, y)| println!("p0 {} {}", x, y))
    .filter(|(x, y)| {
      *x >= 0 && *y >= 0 && *x < map.len() as i32 && *y < map[*x as usize].len() as i32
    })
    .map(|(x, y)| count_toilet(&map[x as usize][y as usize]))
    .sum()
}

fn main() -> Result<()> {
  let mut toilet_grid = io::stdin()
    .lines()
    .map(|l| l.map(|l| l.chars().map(|c| c.into()).collect::<Vec<MapPoint>>()))
    .collect::<Result<Vec<_>, io::Error>>()?;

  let mut sum = 0;

  while {
    let mut removals: Vec<(usize, usize)> = vec![];
    for (i, line) in toilet_grid.iter().enumerate() {
      for (j, item) in line.iter().enumerate() {
        if *item == MapPoint::ToiletRoll && count_near_toilet_rolls(i, j, &toilet_grid) < 4 {
          removals.push((i, j));
          sum += 1;
        }
      }
    }

    for (x, y) in removals.iter() {
      toilet_grid[*x][*y] = MapPoint::Empty;
    }

    removals.len() > 0
  } {}

  println!("{}", sum);

  Ok(())
}

//pt2 173065197311341 too low
//pt2 173065202451341
