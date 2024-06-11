use super::PythonRng;

#[allow(unused)]
impl PythonRng{
    // lambda is used in python macro
    pub fn py_poisson(&mut self, lambda: f64) -> f64 {
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
                    new_state = ("MT19937", [s for s in list('state)], 'iter, 0, 0.0)
                    np.random.set_state(new_state)
                else:
                    np.random.seed('seed)

                litter_size = np.random.poisson('lambda)

                state_excess_data = np.random.get_state()
                state_after = state_excess_data[1]
                state_after = [a.item() for a in state_after]
                iter  = state_excess_data[2]


            };
            self.state = Some(c.get::<Vec<u32>>("state_after"));
            self.iter = c.get::<u32>("iter");
            if !self.active {
                self.active = true;
            }
            let mut d = c.get::<f64>("litter_size");
            return d;
        }
    }
}
