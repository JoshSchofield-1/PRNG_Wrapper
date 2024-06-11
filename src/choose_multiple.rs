#![allow(unreachable_code)]
#![allow(unused)]

use super::PythonRng;
impl PythonRng{
    // both select_from and num_selected are used in python macro.
    pub(super) fn py_choose_multiple(&mut self, select_from: &[usize], num_selected: usize) -> Vec<usize>
where {
        #[cfg(not(feature = "nightly-features"))]
        panic!("You cannot use the python wrapper without running as cargo +nightly (test|run) -F nightly-features");

        let active = self.active; // Setting state here as it is harder to access structs in python macro than just a var
        let seed = self.seed; // Needed if it is first call to one of the python generation methods.
        let state = if active { // State is needed if it is not the first call to one of the python generation methods.
            self.state.clone().unwrap()
        } else {
            Vec::with_capacity(0)
        };

        let iter = self.iter; // Needed to get to the right iteration of current PRNG state in python macro

        #[cfg(feature = "nightly-features")]
        {
            use inline_python::{python, Context};
            let c: Context = python! {
                import numpy as np
                if 'active:
                    // print("using rust state")
                    // print("iter val", 'iter)
                    new_state = ("MT19937", [s for s in list('state)], 'iter, 0, 0.0)
                    np.random.set_state(new_state)
                else:
                //     import random as rnd
                //     rnd.seed('seed)
                //     state = rnd.getstate()
                //     new_state = ("MT19937", [int(s) for s in list(state[1])], 624, 0, 0.0)
                // np.random.set_state(new_state)
                    np.random.seed('seed)

                infected = np.random.choice('select_from, 'num_selected)

                state_excess_data = np.random.get_state()
                state_after = state_excess_data[1]
                state_after = [a.item() for a in state_after]
                iter  = state_excess_data[2]


            };
            // let d = c.get::<f64>("rng");
            self.state = Some(c.get::<Vec<u32>>("state_after"));
            self.iter = c.get::<u32>("iter");
            // println!("state: {:?}",self.state);
            if !self.active {
                self.active = true;
            }
            let mut d = c.get::<Vec<usize>>("infected");
            return d;
        }
    }
}
