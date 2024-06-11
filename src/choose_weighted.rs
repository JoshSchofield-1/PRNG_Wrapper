use crate::py_rng::helper::NeighboursAndWeights;
use super::PythonRng;
#[allow(unused)]
impl PythonRng{

    pub(super) fn py_choose_weighted(&mut self, n_w: &NeighboursAndWeights, neighbours_to_exclude: Option<&Vec<usize>>) -> usize {
        #[cfg(not(feature = "nightly-features"))]
        panic!("You cannot use the python wrapper without running as cargo +nightly (test|run) -F nightly-features");

        //  prefix to keep clippy happy as variables are used in python macro.
        let active = self.active; // Setting state here as it is harder to access structs in python macro than just a var
        let seed = self.seed; // Needed if it is first call to one of the python generation methods.
        let state = if active { // State is needed if it is not the first call to one of the python generation methods.
            self.state.clone().unwrap()
        } else {
            Vec::with_capacity(0)
        };

        let iter = self.iter; // Needed to get to the right iteration of current PRNG state in python macro

        let patches = &n_w.p; // It is easier to pass references to Vecs to python macro than construct PyO3 Object from Struct.
        let weights = &n_w.w; // Above
        let exclude = neighbours_to_exclude.unwrap_or(&vec![]).to_owned();
        #[cfg(feature = "nightly-features")]
        {
            use inline_python::{python, Context};
            let c: Context = python! {
                import numpy as np

                if 'active:
                    new_state = ("MT19937", [s for s in list('state)], 'iter, 0, 0.0)
                    np.random.set_state(new_state)
                else:
                    np.random.seed('seed)

                all_neighbours = range(len('patches))
                neighbours = [p for p in all_neighbours if(not(p in 'exclude))]

                ans = 'patches[np.random.choice(neighbours,p=[weight/np.sum('weights) for weight in 'weights])]

                state_excess_data = np.random.get_state()
                state_after = state_excess_data[1]
                state_after = [a.item() for a in state_after]
                iter  = state_excess_data[2]
            };
            let d = c.get::<usize>("ans");
            self.state = Some(c.get::<Vec<u32>>("state_after"));
            self.iter = c.get::<u32>("iter");
            return d;
        }
    }
}
