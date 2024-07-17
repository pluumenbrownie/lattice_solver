Crystacean is a library and cli program, which can generate different amourphous silicon dioxide structures on a substrate. The library can be used in both Rust and Python. 

# Installation
To use Crystacean, you need to install the required dependencies:
1. Clone the repository.
1. Install the Rust compiler with [Rustup](https://www.rust-lang.org/tools/install).
1. Create a python virtual environment with your tool of choice, containing the requirements in `requirements.txt`.
1. Change directory to `lattice_solver_python`.
1. Create the Python module by running
```bash
$ maturin develop --release
```
This makes it possible to use both the Python library and the cli.

# Usage
## CLI
The Crystacean cli can be accessed by activating the right python virtual environment and running
```bash
$ python cli.py <command>
```
when located in the directory of the cloned repository, or in any other locations by running
```bash
$ python /path/to/cli.py <command>
```
To see a list of available commands, run
```bash
$ python cli.py --help
```

### Quickstart
I want to find possible surface structures for the `T16.json` lattice in the `test_lattices` directory:
```bash
$ python cli.py from-file -f -d 0.1 -m 0 -j -s exports/T16_example test_lattices/T16.json example
```
This command will take the structure from `exports/T16_example` and find structures, which will be filtered by similarity within 0.1 Angstrom, and which contain no singlet points. These structures will be called `example_<number>` and will be saved in an ASE readable json format. For more information on cli arguments and options, run
```bash
$ python cli.py <command> --help
```

### ASE readable json formate
Crystacean can read json files containing crystal structures which have been generated by ASE. These files can be created with ASE by converting more conventional file formats like `xyz` or `vasp` to `json` using 
```bash
$ ase convert <name>.<format> <name>.json
```
Crystacean can use these files to find interface configurations and create new files with the found configurations by adding atoms at the correct locations. The file must follow a few criteria:
1. The attachment sites are marked with hydrogen atoms.
1. The attachment sites are below `z = 20.0`.