use anyhow::Result;
use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

use std::env;

use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
  x: u64,
  y: u64,
  z: u64,
}

impl Coordinate {
  pub fn distance(&self, c: &Coordinate) -> f64 {
    (((self.x - c.x).pow(2) + (self.y - c.y).pow(2) + (self.z - c.z).pow(2)) as f64).sqrt()
  }
}

impl From<&str> for Coordinate {
  fn from(value: &str) -> Self {
    problem::coordinate(value).expect("Invalid coordinate")
  }
}

peg::parser! {
  grammar problem() for str {
    pub rule coordinate() -> Coordinate
      = x:number() "," y:number() "," z:number() { Coordinate {x, y, z} }

    rule number() -> u64
      = n:$(['0'..='9']+) {? n.parse().or(Err("Cant parse f64")) }

    pub rule coordinate_list() -> Vec<Coordinate>
      = l:(coordinate() ** "\n") "\n"* { l }
  }
}

fn all_linked<T>(item: &T, links: &Vec<(&T, &T)>, linked: &mut HashSet<T>)
where
  T: Eq + Hash + Copy,
{
  for x in links
    .iter()
    .filter(|(a, b)| a == &item || b == &item)
    .flat_map(|(a, b)| vec![a, b])
  {
    if linked.get(x) == None {
      linked.insert(**x);
      all_linked(*x, links, linked);
    }
  }
}

fn main() -> Result<()> {
  let mut data = String::new();
  io::stdin().read_to_string(&mut data)?;
  let coordinates = problem::coordinate_list(&data)?;

  let args: Vec<String> = env::args().collect();
  let solution_part = args.get(1).map(|x| x.as_str()).unwrap_or("pt1");
  let max_connections = args
    .get(2)
    .map(|x| str::parse::<usize>(x).unwrap_or(10))
    .unwrap_or(10);
  let mut potential_links = coordinates
    .iter()
    .enumerate()
    .flat_map(|(ix, c1)| {
      coordinates
        .iter()
        .enumerate()
        .flat_map(|(jx, c2)| if jx > ix { vec![(c1, c2)] } else { vec![] })
        .collect::<Vec<_>>()
    })
    .collect::<Vec<_>>();

  potential_links.sort_by(|(p1, p2), (p3, p4)| f64::total_cmp(&p1.distance(p2), &p3.distance(p4)));

  if solution_part == "pt1" {
    let links: Vec<_> = potential_links.into_iter().take(max_connections).collect();

    let mut groups: Vec<HashSet<Coordinate>> = vec![];
    let mut belonging: HashMap<Coordinate, usize> = HashMap::new();

    for c in coordinates.iter() {
      let group_id = match belonging.get(c) {
        Some(id) => *id,
        None => {
          groups.push(HashSet::from_iter(vec![*c].into_iter()));
          let id = groups.len() - 1;
          id
        }
      };

      let mut link_set = HashSet::new();
      all_linked(c, &links, &mut link_set);

      for linked in link_set {
        // println!("{:?}, {}", linked, group_id);
        groups[group_id].insert(linked);
        belonging.insert(linked, group_id);
      }
    }
    let mut sizes = groups.iter().map(|g| g.len()).collect::<Vec<_>>();
    sizes.sort();
    println!("sizes {:?}", sizes);
    let solution = sizes.iter().rev().take(3).product::<usize>();

    println!("{}", solution);
  } else {
    let mut groups: Vec<HashSet<Coordinate>> = vec![];
    let mut belonging: HashMap<Coordinate, usize> = HashMap::new();

    let mut current_pair = potential_links[0];
    for (ix, (c1, c2)) in potential_links.iter().enumerate() {
      current_pair = (*c1, *c2);
      let current_ids = { (belonging.get(c1).map(|s| *s), belonging.get(c2).map(|s| *s)) };
      let current_len = match current_ids {
        (Some(id1), Some(id2)) if id1 == id2 => groups[id1].len(),
        (Some(id1), Some(id2)) => {
          let min_id = min(id1, id2);
          let max_id = max(id1, id2);
          groups[min_id] = groups[min_id].union(&groups[max_id]).map(|c| *c).collect();
          for item in groups[max_id].iter() {
            belonging.insert(*item, min_id);
          }
          //groups[max_id] = HashSet::new();
          groups[min_id].len()
        }
        (Some(id), _) => {
          groups[id].insert(**c2);
          belonging.insert(**c2, id);
          groups[id].len()
        }
        (_, Some(id)) => {
          groups[id].insert(**c1);
          belonging.insert(**c1, id);
          groups[id].len()
        }
        (None, None) => {
          let hs = HashSet::from_iter(vec![**c1, **c2].into_iter());
          let hslen = hs.len();
          groups.push(hs);
          let id = groups.len() - 1;
          belonging.insert(**c1, id);
          hslen
        }
      };
      println!("Got {} at link {}: {:?}", current_len, ix, current_pair);
      if current_len == coordinates.len() {
        break;
      }
    }

    println!("{:?}", current_pair.0.x * current_pair.1.x);
  }

  Ok(())
}
// 1000 too low
