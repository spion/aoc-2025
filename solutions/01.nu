#!/bin/env nu

def main [] {
  cat | 
  parse -r '([LR])([\d]+)' |
  rename dir val |
  into int val |
  reduce --fold { val: 50, count: 0 }  {|el, acc|
    let val = (if $el.dir == "R" { ($acc.val + $el.val) } else { $acc.val - $el.val }) mod 100
    let count = $acc.count + (if $val == 0 { 1 } else { 0 })
    return {val: $val, count: $count} 
  }
}
