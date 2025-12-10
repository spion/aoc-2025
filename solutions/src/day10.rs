use anyhow::Result;
use good_lp::{
  self, Expression, ProblemVariables, Solution, SolverModel, Variable, coin_cbc, variable,
};
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::io;

#[derive(Debug)]
struct Machine {
  indicators: u64,
  buttons: Vec<Vec<u64>>,
  joltages: Vec<u64>,
}

fn bools_to_u64(bools: &Vec<bool>) -> u64 {
  bools.iter().enumerate().map(|(ix, b)| if *b { 2u64.pow(ix as u32) } else { 0 }).sum()
}

fn wirings_to_u64(wirings: &Vec<u64>) -> u64 {
  wirings.iter().map(|ix| 2u64.pow(*ix as u32)).sum()
}

fn bfs<T, F>(start: T, goal: T, moves: F) -> usize
where
  F: Fn(&T) -> Vec<T>,
  T: Eq + Hash + Clone,
{
  let mut visited: HashMap<T, usize> = HashMap::new();
  let mut queue: VecDeque<T> = VecDeque::new();

  let mut pos = start;
  visited.insert(pos.clone(), 0);
  loop {
    let new_moves = moves(&pos);
    let pos_len = *visited.get(&pos).unwrap();

    for c in new_moves.into_iter() {
      if visited.contains_key(&c) {
        continue;
      }
      if queue.contains(&c) {
        continue;
      }
      if c == goal {
        return pos_len + 1;
      }
      visited.insert(c.clone(), pos_len + 1);
      queue.push_back(c.clone());
    }

    match queue.pop_front() {
      Some(el) => pos = el,
      None => return 0,
    }
  }
}

fn moves_pt1(pos: u64, buttons: Vec<u64>) -> Vec<u64> {
  buttons.iter().map(|b| pos ^ b).collect()
}

fn wirings_to_counters(ws: Vec<u64>, len: usize) -> Vec<u64> {
  let mut r = vec![0; len];
  for w in ws {
    r[w as usize] = 1;
  }
  r
}

fn moves_pt2(pos: &Vec<u64>, buttons: &Vec<Vec<u64>>, max: &Vec<u64>) -> Vec<Vec<u64>> {
  let m = buttons
    .iter()
    .filter_map(|b| {
      let mut new_opt = pos.clone();
      for &ix in b {
        let i = ix as usize;
        new_opt[i] += 1;
        if new_opt[i] > max[i] {
          return None;
        }
      }
      return Some(new_opt);
    })
    .collect();

  //println!("{:?} :: {:?} -> {:?}", pos, max, m);
  m
}

peg::parser! {
  grammar problem() for str {
    rule _() = quiet!{[' ' | '\n' | '\t']+}

    rule number() -> u64
      = n:$(['0'..='9']+) {? n.parse().or(Err("Cant parse u64")) }

    rule list() -> Vec<u64>
      = l:(number() ** ",") { l }

    rule light_off() -> bool
      = "." { false }

    rule light_on() -> bool
      = "#" { true }

    rule lights() -> u64
      = "[" ls:((light_off() / light_on())+) "]" { bools_to_u64(&ls) }

    rule button_wires() -> Vec<u64>
      = "(" l:list() ")" { l }

    rule buttons() -> Vec<Vec<u64>>
      = bts:(button_wires() ++ " ") { bts }

    rule joltages() -> Vec<u64>
      = "{" l:list() "}" { l }

    pub rule machine() -> Machine
      = l:lights() _ b:buttons() _ j:joltages() {
        Machine {indicators: l, buttons: b, joltages: j }
      }

    pub rule machines() -> Vec<Machine>
      = l:(machine() ** "\n") _* { l }

  }
}

fn main() -> Result<()> {
  //let args: Vec<String> = env::args().collect();
  //let solution_part = args.get(1).map(|x| x.as_str()).unwrap_or("pt1");

  let mut solution_pt1 = 0usize;
  let mut solution_pt2 = 0.0f64;
  for line in io::stdin().lines() {
    let data = line?;
    println!("Data {}", data);
    let machine = problem::machine(&data)?;
    let presses_pt1 = bfs(0, machine.indicators, |p| {
      moves_pt1(
        *p,
        machine.buttons.iter().map(|b| wirings_to_u64(b)).collect(),
      )
    });

    solution_pt1 += presses_pt1;
    println!("Solution pt1 {}", presses_pt1);

    let mut p = ProblemVariables::new();
    let vars = vec![variable().min(0).integer(); machine.buttons.len()];
    let y: Vec<Variable> = p.add_all(vars);
    let objective: Expression = y.iter().sum();

    let mut model = p.minimise(objective).using(coin_cbc);
    // add constraints
    for (jx, jolt) in machine.joltages.iter().enumerate() {
      let expr: Expression = machine
        .buttons
        .iter()
        .enumerate()
        .filter(|(_, b)| b.contains(&(jx as u64)))
        .map(|(bx, _)| y[bx])
        .sum();
      let constr = expr.eq(*jolt as u32);

      model = model.with(constr);
    }

    let solution = model.solve()?;
    //let solution = model.solve()?

    let presses_pt2 = y.iter().map(|v| solution.value(*v)).sum::<f64>();
    solution_pt2 += presses_pt2;
    println!("Solution pt2 {}", presses_pt2);
  }

  println!("{}", solution_pt1);
  println!("{}", solution_pt2);

  Ok(())
}
// 20085 too low
