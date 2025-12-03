#!/bin/env nu

def main [] {
  cat |
  parse -r '([LR])([\d]+)' |
  rename dir val |
  into int val |
  reduce --fold { val: 50, count: 0 }  {|el, acc|
    let val_no_mod = if $el.dir == "R" { ($acc.val + $el.val) } else { $acc.val - $el.val }

    let val = $val_no_mod mod 100

    # too late, cant math
    let backward_zero_bonus = if $val_no_mod <= 0 and $acc.val > 0 { 1 } else { 0 }
    let count = $acc.count + $backward_zero_bonus + ($val_no_mod | math abs) // 100
    return {val: $val, count: $count}
  }
}

