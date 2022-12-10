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
day01a: .003240754 seconds
day01b: .004567574 seconds
day02a: .003123273 seconds
day02b: .004573721 seconds
day03a: .003215318 seconds
day03b: .004886804 seconds
day04a: .003394494 seconds
day04b: .005017601 seconds
day05a: .007347329 seconds
day05b: .007845915 seconds
day06a: .005269918 seconds
day06b: .006309376 seconds
day07a: .004375795 seconds
day07b: .006059083 seconds
day08a: .010002792 seconds
day08b: .005799080 seconds
day09a: .005946728 seconds
day09b: .006576986 seconds
Running all
Total: .036772042 seconds
```

### Clojure
I only run this interactively in the lein REPL, though it can be run as a script from the repo root or being included in a REPL session with `(load-file "clojure/dayXX.clj")`.

### Julia 
I run this interactively, but can also be run with `julia julia/dayXX.jl`.

## Notes

* Compile to WASM and build simple html frontend to see how we can run it through the browser?
* Have everything in lib, and bin just launches function? Then tests could be used for performance benchmarks.