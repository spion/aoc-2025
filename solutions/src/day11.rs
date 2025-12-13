use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::io;

peg::parser! {
  grammar problem() for str {

    rule _() = quiet!{[' ' | '\n' | '\t']+}
    rule node() -> String
      = n:$(['a'..='z']+) { n.to_string() }

    pub rule line() -> (String, Vec<String>)
      = from:node() ":" _ to:(node() ** _) { (from, to) }
  }
}

fn dfs<T, F>(start: &T, goal: &T, path: &mut Vec<T>, moves_fn: &F) -> usize
where
  F: Fn(&T) -> Vec<T>,
  T: Eq + Hash + Clone,
{
  if start == goal {
    return 1;
  }
  let mut counter = 0;
  let moves_all = moves_fn(start);

  for m in moves_all.into_iter() {
    if path.contains(&m) {
      continue;
    }
    path.push(m.clone());
    counter += dfs(&m, goal, path, moves_fn);
    path.pop();
  }
  return counter;
}

#[derive(Debug, Clone)]
struct PathCounters {
  item: String,
  outs: u64,
  dacs: u64,
  ffts: u64,
  both: u64,
}

impl PathCounters {
  fn start(item: &str) -> PathCounters {
    PathCounters {
      item: item.to_string(),
      outs: 1,
      dacs: 0,
      ffts: 0,
      both: 0,
    }
  }

  fn other(item: &str) -> PathCounters {
    PathCounters {
      item: item.to_string(),
      outs: 0,
      dacs: 0,
      ffts: 0,
      both: 0,
    }
  }

  fn absorb(&mut self, other: &PathCounters) {
    self.outs += other.outs;
    self.ffts += if other.item == "fft" {
      other.outs
    } else {
      other.ffts
    };
    self.dacs += if other.item == "dac" {
      other.outs
    } else {
      other.dacs
    };
    let add_boths = if other.item == "fft" {
      other.dacs
    } else if other.item == "dac" {
      other.ffts
    } else {
      other.both
    };
    self.both += add_boths;
  }
}

struct TriColor {
  map: HashMap<String, Vec<String>>,
  path: Vec<String>,
  black: HashSet<String>,
  pcs: HashMap<String, PathCounters>,
}

impl TriColor {
  fn new(map: HashMap<String, Vec<String>>) -> TriColor {
    let mut pcs =
      map.keys().map(|k| (k.clone(), PathCounters::other(k))).collect::<HashMap<_, _>>();

    pcs.insert("out".to_string(), PathCounters::start("out"));
    let black: HashSet<String> = HashSet::new();
    let path: Vec<String> = vec![];
    Self {
      map,
      path,
      black,
      pcs,
    }
  }

  fn dfs(&mut self, node: &String) {
    let moves = self.map.get(node).unwrap_or(&vec![]).clone();

    for m in moves.iter() {
      if self.black.contains(m) {
        let mut pnode = self.pcs.get(node).unwrap().clone();
        let mnode = self.pcs.get(m).unwrap();
        pnode.absorb(mnode);
        self.pcs.insert(node.clone(), pnode);
        continue;
      }

      self.path.push(m.clone());
      self.dfs(m);
      self.path.pop();
      self.black.insert(m.clone());

      let mut pnode = self.pcs.get(node).unwrap().clone();
      let mnode = self.pcs.get(m).expect(&format!("Not finding {}", m));
      pnode.absorb(mnode);
      self.pcs.insert(node.clone(), pnode);
    }
  }

  fn run(&mut self) -> PathCounters {
    self.dfs(&"svr".to_string());
    return self.pcs.get("svr").unwrap().clone();
  }
}

fn main() -> Result<()> {
  let mut map: HashMap<String, Vec<String>> = HashMap::new();

  for line in io::stdin().lines() {
    let data = line?;
    let (from, to) = problem::line(&data)?;
    map.insert(from, to);
  }
  let start = "you".to_string();
  let end = "out".to_string();
  let result = dfs(&start, &end, &mut vec![], &|node| {
    map.get(node).unwrap().clone()
  });

  println!("{}", result);

  let mut tc = TriColor::new(map);
  let res = tc.run();

  println!("{:?}", res);

  Ok(())
}
