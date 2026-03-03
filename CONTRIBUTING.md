# Contributing to openapi-ui

Thank you for considering contributing to openapi-ui! This document provides guidelines and instructions for contributing.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the existing issues as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

* **Use a clear and descriptive title**
* **Describe the exact steps to reproduce the problem**
* **Provide specific examples to demonstrate the steps**
* **Describe the behavior you observed and what behavior you expected**
* **Include screenshots if possible**
* **Include Rust version and openapi-ui version**

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

* **Use a clear and descriptive title**
* **Provide a detailed description of the suggested enhancement**
* **Explain why this enhancement would be useful**
* **List some examples of how this enhancement would be used**

### Pull Requests

* Fill in the required template
* Follow the Rust style guide (use `cargo fmt`)
* Include tests if applicable
* Update documentation as needed
* Ensure all tests pass (`cargo test`)
* Ensure clippy warnings are resolved (`cargo clippy`)

## Development Setup

### Prerequisites

* Rust 1.70 or later
* Cargo

### Setting Up

```bash
# Clone the repository
git clone https://github.com/kimolalekan/openapi-ui.git
cd openapi-ui

# Build the project
cargo build

# Run tests
cargo test

# Run clippy
cargo clippy

# Format code
cargo fmt
```

## Coding Guidelines

### Code Style

* Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
* Use `cargo fmt` to format code before committing
* Run `cargo clippy` to catch common mistakes

### Documentation

* Document all public items with rustdoc comments
* Include examples in documentation where helpful
* Keep README.md up to date

### Testing

* Write tests for new features
* Ensure existing tests pass
* Include both unit tests and integration tests where appropriate

## Release Process

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md (if exists)
3. Create a git tag
4. Push to GitHub
5. Publish to crates.io

## Questions?

Feel free to open an issue for any questions or concerns.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
