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
day01a: .003418518 seconds
day01b: .005495567 seconds
day02a: .004095584 seconds
day02b: .005723129 seconds
day03a: .003571431 seconds
day03b: .004787076 seconds
day04a: .004274599 seconds
day04b: .005927151 seconds
day05a: .004172499 seconds
day05b: .004871803 seconds
day06a: .003372952 seconds
day06b: .004608243 seconds
day07a: .003361521 seconds
day07b: .005312975 seconds
day08a: .003458365 seconds
day08b: .005724442 seconds
day09a: .004818322 seconds
day09b: .008248033 seconds
day10a: .003285266 seconds
day10b: .005485463 seconds
day11a: .003628559 seconds
day11b: .012957232 seconds
day12a: .003674779 seconds
day12b: .006322531 seconds
day13a: .004382679 seconds
day13b: .006866511 seconds
day14a: .005363694 seconds
day14b: .014036432 seconds
Running all
Total: .070116435 seconds
```

### Clojure
I only run this interactively in the lein REPL, though it can be run as a script from the repo root or being included in a REPL session with `(load-file "clojure/dayXX.clj")`.

### Julia 
I run this interactively, but can also be run with `julia julia/dayXX.jl`.

## Notes

* Compile to WASM and build simple html frontend to see how we can run it through the browser?
* Have everything in lib, and bin just launches function? Then tests could be used for performance benchmarks.