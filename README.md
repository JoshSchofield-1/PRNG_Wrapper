# PRNG_Wrapper
Rust wrapper to allow use of either python or rust random number generation. The goal of this project is to allow for easier debugging of stochastic processes in programs ported from Python to Rust.
- Note: nightly is required </br>
  https://github.com/rust-lang/rust/issues/54725 </br>
- gen_range takes a range however; it is not used. This is done to make it easier to implement into an existing project. </br>
- Using the Python wrapper in a large project is very slow due to the repeated creation and deletion of VMs. </br>
- NeighboursAndWeights describes a Struct that contains 2 vectors a vector of values to select from and a vector of weights for each of the values. </br>
## Examples
let a = RngWrapper::PythonRng(seed); (seed must be within the bounds of a u32)</br></br>
let StdRng = StdRng::seed_from_u64(seed);</br>
let b = RngWrapper::RustRng(&mut StdRng)
## Running
cargo +nightly run -F nightly-features <- if using PythonRng </br>
cargo run <- if using RustRng (can also be run with nightly but is not necessary)
## Functionality implemented
There are wrapper methods for the following:
Wrapper name|Rust | python
|---|---|---|
|gen_range()|gen_range| random.random()|
|choose_multiple()|choose_multiple|choose_multiple()| np.random.choice()|
|choose_weighted()|choose_weighted| np.random.choice(p=weights)|
|wrapper_poisson()|poisson|np.random.poisson|
|return_info|None|return_info (returns state info of PRNG)|
