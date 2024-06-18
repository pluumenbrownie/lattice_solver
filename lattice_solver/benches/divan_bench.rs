use lattice_solver::{bit_array_settings, test_points::lattice_points, BitArraySettings, Lattice};
use termion::{clear, cursor};

#[divan::bench(max_time = 120, args = [true, false])]
fn full_run(silent: bool) -> Vec<lattice_solver::BitArraySolution> {
    let lattice_points = lattice_points();

    let lattice = Lattice::python_new(lattice_points, 1.1, true);
    let options = bit_array_settings!(lattice);

    let bit_lattice = lattice.get_intermediary(options);

    let result = bit_lattice.solve(true, silent);
    print!("\r{}{}", cursor::Right(22), clear::AfterCursor);
    result
}

#[divan::bench(max_time = 120, args = [true, false])]
fn full_run_parallel(silent: bool) -> Vec<lattice_solver::BitArraySolution> {
    let lattice_points = lattice_points();

    let lattice = Lattice::python_new(lattice_points, 1.1, true);

    let options = bit_array_settings!(lattice);
    let bit_lattice = lattice.get_intermediary(options);

    let result = bit_lattice.solve_parallel(true, silent);
    print!("\r{}{}", cursor::Right(22), clear::AfterCursor);
    result
}

fn main() {
    divan::main();
}
