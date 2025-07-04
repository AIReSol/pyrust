# Rust-Python Integration with PyO3

This project demonstrates how to integrate Rust code with Python using PyO3, allowing you to write high-performance extensions in Rust that can be called from Python.

## Overview

**High-level concept**: PyO3 is a Rust crate that provides bindings for Python, allowing you to create Python modules written in Rust. This enables you to leverage Rust's performance and safety features while maintaining Python's ease of use and ecosystem.

## Project Structure

```
pyrust/
├── Cargo.toml          # Rust package configuration
├── pyproject.toml      # Python package configuration
├── setup.py            # Alternative Python setup (legacy)
├── src/
│   └── lib.rs          # Rust source code with Python bindings
├── example.py          # Python script demonstrating usage
└── README.md           # This file
```

## Features Demonstrated

1. **Simple Functions**: Basic Rust functions callable from Python
2. **Data Processing**: Functions that work with Python lists/vectors
3. **Classes/Structs**: Rust structs exposed as Python classes
4. **Performance**: Comparison between Python and Rust implementations

## Prerequisites

- **Rust**: Install from [rustup.rs](https://rustup.rs/)
- **Python 3.8+**: Any recent Python version
- **Maturin**: Python package for building Rust extensions

## Setup Instructions

### 1. Install Maturin

```bash
pip install maturin
```

### 2. Build the Extension

From the `pyrust` directory:

```bash
# Development build (creates extension in current environment)
maturin develop

# Or for release build
maturin develop --release
```

### 3. Test the Integration

```bash
python example.py
```

## License
MIT License - feel free to use this as a template for your own projects!