# factorflow: a rust rewrite of factor

[![License](https://img.shields.io/badge/license-Apache%202.0%20%2F%20MIT-blue.svg)](LICENSE_APACHE.md)

A Rust rewrite of the `factor` program.

## Description

`factorflow` is a Rust rewrite of the `factor` command from the Coreutils. It
takes a list of numbers as input and provides their prime factors as output.
Optionally, it can display repeated factors in the form p^e unless e is 1, or
calculate the GCD and LCM of the list of numbers.

## Features

  - Factorization of numbers into prime factors.
  - Calculating GCD and LCM of numbers.
  - Optional display of repeated factors in exponent form.

## Installation

1.  Clone the repository:

``` console
$ git clone https://github.com/walker84837/factorflow-rs.git
```

2.  Navigate to the project directory:

``` console
$ cd factorflow-rs
```

3.  Build and install the executable:

``` console
$ cargo build --release
```

The compiled binary will be available in the `target/release` directory.

## Usage

``` console
factorflow-rs [OPTIONS] <numbers>...
```

  - `numbers`: The list of numbers to factorize.

Options:

  - `-h, --exponents`: Print repeated factors in the form p^e unless e is 1.
  - `-l`: Print the GCD and LCM of the numbers.

### Examples

``` console
$ factorflow-rs 36
36: 2 2 3 3

$ factorflow-rs -l 36 48 72
LCM: 144
GCD: 12

$ factorflow-rs -h 72 36
72: 2^3 3^2
36: 2^2 3^2
```

## Support

If you encounter any issues or have questions, please feel free to open an
[issue](https://github.com/walker84837/factorflow-rs/issues).

## Contributing

Contributions are welcome!

## Authors and acknowledgments

  - Author: [walker84837](https://github.com/walker84837) - Sole mantainer

## License

This project is dual-licensed under [Apache 2.0](LICENSE_APACHE.md) or
[MIT](LICENSE_MIT.md) licenses.

## Project status

Development is not very active, and contributions are welcome. If you are
interested in becoming a maintainer, please reach out.
