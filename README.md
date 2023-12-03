# factor-win

[![License](https://img.shields.io/badge/license-Apache%202.0%20%2F%20MIT-blue.svg)](LICENSE_APACHE.md)

A Windows port of the `factor` program.

## Description

`factor-win` is a Rust-based utility designed for factoring numbers on the Windows platform. It takes a list of numbers as input and provides their prime factors as output. Optionally, it can display repeated factors in the form p^e unless e is 1.

## Features

- Factorization of numbers into prime factors.
- Optional display of repeated factors in exponent form.

## Installation

1. Clone the repository:

```bash
git clone https://github.com/your_username/factor-win.git
```

2. Navigate to the project directory:

```bash
cd factor-win
```

3. Build and install the executable:

```bash
cargo build --release
```

The compiled binary will be available in the `target/release` directory.

## Usage

```bash
factor-win [OPTIONS] <numbers>...
```

- `numbers`: The list of numbers to factorize.

Options:

- `-h, --exponents`: Print repeated factors in the form p^e unless e is 1.

### Examples

```bash
# Factorize a single number
factor-win 36

# Factorize multiple numbers
factor-win 36 48 72

# Factorize with exponent notation
factor-win -h 72 36
```

## Support

If you encounter any issues or have questions, please feel free to open an [issue](https://github.com/walker84837/factor-win-rs/issues).

## Roadmap

- [ ] Performance optimizations for large numbers (multi-threading).

## Contributing

Contributions are welcome! If you would like to contribute, please follow the guidelines in [CONTRIBUTING.md](CONTRIBUTING.md).

## Authors and Acknowledgment

- Author: [walker84837](https://github.com/walker84837) - Sole mantainer
- Acknowledgments: None at the moment.

## License

This project is dual-licensed under [Apache 2.0](LICENSE_APACHE.md) or [MIT](LICENSE_MIT.md) licenses.

## Project Status

Development is not very active, and contributions are welcome. If you are interested in becoming a maintainer, please reach out.
