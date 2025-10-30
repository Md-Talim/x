package main

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Println("Usage: ktrun <file.kt>")
		os.Exit(1)
	}

	file := os.Args[1]
	if filepath.Ext(file) != ".kt" {
		fmt.Println("Please provide a .kt file")
		os.Exit(1)
	}

	// Temporary output jar
	out := "out.jar"

	// Compile
	cmd := exec.Command("kotlinc", file, "-include-runtime", "-d", out)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	if err := cmd.Run(); err != nil {
		fmt.Println("Compilation failed:", err)
		os.Exit(1)
	}

	// Run
	run := exec.Command("java", "-jar", out)
	run.Stdout = os.Stdout
	run.Stderr = os.Stderr
	if err := run.Run(); err != nil {
		fmt.Println("Execution failed:", err)
		os.Exit(1)
	}

	// Clean up
	os.Remove(out)
}
