# `x`

The fastest way to run single code files. Just `x filename.ext`.

`x` is a minimal CLI tool for compiling and running single source files directly from the terminal. Perfect for quick scripts, DSA problems, competitive programming, or experiments - no project setup or IDE needed.

## Features

- **Ultra-minimal**: Just 1 character to type - `x`
- **Multi-language**: Supports Kotlin, Java, C/C++, Python (more coming)
- **Single file focus**: No project setup, just run individual files
- **Auto-cleanup**: Removes generated files after execution
- **Fast**: Written in Rust, starts instantly

## Supported Languages

| Language | Extensions    | Requirements      |
| -------- | ------------- | ----------------- |
| C        | `.c`          | `gcc`             |
| C++      | `.cpp`, `.cc` | `g++`             |
| Java     | `.java`       | `javac`, `java`   |
| Kotlin   | `.kt`         | `kotlinc`, `java` |
| Python   | `.py`         | `python`          |

The relevant compiler/runtime must be installed and available in your `PATH`:

## Installation

### From Source (requires [Rust](https://rustup.rs/))

```sh
# Install directly from GitHub (no clone needed)
cargo install --git https://github.com/Md-Talim/x

# Or clone and install locally
git clone https://github.com/Md-Talim/x.git
cd x
cargo install --path .
```

## Usage

```bash
x main.c          # C
x algorithm.cpp   # C++
x Solution.java   # Java
x main.kt         # Kotlin
x hello.py        # Python
```

## Why x?

`x` = execute. One character. No friction. Run your code.
