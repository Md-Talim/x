# x

The fastest way to run single code files. Just `x filename.ext`.

`x` is a minimal CLI tool for running single code files directly from the terminal. Perfect for quick scripts, DSA problems, competitive programming, or experiments without IDE overhead.

## Features

- **Ultra-minimal**: Just 1 character to type - `x`
- **Multi-language**: Supports Kotlin, Java, C++ (more coming)
- **Single file focus**: No project setup, just run individual files
- **Auto-cleanup**: Removes generated files after execution

## Supported Languages

| Language | Extensions    | Requirements      |
| -------- | ------------- | ----------------- |
| Kotlin   | `.kt`         | `kotlinc`, `java` |
| Java     | `.java`       | `javac`, `java`   |
| C++      | `.cpp`, `.cc` | `g++`             |

## Installation

```bash
go install github.com/md-talim/x@latest
```

## Usage

```bash
x main.kt         # Kotlin
x Solution.java   # Java
x algorithm.cpp   # C++
```

## Requirements

- **Go** (for installation)
- **Language compilers**: `kotlinc`/`java`, `javac`/`java`, `g++`

Make sure compilers are in your PATH.

## Future Plans

- **More languages**: Python, Rust, Go, JavaScript, TypeScript
- **Multi-file support**: Run projects with multiple source files
- **That's it**: No complex features, just fast single-file execution

## Why x?

This tool does one thing well: quickly run single code files. For complex project management, build systems, or scaffolding - use proper tools or a future `dash` CLI.

`x` = execute. Simple.
