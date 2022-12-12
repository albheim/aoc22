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
day01a: .002731227 seconds
day01b: .004126877 seconds
day02a: .003301529 seconds
day02b: .005724194 seconds
day03a: .003913865 seconds
day03b: .008602617 seconds
day04a: .005485742 seconds
day04b: .005960392 seconds
day05a: .003934152 seconds
day05b: .004895211 seconds
day06a: .003388487 seconds
day06b: .005647660 seconds
day07a: .004257577 seconds
day07b: .005746447 seconds
day08a: .004624830 seconds
day08b: .005601862 seconds
day09a: .005300220 seconds
day09b: .008312008 seconds
day10a: .003603243 seconds
day10b: .005278930 seconds
day11a: .004016940 seconds
day11b: .013537020 seconds
day12a: .004025859 seconds
day12b: .005743775 seconds
Running all
Total: .059686830 seconds
```

### Clojure
I only run this interactively in the lein REPL, though it can be run as a script from the repo root or being included in a REPL session with `(load-file "clojure/dayXX.clj")`.

### Julia 
I run this interactively, but can also be run with `julia julia/dayXX.jl`.

## Notes

* Compile to WASM and build simple html frontend to see how we can run it through the browser?
* Have everything in lib, and bin just launches function? Then tests could be used for performance benchmarks.