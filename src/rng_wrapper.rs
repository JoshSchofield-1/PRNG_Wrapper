use rand::{rngs::StdRng, seq::SliceRandom, Rng};
use rand_distr::Poisson;

use crate::helper::NeighboursAndWeights;

#[cfg(feature = "nightly-features")]
use inline_python::{python, Context};

// #[cfg(feature = "nightly-features")]
// use inline_python::{python, Context};

// // using #[allow(unused)] to prevent clippy showing issues where variables are accessed from python macro / locked behind feature = nightly-feature
// // Gen _ wrapper methods match self.enum type to work out whether to use rust StdRng call, or use python VM
// // Python implementations are in py_rng

pub enum RngWrapper<'a> {
    RustRng(&'a mut StdRng),
    // #[cfg(feature = "nightly-features")]
    PythonRng(PythonRng),
}


pub struct PythonRng{
    #[cfg(feature = "nightly-features")]
    py_ctx: Context
}

impl PythonRng{
    #[cfg(feature = "nightly-features")]
    pub fn new(new_seed: u32) -> PythonRng {
        PythonRng{
            py_ctx: python!{
                import numpy as np
                np.random.seed('new_seed)
            }
        }
    }
    #[cfg(not(feature = "nightly-features"))]
            pub fn new(new_seed: u32) -> PythonRng {
                panic!("Must use -F nightly-features and +nightly to use PythonRng")
            }
}

impl<'a> RngWrapper<'a> {

    // Wrapper to allow selection from a poisson distribution centred on Lambda
    // NOTE internal functions return f64, external wrapper returns Option<u8>
    //      for better interaction elsewhere. This could be changed if there where additional uses of Poisson Distr
    pub fn poisson(&mut self, lambda: f64) -> f64 {
        match self {
            RngWrapper::RustRng(rng) => {
                let poisson_distr = Poisson::new(lambda).unwrap();
                rng.sample(poisson_distr)
            }
            #[cfg(feature = "nightly-features")]
            RngWrapper::PythonRng(py_rng) => {
                    py_rng.py_ctx.run(python!{
                        answer = np.random.poisson('lambda)
                    });
                    return py_rng.py_ctx.get::<f64>("answer");            
            }
            #[cfg(not(feature = "nightly-features"))]
            RngWrapper::PythonRng(_) => panic!("Must use -F nightly-features and +nightly to use PythonRng")
        }
        
    }

    // Wrapper to allow selection of multiple indexes from a given vector of indexes
    // This operation needs to be done on indexes as otherwise we would have to find convert structs into PyO3 Objects.
    // Wrapper takes in a vector of indexes to select from and a number of samples to return.
    pub fn choose_multiple(&mut self, select_from: &Vec<usize>, num_selected: usize) -> Vec<usize> {
        match self {
            RngWrapper::RustRng(rng) => {
                let infected: Vec<&usize> =
                    select_from.choose_multiple(rng, num_selected).collect();
                infected.into_iter().cloned().collect()
            }
            #[cfg(feature = "nightly-features")]
            RngWrapper::PythonRng(py_rng) => {
                    py_rng.py_ctx.run(python!{
                        answer = np.random.choice('select_from, 'num_selected)
                    });
                    return py_rng.py_ctx.get::<Vec<usize>>("answer");
            }
            #[cfg(not(feature = "nightly-features"))]
            RngWrapper::PythonRng(_) => panic!("Must use -F nightly-features and +nightly to use PythonRng")
        }
    }

    // If not +nightly and -F nightly-features then panic
    // Wrapper to generate range, between 0.0..1.0
    // This could be extended to allow range to be passed into the function, by taking a Range<T> and returning type T with constraits below
    // This is not needed for our application.
    pub fn gen_range<T>(&mut self, _discarded_range: T) -> f64 {
        match self {
            RngWrapper::RustRng(rng) => rng.gen_range(0.0..1.0),

            #[cfg(feature = "nightly-features")]
            RngWrapper::PythonRng(py_rng) => {
                    py_rng.py_ctx.run(python!{
                        answer = np.random.random()
                    });
                    return py_rng.py_ctx.get::<f64>("answer");
                
            }
            #[cfg(not(feature = "nightly-features"))]
            RngWrapper::PythonRng(_) => panic!("Must use -F nightly-features and +nightly to use PythonRng")
        }
    }

    // Wrapper to select an option from an array given a NeighboursAndWeights struct.
    // Struct is madfe up of a vector of usize (neighbours)
    // And a vector of weights for those neighbours.
    pub fn choose_weighted(
        &mut self,
        n_w: &NeighboursAndWeights,
        neighbours_to_exclude: Option<&Vec<usize>>,
    ) -> usize {
        match self {
            RngWrapper::RustRng(rng) => {
                n_w.construct_tuple_arr(neighbours_to_exclude.to_owned()) // HACK to owned
                    .choose_weighted(rng, |item| item.1)
                    .unwrap()
                    .0
                    - 1
            }
            #[cfg(feature = "nightly-features")]
            RngWrapper::PythonRng(py_rng) => {                    
                    let patches = &n_w.p; // It is easier to pass references to Vecs to python macro than construct PyO3 Object from Struct.
                    let weights = &n_w.w; // Above
                    let exclude = neighbours_to_exclude.unwrap_or(&vec![]).to_owned();                    
                    py_rng.py_ctx.run(python!{
                        all_neighbours = range(len('patches))
                        neighbours = [p for p in range(len('patches)) if(not('patches[p]-1 in 'exclude))]
                        weights = ['weights[p] for p in neighbours]
                        a = np.random.choice(neighbours,p=[weight/np.sum(weights) for weight in weights])
                        answer = 'patches[a] -1                            
                    });
                    return py_rng.py_ctx.get::<usize>("answer");
            }
            #[cfg(not(feature = "nightly-features"))]
            RngWrapper::PythonRng(_) => panic!("Must use -F nightly-features and +nightly to use PythonRng")
        }
    }
}