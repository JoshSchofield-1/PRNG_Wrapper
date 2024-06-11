#[test]
fn test2(){
    #[allow(unused)]
    #[cfg(feature = "nightly-features")]
    use inline_python::{python, Context};

    use rand::{rngs::StdRng, SeedableRng};

    use super::*;
    use crate::py_rng::{helper::NeighboursAndWeights, RngWrapper};

    let seed = 12345;

    let mut a = RngWrapper::PythonRng(PythonRng::new(seed));

    for i in 0..10{
        println!("rng = {}", a.wrapper_gen_range());
    }
}
#[test]
fn test() {
    #[allow(unused)]
    #[cfg(feature = "nightly-features")]
    use inline_python::{python, Context};

    use rand::{rngs::StdRng, SeedableRng};

    use super::*;
    use crate::py_rng::{helper::NeighboursAndWeights, RngWrapper};

    println!("testing");
    let seed = 12345;
    // let mut a = RngWrapper::RustRng(StdRng::seed_from_u64(seed));
    // println!("rust {:?}", a.wrapper_test());
    let mut a = RngWrapper::PythonRng(PythonRng::new(seed));
    // println!("py {:?}", a.wrapper_gen_range());
    // println!("py {:?}", a.wrapper_gen_range());
    // println!("py {:?}", a.wrapper_gen_range());

    let mut rng = StdRng::seed_from_u64(seed);
    let mut b = RngWrapper::RustRng(&mut rng);

    // println!(
    //     "rust {:?} Checking no interference",
    //     b.wrapper_choose_weighted(&NeighboursAndWeights {
    //         p: vec![99, 119, 121, 125],
    //         w: vec![0.49, 0.46, 0.45, 0.34]
    //     })
    // );

    let native_py = [
        120, 124, 118, 120, 120, 124, 98, 98, 124, 120, 98, 120, 98, 118, 118, 98, 98, 118, 120,
        118,
    ];
    // for py_val in native_py {
    //     let ans = a.wrapper_choose_weighted(&NeighboursAndWeights {
    //         p: vec![99, 119, 121, 125],
    //         w: vec![0.49, 0.46, 0.45, 0.34],
    //     });
    //     // assert_eq!(ans, py_val);
    //     println!("py {:?}", ans);
    // }

    let py_choose_mult: Vec<Vec<usize>> = vec![
        vec![4, 8],
        vec![1, 2],
        vec![1, 1],
        vec![6, 0],
        vec![8, 3],
        vec![3, 4],
        vec![3, 8],
        vec![7, 8],
        vec![3, 7],
        vec![0, 5],
    ];
    // for sub_vec in py_choose_mult {
    //     let res = a.wrapper_choose_multiple(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 2);
    //     println!("choose multiple {:?}", res);
    //     for j in 0..1 {
    //         // assert_eq!(sub_vec[j], res[j])
    //     }
    // }
    // Rust gen
    // let mut rng = StdRng::seed_from_u64(654);
    // let mut b = RngWrapper::RustRng(&mut rng);
    // println!(
    //     "bbbb {:?}",
    //     b.wrapper_choose_multiple(&vec![0, 1, 2, 3, 4, 5], 2)
    // );
    let py_poisson_distr = [
        Some(1),
        Some(1),
        Some(5),
        Some(8),
        Some(8),
        Some(1),
        Some(2),
        Some(5),
        Some(8),
        Some(80),
    ];
    let lambda_choice = [1.2, 4.5, 5.3, 7.6, 8.9, 2.1, 3.2, 4.5, 8.9, 87.4];
    for i in 0..10 {

        println!("py {:?}", a.wrapper_gen_range());

        let ans = a.wrapper_choose_weighted(&NeighboursAndWeights {
                    p: vec![99, 119, 121, 125],
                    w: vec![0.49, 0.46, 0.45, 0.34],
                });
        
            println!("py {:?}", ans);

            let ans = a.wrapper_choose_weighted(&NeighboursAndWeights {
                p: vec![99, 119, 121, 125],
                w: vec![0.49, 0.46, 0.45, 0.34],
            });
    
        println!("py {:?}", ans);
            
        let res = a.wrapper_choose_multiple(&vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9], 2);
        println!("choose multiple {:?}", res);
        
        let res = a.wrapper_poisson(lambda_choice[i]);
        println!("poisson: {:?}", res);
        // assert_eq!(res, py_poisson_distr[i]);
    }

    // println!("poisson rust: {:?}", b.wrapper_poisson(4.0));
    // } //for big for loop
}

// Relevant python code
/*
import random as rnd
import numpy as np
seed = 53
rnd.seed(seed)
state = rnd.getstate()
new_state = ("MT19937", [int(s) for s in list(state[1])], 624, 0, 0.0)
np.random.set_state(new_state)
p = [99, 119, 121, 125]
w = [0.49, 0.46, 0.45, 0.34]
neighbours = range(len(p))
for i in range(3):
    rng = np.random.random()
    print(rng)
for i in range(20):
    ans = p[np.random.choice(neighbours,p=[weight/np.sum(w) for weight in w])]-1
    print(ans)
for i in range(10):
    print("choose multiple ", np.random.choice([0,1,2,3,4,5,6,7,8,9], 2))
    
lambda_choice=[1.2, 4.5, 5.3, 7.6, 8.9, 2.1, 3.2, 4.5, 8.9, 87.4]
for i in range(10):
    print("poisson: ", np.random.poisson(lambda_choice[i]))
 */
