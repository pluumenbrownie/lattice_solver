use pyo3::prelude::*;
use std::ffi::OsString;

use ::lattice_solver::BitArrayRepresentation as WrappedRepresentation;
use ::lattice_solver::BitArraySolution as WrappedSolution;
use ::lattice_solver::Lattice as WrappedLattice;
use ::lattice_solver::SiteFilter as WrappedFilter;

#[pyclass]
struct BitArraySolution {
    wrapped: WrappedSolution,
}

#[pyclass]
#[derive(Clone)]
struct SiteFilter {
    wrapped: WrappedFilter
}

impl SiteFilter {
    fn empty() -> Self {
        SiteFilter{wrapped: WrappedFilter::empty()}
    }
}

#[pyclass]
struct BitArrayRepresentation {
    wrapped: WrappedRepresentation,
}

#[pymethods]
impl BitArrayRepresentation {
    fn solve(&self, find_all: bool) -> Vec<BitArraySolution> {
        self.wrapped
            .solve(find_all)
            .into_iter()
            .map(|a| BitArraySolution { wrapped: a })
            .collect()
    }

    fn __str__(&self) -> String {
        self.wrapped.__str__()
    }

    fn __repr__(&self) -> String {
        self.wrapped.__repr__()
    }
}

#[pyclass]
struct Lattice {
    wrapped: WrappedLattice,
}

#[pymethods]
impl Lattice {
    #[new]
    #[pyo3(signature = (input_lattice, distance_margin, autodetect_margin=true))]
    fn python_new(input_lattice: Vec<(Vec<f32>, Vec<Vec<f32>>)>, distance_margin: f32, autodetect_margin: bool) -> Self {
        Lattice {
            wrapped: WrappedLattice::python_new(input_lattice, distance_margin, autodetect_margin),
        }
    }

    fn points_to_plot(&self) -> (Vec<f32>, Vec<f32>) {
        self.wrapped.points_to_plot()
    }

    fn oxygens_to_plot(&self) -> (Vec<f32>, Vec<f32>) {
        self.wrapped.oxygens_to_plot()
    }

    fn tripoints_to_plot(&self) -> (Vec<f32>, Vec<f32>) {
        self.wrapped.tripoints_to_plot()
    }

    fn midpoints_to_plot(&self) -> (Vec<f32>, Vec<f32>) {
        self.wrapped.midpoints_to_plot()
    }

    fn singlets_to_plot(&self) -> (Vec<f32>, Vec<f32>) {
        self.wrapped.singlets_to_plot()
    }

    #[pyo3(signature = (filter=SiteFilter::empty()))]
    fn get_intermediary(&self, filter: SiteFilter) -> BitArrayRepresentation {
        BitArrayRepresentation {
            wrapped: self.wrapped.get_intermediary(filter.wrapped),
        }
    }

    fn to_solved_lattice(&self, solution: &BitArraySolution) -> Self {
        Self {
            wrapped: self.wrapped.to_solved_lattice(&solution.wrapped),
        }
    }

    pub fn export(&self, path: OsString, name: String) {
        self.wrapped.export(path, name);
    }

    fn export_to_ase(&self) {
        self.wrapped.export_to_ase();
    }

    fn no_rings(&self) -> SiteFilter {
        SiteFilter{wrapped: self.wrapped.no_rings()}
    }
}

#[pyfunction]
#[pyo3(signature = (filename, distance_margin, autodetect_margin=true))]
/// Put a new layer on top of an existing structure, calculated with DFT.
fn from_dft_json(filename: String, distance_margin: f32, autodetect_margin: bool) -> Lattice {
    Lattice {
        wrapped: WrappedLattice::from_dft_json(filename, distance_margin, autodetect_margin)
    }
}

#[pyfunction]
/// Test the import of the library
fn test_module() {
    println!("It works on my computer!")
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "lattice_solver_python")]
fn lattice_solver(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(test_module, m)?)?;
    m.add_function(wrap_pyfunction!(from_dft_json, m)?)?;
    m.add_class::<Lattice>()?;
    m.add_class::<BitArrayRepresentation>()?;
    m.add_class::<BitArraySolution>()?;
    Ok(())
}
