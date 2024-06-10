#![allow(clippy::excessive_precision)]
use lattice_solver::{BitArraySettings, Lattice};

fn main() {    
    let lattice_points = vec![(vec![1.5, 0.0], vec![vec![1.5, 5.196152422706632]]), (vec![0.0, 0.0], vec![vec![0.0, 5.196152422706632], vec![6.0, 0.0], vec![6.0, 5.196152422706632]]), (vec![2.25, 1.299038105676658], vec![]), (vec![0.75, 1.299038105676658], vec![vec![6.75, 1.299038105676658]]), (vec![1.5, 2.598076211353316], vec![]), (vec![0.0, 2.598076211353316], vec![vec![6.0, 2.598076211353316]]), (vec![2.25, 3.897114317029974], vec![]), (vec![0.75, 3.897114317029974], vec![vec![6.75, 3.897114317029974]]), (vec![4.5, 0.0], vec![vec![4.5, 5.196152422706632]]), (vec![3.0, 0.0], vec![vec![3.0, 5.196152422706632]]), (vec![5.25, 1.299038105676658], vec![]), (vec![3.75, 1.299038105676658], vec![]), (vec![4.5, 2.598076211353316], vec![]), (vec![3.0, 2.598076211353316], vec![]), (vec![5.25, 3.897114317029974], vec![]), (vec![3.75, 3.897114317029974], vec![])];

    let lattice = Lattice::python_new(lattice_points, 1.1, true);
    let options = BitArraySettings::create(2, 0.1, 1.0, 1.0);
    let bit_lattice = lattice.get_intermediary(options);

    let _ = bit_lattice.solve_parallel(true, false);
}