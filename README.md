# factorflow: a rust rewrite of factor

[![License](https://img.shields.io/badge/license-Apache%202.0%20%2F%20MIT-blue.svg)](LICENSE_APACHE.md)

A Rust rewrite of the well-known `factor` command.

## Installation

Clone the repository, navigate to the project directory, and build, and install
the executable:

``` console
$ git clone https://github.com/walker84837/factorflow-rs.git
$ cd factorflow-rs
$ cargo build --release
```

## Usage

``` console
factorflow [OPTIONS] <numbers>...
```

  - `numbers`: The list of numbers to factorise.

Options:

  - `-h, --exponents`: Print repeated factors in the form p^e unless e is 1.
  - `-p`: Print only the prime factors.
  - `-l`: Print the GCD and LCM of the numbers.

### Examples

``` console
$ factorflow 36
36: 2 2 3 3

$ factorflow -l 36 48 72
LCM: 144
GCD: 12

$ factorflow -h 72 36
72: 2^3 3^2
36: 2^2 3^2
```

## Support

If you encounter any issues or have questions, please feel free to open an
[issue](https://github.com/walker84837/factorflow-rs/issues).

## Contributing

Contributions are welcome\!

## Authors and acknowledgments

  - Author: [walker84837](https://github.com/walker84837) - Sole mantainer (for
    now)

## License

This project is dual-licensed under [Apache 2.0](LICENSE_APACHE.md) or
[MIT](LICENSE_MIT.md) licenses.

## Project status

Development is active, and contributions are welcome. If you are interested in
contributing, please reach out.
