# Advent Of Code 2022
My solutions to the puzzles in https://adventofcode.com/2022

I aim to do all of them in Rust, though I might use Clojure or Julia at least for an initial solution if that seems like a better fit.

## Running

### Rust
From the root directory run
`cargo run bin --dayXX Y`
where XX is they day (number 01-25) and Y is the problem (a or b).

Run `./time.sh` to time the execution for the release builds of all the scripts.
The goal is to keep the total under 1 second.

```
Running individual
day01a: .002847715 seconds
day01b: .004066580 seconds
day02a: .002845375 seconds
day02b: .004002992 seconds
day03a: .003129370 seconds
day03b: .004642451 seconds
day04a: .003884889 seconds
day04b: .004963463 seconds
day05a: .003271334 seconds
day05b: .005545138 seconds
day06a: .003217793 seconds
day06b: .006004574 seconds
day07a: .004234682 seconds
day07b: .005534068 seconds
day08a: .003785911 seconds
day08b: .005902191 seconds
day09a: .005648532 seconds
day09b: .006937284 seconds
day10a: .002854194 seconds
day10b: .005736872 seconds
day11a: .002965191 seconds
day11b: .013141383 seconds
day12a: .003741466 seconds
day12b: .004607043 seconds
day13a: .003727160 seconds
day13b: .005470753 seconds
day14a: .007654738 seconds
day14b: .130029328 seconds
Running all
Total: .202321371 seconds
```

### Clojure
I only run this interactively in the lein REPL, though it can be run as a script from the repo root or being included in a REPL session with `(load-file "clojure/dayXX.clj")`.

### Julia 
I run this interactively, but can also be run with `julia julia/dayXX.jl`.

## Notes

* Compile to WASM and build simple html frontend to see how we can run it through the browser?
* Have everything in lib, and bin just launches function? Then tests could be used for performance benchmarks.