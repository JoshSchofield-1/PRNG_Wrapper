use super::PythonRng;
#[allow(unused)]
impl PythonRng {
    // Could in future be extended to take a Range<T> and:
    // where T: std::convert::From<f64> + std::fmt::Debug + pyo3::ToPyObject,
    // This range could be destructed using .start and .end if InclusiveRange is a necessary input this might be harder.
    pub(super) fn py_gen_range(&mut self) -> f64 {
        #[cfg(not(feature = "nightly-features"))]
        panic!("You cannot use the python wrapper without running as cargo +nightly (test|run) -F nightly-features");

        let active = self.active;

        //  prefix to keep clippy happy as variables are used in python macro.
        let seed = self.seed;
        let state = if active {
            self.state.clone().unwrap()
        } else {
            Vec::with_capacity(0) // If state is false then we use seed to start process.
        };
        let iter = self.iter;

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
                rng = np.random.random()

                state_excess_data = np.random.get_state()
                state_after = state_excess_data[1]
                state_after = [a.item() for a in state_after]
                iter  = state_excess_data[2]
            };
            let d = c.get::<f64>("rng");
            self.state = Some(c.get::<Vec<u32>>("state_after"));
            self.iter = c.get::<u32>("iter");
            if !self.active {
                self.active = true;
            }
            return d;
        }
    }
}
