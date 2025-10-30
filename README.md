# ktrun

`ktrun` is a small CLI tool that lets you run Kotlin files directly from the terminal. It's built for people who want to use kotlin for quick scripts, DSA problems, or experiments without opening an IDE or setting up Gradle.

## Features

- Run a single `.kt` file directly from the terminal

## Future Plans

This project is more personal right now, but I plan to add:

- Support for multiple files and simple project structure
- Commands like `ktrun run .` or `ktrun init` similar to Go
- Faster build and execution options

## Installation

Make sure you have Go and Kotlin installed, then run:

```bash
go install github.com/md-talim/ktrun@latest
```

## Usage

Run a Kotlin file:

```bash
ktrun main.kt
```

## Requirements

- Go
- Kotlin CLI (`kotlinc` and `kotlin` available in PATH)
