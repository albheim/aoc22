# Advent Of Code 2022
My solutions to the puzzles in https://adventofcode.com/2022

I aim to do all of them in Rust, though I might use Clojure or Julia at least for an initial solution if that seems like a better fit.

## How to run

### Rust
From the root directory run
`cargo run bin --dayXX Y`
where XX is they day (number 01-25) and Y is the problem (a or b).

Run `./time.sh` to time the execution for the release builds of all the scripts.

### Clojure
I only run this interactively in the lein REPL, though it can be run as a script from the repo root or being included in a REPL session with `(load-file "clojure/dayXX.clj")`.

### Julia 
I run this interactively, but can also be run with `julia julia/dayXX.jl Y`.

## Notes

* Compile to WASM and build simple html frontend to see how we can run it through the browser?
* Have everything in lib, and bin just launches function? Then tests could be used for performance benchmarks.