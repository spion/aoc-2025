use anyhow::Result;
use std::{
  cmp::max,
  io::{self, Read},
};

use good_lp::{
  self, Constraint, ProblemVariables, Solution, SolutionStatus, SolverModel, microlp, variable,
};

#[derive(Debug)]
struct Present {
  _index: usize,
  structure: Vec<Vec<bool>>,
}

impl Present {
  fn sum_tiles(&self) -> i32 {
    self.structure.iter().map(|l| l.iter().filter(|v| **v).count()).sum::<usize>() as i32
  }
}

#[derive(Debug)]
struct Region {
  w: i32,
  l: i32,
  desired_presents: Vec<i32>,
}

peg::parser! {
  grammar problem() for str {
    rule _() = quiet!{[' ' | '\t']+}

    rule number() -> i32
      = n:$(['0'..='9']+) {? n.parse().or(Err("Cant parse unsigned")) }

    rule present() -> Present
      = n:number() ":" _* "\n" t:(tile_line() ++ "\n") { Present {_index: n as usize, structure: t}  }

    rule tile_dot() -> bool
      = d:['.' | '#'] { d == '#' }

    rule tile_line() -> Vec<bool>
      = l:(tile_dot() ++ (_*)) { l }


    rule presents() -> Vec<Present>
      = t:(present() ++ ("\n"+)) { t }

    rule region() -> Region
      = w:number() "x" l:number() ":" _ desired_presents:(number() ++ _) _* { Region { w, l, desired_presents } }

    rule regions() -> Vec<Region>
      = l:(region() ++ "\n") { l }

    pub rule parse() -> (Vec<Present>, Vec<Region>)
      = p:presents() "\n\n" r:regions() "\n"* { (p, r) }

  }
}

fn fits_easily(problem: &Region) -> bool {
  (problem.w / 3) * (problem.l / 3) >= problem.desired_presents.iter().sum()
}

fn fit_impossible(problem: &Region, presents: &Vec<Present>) -> bool {
  let total = problem
    .desired_presents
    .iter()
    .enumerate()
    .map(|(ix, dp)| presents[ix].sum_tiles() * *dp)
    .sum::<i32>();
  total > problem.w * problem.l
}

// This is a simplified version that used no rotation and combines some presents into rectangles
// in a hardcoded way,
// but it turned out that between the easily and impossible, there are no remaining items to run
// with the rectangle solver
fn linear_solve(objs: &Vec<(i32, i32)>, w: i32, l: i32) -> bool {
  let big_m = max(w, l);

  let mut p = ProblemVariables::new();
  let vars = objs
    .iter()
    .map(|_| {
      (
        p.add(variable().min(0).max(w - 1).integer()),
        p.add(variable().min(0).max(l - 1).integer()),
        p.add(variable().binary()),
      )
    })
    .collect::<Vec<_>>();

  let mut cons: Vec<Constraint> = vec![];
  for ((ox, oy), (vx, vy, vrot)) in objs.iter().zip(vars.iter()) {
    if ox == oy {
      let con1: Constraint = (*vx + *ox).leq(w);
      let con2: Constraint = (*vy + *oy).leq(l);
      cons.push(con1);
      cons.push(con2);
    } else {
      let con1 = (*vx + (1 - *vrot) * *ox + *vrot * *oy).leq(w);
      let con2 = (*vy + (1 - *vrot) * *oy + *vrot * *ox).leq(l);
      cons.push(con1);
      cons.push(con2);
    }
  }

  for ((i, ((ox1, oy1), (vx1, vy1, vrot1))), (j, ((ox2, oy2), (vx2, vy2, vrot2)))) in
    objs.iter().zip(vars.iter()).enumerate().zip(objs.iter().zip(vars.iter()).enumerate())
  {
    if i >= j {
      continue;
    }
    let lij = p.add(variable().binary());
    let bij = p.add(variable().binary());
    let rij = p.add(variable().binary());
    let tij = p.add(variable().binary());

    let w1 = *ox1 + (*oy1 - *ox1) * *vrot1;
    let h1 = *oy1 + (*ox1 - *oy1) * *vrot1;
    let w2 = *ox2 + (*oy2 - *ox2) * *vrot2;
    let h2 = *oy2 + (*ox2 - *oy2) * *vrot2;

    let con: Constraint = (tij + lij + bij + rij).eq(1);

    let con_left: Constraint = (*vx1 + w1).leq(*vx2 + (1 - lij) * big_m);
    let con_bottom: Constraint = (*vy1 + h1).leq(*vy2 + (1 - bij) * big_m);
    let con_right: Constraint = (*vx2 + w2).leq(*vx1 + (1 - rij) * big_m);
    let con_top: Constraint = (*vy2 + h2).leq(*vy1 + (1 - tij) * big_m);

    cons.push(con);
    cons.push(con_left);
    cons.push(con_bottom);
    cons.push(con_right);
    cons.push(con_top);
  }

  let mut model = p.minimise(0).using(microlp);
  for c in cons {
    model = model.with(c);
  }

  matches!(model.solve(), Ok(v) if {
    match v.status() {
      SolutionStatus::Optimal => true,
      _ => false
    }
  })
}

fn fits_linear_simple(problem: &Region) -> bool {
  let pc4x3 = problem.desired_presents[2] + problem.desired_presents[1];
  let c4x3 = (pc4x3) / 2;
  let extra3x3 = pc4x3 % 2;
  let c4x4 = problem.desired_presents[3] / 2;
  let extra3x3p2 = problem.desired_presents[3] % 2;

  let c3x3 = problem.desired_presents[0]
    + problem.desired_presents[4]
    + problem.desired_presents[5]
    + extra3x3
    + extra3x3p2;

  let objs = (0..c3x3)
    .map(|_| (3, 3))
    .chain((0..c4x3).map(|_| (4, 3)))
    .chain((0..c4x4).map(|_| (4, 4)))
    .collect::<Vec<_>>();

  return linear_solve(&objs, problem.w as i32, problem.l as i32);
}

fn main() -> Result<()> {
  let mut data = String::new();
  io::stdin().read_to_string(&mut data)?;
  let (presents, regions) = problem::parse(&data)?;

  let mut sum = 0;
  for r in regions {
    if fits_easily(&r) {
      sum += 1;
    } else if fit_impossible(&r, &presents) {
      continue;
    } else if fits_linear_simple(&r) {
      sum += 1;
      println!("Linear simple");
    }
  }

  println!("Found {}", sum);
  Ok(())
}
// 1000 too high
