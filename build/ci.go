//go:build ignore

/*
The ci command is called from Continuous Integration scripts.

Usage: go run build/ci.go <command>

Available commands are:

	fmt     -- checks Rust formatting via cargo +nightly fmt
	lint    -- runs Clippy with warnings as errors
	test    -- runs the test suite
	check   -- runs all checks (fmt + lint + test)
*/
package main

import (
	"fmt"
	"log"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)

func main() {
	log.SetFlags(log.Lshortfile)

	if _, err := os.Stat(filepath.Join("build", "ci.go")); os.IsNotExist(err) {
		log.Fatal("this script must be run from the root of the repository")
	}
	if len(os.Args) < 2 {
		log.Fatal("need subcommand as first argument")
	}

	switch os.Args[1] {
	case "fmt":
		doFmt()
	case "lint":
		doLint()
	case "test":
		doTest()
	case "check":
		doCheck()
	default:
		log.Fatal("unknown command ", os.Args[1])
	}
}

// runCommand executes a command and streams output to stdout/stderr.
func runCommand(name string, args ...string) error {
	cmd := exec.Command(name, args...)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	return cmd.Run()
}

// mustRun executes a command and exits on failure.
func mustRun(name string, args ...string) {
	if err := runCommand(name, args...); err != nil {
		log.Fatalf("command failed: %s %v: %v", name, args, err)
	}
}

// checkRustToolchain verifies that cargo and rustup are available,
// and that the nightly toolchain is installed for formatting.
func checkRustToolchain() {
	// Check for cargo
	if _, err := exec.LookPath("cargo"); err != nil {
		log.Fatal("cargo not found in PATH. Please install Rust: https://rustup.rs")
	}

	// Check for rustup (needed for +nightly)
	if _, err := exec.LookPath("rustup"); err != nil {
		log.Fatal("rustup not found in PATH. Please install rustup: https://rustup.rs")
	}

	// Check if nightly toolchain is installed
	cmd := exec.Command("rustup", "toolchain", "list")
	output, err := cmd.Output()
	if err != nil {
		log.Fatalf("failed to list rustup toolchains: %v", err)
	}

	if !strings.Contains(string(output), "nightly") {
		fmt.Println("==> Nightly toolchain not found. Installing...")
		mustRun("rustup", "toolchain", "install", "nightly")
		mustRun("rustup", "component", "add", "rustfmt", "--toolchain", "nightly")
	}
}

// doFmt checks Rust formatting using nightly rustfmt.
func doFmt() {
	fmt.Println("==> Checking Rust formatting...")
	mustRun("cargo", "+nightly", "fmt", "--all", "--", "--check")
	fmt.Println("==> Formatting check passed!")
}

// doLint runs Clippy with warnings treated as errors.
func doLint() {
	fmt.Println("==> Running Clippy...")
	mustRun("cargo", "clippy", "--all-targets", "--")
	fmt.Println("==> Lint passed!")
}

// doTest runs the test suite.
func doTest() {
	fmt.Println("==> Running tests...")
	mustRun("cargo", "test", "--all")
	fmt.Println("==> Tests passed!")
}

// doCheck runs all checks: fmt, lint, and test.
func doCheck() {
	fmt.Println("==> Running all checks...")
	checkRustToolchain()
	doFmt()
	doLint()
	doTest()
	fmt.Println("==> All checks passed!")
}
