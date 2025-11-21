# ktrun

`ktrun` is a versatile CLI tool that lets you run code files directly from the terminal. Originally built for Kotlin, it now supports multiple programming languages, making it perfect for quick scripts, DSA problems, competitive programming, or experiments without opening an IDE or setting up complex build systems.

## Features

- **Multi-language support**: Run `.kt` (Kotlin), `.java` (Java), `.cpp/.cc` (C++) files directly
- **Simple workflow**: Compile and run in one command
- **Clean execution**: Automatically cleans up generated files after execution
- **Lightweight**: No complex configuration or project setup required

## Supported Languages

| Language | File Extensions | Requirements          |
| -------- | --------------- | --------------------- |
| Kotlin   | `.kt`           | `kotlinc`, `java`     |
| Java     | `.java`         | `javac`, `java` (JDK) |
| C++      | `.cpp`, `.cc`   | `g++`                 |

## Installation

Make sure you have Go installed, then run:

```bash
go install github.com/md-talim/ktrun@latest
```

## Usage

Run any supported file:

```bash
# Kotlin
ktrun main.kt

# Java
ktrun Solution.java

# C++
ktrun algorithm.cpp
```

## Requirements

### Core

- Go (for installation)

### Language-specific

- **Kotlin**: Kotlin CLI (`kotlinc`) and Java Runtime (`java`)
- **Java**: Java Development Kit (JDK) with `javac` and `java`
- **C++**: GCC compiler (`g++`)

Make sure the required compilers and runtimes are available in your system PATH.

## Future Plans

This project is continuously evolving! Planned features include:

### Near-term

- **Python support** (`.py` files)
- **Rust support** (`.rs` files)
- **Go support** (`.go` files) - ironic, right? 😄

### Medium-term

- Support for multiple files and simple project structures
- Configuration file support for custom compile flags
- Package/dependency management for supported languages
- Faster build and execution options with caching

### Long-term

- Commands like `ktrun run .` or `ktrun init` similar to Go CLI
- Language-specific templates and scaffolding
- Integration with popular online judges
- Plugin system for custom language support

## Contributing

Feel free to open issues or submit pull requests if you'd like to see support for additional languages or features!
