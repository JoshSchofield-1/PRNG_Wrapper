use rand::{rngs::StdRng, seq::SliceRandom, Rng};
use rand_distr::{num_traits::ToPrimitive, Poisson};

use self::helper::NeighboursAndWeights;
mod choose_weighted;
mod poisson;
mod choose_multiple;
mod gen_range;

// using #[allow(unused)] to prevent clippy showing issues where variables are accessed from python macro / locked behind feature = nightly-feature
// Gen _ wrapper methods match self.enum type to work out whether to use rust StdRng call, or use python VM
// Python implementations are in py_rng

#[derive(Debug)]
pub enum RngWrapper<'a> {
    RustRng(&'a mut StdRng),
    PythonRng(PythonRng),
}

impl<'a> RngWrapper<'a> {

    // Used to access PRNG state information at current time
    // Panics if used with rust StdRng
    // Returns (seed, state, iter)
    // seed = u64 used to initialise the PRNG (passed in by)
    pub fn return_info(&self) -> (u64, Vec<u32>, u32){
        match self{
            RngWrapper::RustRng(_) => (panic!("not useable with rust prng")),
            RngWrapper::PythonRng(py_rng) => py_rng.return_info(),
        }
    }
    // Wrapper to allow selection from a poisson distribution centred on Lambda
    // NOTE internal functions return f64, external wrapper returns Option<u8> 
    //      for better interaction elsewhere. This could be changed if there where additional uses of Poisson Distr
    pub fn wrapper_poisson(&mut self, lambda: f64) -> f64 {
        match self {
            RngWrapper::RustRng(rng) => {
                let poisson_distr = Poisson::new(lambda).unwrap();
                let litter_size = rng.sample(poisson_distr);
                litter_size
            }
            #[allow(unused)] // py_rng used within feature restricted call
            RngWrapper::PythonRng(py_rng) => {
                #[cfg(feature = "nightly-features")]
                return py_rng.py_poisson(lambda);     
                #[cfg(not(feature = "nightly-features"))]
                panic!("You cannot use the python wrapper without running as cargo +nightly (test|run) -F nightly-features");
            },


        }
    }
    
    // do we want to pass only indexes? might make more sense...

    // Wrapper to allow selection of multiple indexes from a given vector of indexes
    // This operation needs to be done on indexes as otherwise we would have to find convert structs into PyO3 Objects.
    // Wrapper takes in a vector of indexes to select from and a number of samples to return.
    pub fn choose_multiple(
        &mut self,
        select_from: &Vec<usize>,
        num_selected: usize,
    ) -> Vec<usize>
where {
        match self {
            RngWrapper::RustRng(rng) => {
                let infected: Vec<&usize> =
                    select_from.choose_multiple(rng, num_selected).collect();
                infected.into_iter().cloned().collect()
            }
            #[allow(unused)] // py_rng used within feature restricted call
            RngWrapper::PythonRng(py_rng) => {
                #[cfg(feature = "nightly-features")]
                return py_rng.py_choose_multiple(select_from, num_selected);
                #[cfg(not(feature = "nightly-features"))]
                panic!("You cannot use the python wrapper without running as cargo +nightly (test|run) -F nightly-features");
            }
        }
    }
    
    // If not +nightly and -F nightly-features then panic
    // Wrapper to generate range, between 0.0..1.0
    // This could be extended to allow range to be passed into the function, by taking a Range<T> and returning type T with constraits below
    // This is not needed for our application.
    pub fn gen_range<T>(&mut self, _discarded_range: T) -> f64
    {
        match self {
            RngWrapper::RustRng(rng) => {
                rng.gen_range(0.0..1.0)},
            
            #[allow(unused)] // Due to py_rng being behind feature requirement
            RngWrapper::PythonRng(py_rng) => {
                #[cfg(feature = "nightly-features")]
                return py_rng.py_gen_range();
                #[cfg(not(feature = "nightly-features"))]
                panic!("You cannot use the python wrapper without running as cargo +nightly (test|run) -F nightly-features");
            }
        }
    }

    // Wrapper to select an option from an array given a NeighboursAndWeights struct.
    // Struct is madfe up of a vector of usize (neighbours)
    // And a vector of weights for those neighbours.
    pub fn choose_weighted(&mut self, n_w: &NeighboursAndWeights, neighbours_to_exclude: Option<&Vec<usize>>) -> usize {
        match self {
            RngWrapper::RustRng(rng) => {
                n_w.construct_tuple_arr(neighbours_to_exclude.to_owned()) // HACK to owned
                    .choose_weighted(rng, |item| item.1)
                    .unwrap()
                    .0
                    - 1
            }
            #[allow(unused)] // py_rng used within feature restricted call
            RngWrapper::PythonRng(py_rng) => {
                #[cfg(feature = "nightly-features")]
                return py_rng.py_choose_weighted(n_w, neighbours_to_exclude);
                #[cfg(not(feature = "nightly-features"))]
                panic!("You cannot use the python wrapper without running as cargo +nightly (test|run) -F nightly-features");
            },
        }
    }
}

// Struct to store necessary persistent information for Python! macro
#[derive(Debug)]
pub struct PythonRng {
    pub seed: u64,  // Seeds the Python VM PRNG if it is the first run.
    state: Option<Vec<u32>>, // Array representing current state of random number generator this is needed to reseed the PRNG when the VM is closed
    iter: u32, // not 100% sure I think this represents the point of time within current state (needed for PRNG samples to actually vary given a state)
    active: bool, //Used to check whether or not to use state and iter or whether to start from seed value
}

impl PythonRng {
    pub fn new(new_seed: u64) -> PythonRng {
        PythonRng {
            seed: new_seed,
            state: None,
            iter: 0,
            active: false,
        }
    }

    pub fn return_info(&self) -> (u64, Vec<u32>, u32){
        (self.seed, self.state.clone().unwrap_or(vec![]), self.iter)
    }

}

#[cfg(test)]
mod test;
