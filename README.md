# factorflow: a rust rewrite of factor

[![License](https://img.shields.io/badge/license-0BSD-blue.svg)](LICENSE)

[![No Maintenance Intended](http://unmaintained.tech/badge.svg)](http://unmaintained.tech/)

> A Rust rewrite of the well-known `factor` command.

**Disclaimer**: you should use the [uutils](https://uutils.github.io/coreutils/docs/utils/factor.html) version of this command, as I feel this project is complete enough for me to leave it as-is for a long time.

## Installation

Clone the repository, navigate to the project directory, and build, and install
the executable:

``` console
$ git clone https://github.com/walker84837/factorflow.git
$ cd factorflow
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

If you encounter any issues or have questions, please feel free to open an [issue](https://github.com/walker84837/factorflow/issues).

## Contributing

Contributions are welcome!

## Authors and acknowledgments

  - Author: [walker84837](https://github.com/walker84837)

## License

This project is licensed under the 0BSD license.

## Project status

Contributions are welcome. If you are interested in contributing, please reach out.
