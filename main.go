package main

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)

func runKotlin(file string) {
	out := "out.jar"

	// Compile
	compileCmd := exec.Command("kotlinc", file, "-include-runtime", "-d", out)
	compileCmd.Stdout = os.Stdout
	compileCmd.Stderr = os.Stderr
	if err := compileCmd.Run(); err != nil {
		fmt.Println("Compilation failed:", err)
		os.Exit(1)
	}

	// Run
	runCmd := exec.Command("java", "-jar", out)
	runCmd.Stdout = os.Stdout
	runCmd.Stderr = os.Stderr
	if err := runCmd.Run(); err != nil {
		fmt.Println("Execution failed:", err)
		os.Exit(1)
	}

	os.Remove(out)
}

func runCpp(file string) {
	// Compile
	compileCmd := exec.Command("g++", file, "-o", "main")
	compileCmd.Stdout = os.Stdout
	compileCmd.Stderr = os.Stderr
	if err := compileCmd.Run(); err != nil {
		fmt.Println("Compilation failed:", err)
		os.Exit(1)
	}

	// Run
	runCmd := exec.Command("./main")
	runCmd.Stdout = os.Stdout
	runCmd.Stderr = os.Stderr
	if err := runCmd.Run(); err != nil {
		fmt.Println("Execution failed:", err)
		os.Exit(1)
	}

	os.Remove("main")
}

func runJava(file string) {
	baseName := filepath.Base(file)
	className := strings.TrimSuffix(baseName, filepath.Ext(baseName))

	// Compile
	compileCmd := exec.Command("javac", file)
	compileCmd.Stdout = os.Stdout
	compileCmd.Stderr = os.Stderr
	if err := compileCmd.Run(); err != nil {
		fmt.Println("Compilation failed:", err)
		os.Exit(1)
	}

	// Run
	runCmd := exec.Command("java", className)
	runCmd.Stdout = os.Stdout
	runCmd.Stderr = os.Stderr
	if err := compileCmd.Run(); err != nil {
		fmt.Println("Execution failed:", err)
		os.Exit(1)
	}

	os.Remove(className + ".class")
}

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: x <file.kt|file.java|file.cpp>")
		os.Exit(1)
	}

	file := os.Args[1]
	ext := filepath.Ext(file)

	switch ext {
	case ".kt":
		runKotlin(file)
	case ".cpp", ".cc":
		runCpp(file)
	case ".java":
		runJava(file)
	default:
		fmt.Println("File not supported")
		os.Exit(1)
	}
}
