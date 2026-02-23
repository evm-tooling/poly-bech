//go:build ignore

/*
The ci command is called from Continuous Integration scripts.

Usage: go run build/ci.go <command>

Available commands are:

	fmt     -- checks Rust formatting via just
	lint    -- runs Clippy with warnings as errors via just
	test    -- runs the test suite via just
	check   -- runs all checks (fmt + lint + test) via just
*/
package main

import (
	"log"
	"os"
	"os/exec"
	"path/filepath"
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

// doFmt checks Rust formatting via just.
func doFmt() {
	mustRun("just", "dev", "fmt", "check")
}

// doLint runs Clippy with warnings treated as errors via just.
func doLint() {
	mustRun("just", "dev", "lint")
}

// doTest runs the test suite via just.
func doTest() {
	mustRun("just", "dev", "test")
}

// doCheck runs all checks: fmt, lint, and test.
func doCheck() {
	doFmt()
	doLint()
	doTest()
}
