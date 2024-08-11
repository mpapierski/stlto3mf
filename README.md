# STL to 3MF Converter

![Crates.io Version](https://img.shields.io/crates/v/stlto3mf)

`stlto3mf` is a command line tool written in Rust that merges multiple STL files representing a single object into a single 3MF file. This is especially useful for multi-material 3D printing, saving time and effort when dealing with slicer programs.

## Features

- Merge multiple STL files into a single 3MF file.
- Generate a generic 3MF file compatible with any slicer program or 3D viewer that supports the 3MF format.

## Requirements

- Rust (for building from source)

## Installation

To build from source, you need to have Rust installed. Clone the repository and build the project:

```sh
cargo install stlto3mf
```

Or from source:

```sh
git clone https://github.com/mpapierski/stlto3mf.git
cd stlto3mf
cargo run
# or
cargo install --path '.'
```

The executable will be available in the target/release directory (or in your $PATH)

## Usage

```sh
stlto3mf [OPTIONS] --output <OUTPUT> [STL_FILES]...
```

### Arguments

- `[STL_FILES]...`

  List of STL files to merge.

### Options

* `--output <OUTPUT>`
  Output filename for the 3MF file.

* `--name <NAME>`
  Name of a group of STLs in the final .3MF file.

* `-h, --help`
 Print help information.

* `-V, --version`
 Print version information.

# Examples

## Basic Usage

Merge multiple STL files into a single 3MF file with a specified output name:

```sh
stlto3mf --name final_object --output output.3mf part1.stl part2.stl part3.stl
```

This command will merge part1.stl, part2.stl, and part3.stl into a single output.3mf file, grouping them under final_object.

## Help and Version

To print the help information:

```sh
stlto3mf --help
```

To print the version information:

```sh
stlto3mf --version
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request for any improvements or bug fixes.
License

## License

This project is licensed under the MIT License. See the LICENSE file for details.

# Contact

For any questions or support, please open an issue on the GitHub repository or contact [michal@papierski.net](michal@papierski.net).
