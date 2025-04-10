package main

import (
	"context"
	"fmt"
	"os"

	"dagger.io/dagger"
)

func main() {
	ctx := context.Background()

	// Initialize Dagger client
	client, err := dagger.Connect(ctx, dagger.WithLogOutput(os.Stdout))
	if err != nil {
		fmt.Println("Failed to connect to Dagger:", err)
		os.Exit(1)
	}
	defer client.Close()

	// Get reference to project directory
	src := client.Host().Directory(".")

	// Create Rust container with mounted source
	rust := client.Container().
		From("rust:latest").
		WithMountedDirectory("/src", src).
		WithWorkdir("/src")

	// Run checks with caching
	fmt.Println("Running cargo fmt...")
	_, err = rust.WithExec([]string{"cargo", "fmt", "--check"}).Stdout(ctx)
	if err != nil {
		fmt.Println("❌ Formatting check failed:", err)
		os.Exit(1)
	}

	fmt.Println("Running cargo clippy...")
	_, err = rust.WithExec([]string{"cargo", "clippy", "--all-targets", "--all-features", "--", "-D", "warnings"}).Stdout(ctx)
	if err != nil {
		fmt.Println("❌ Linting failed:", err)
		os.Exit(1)
	}

	fmt.Println("Running cargo test...")
	_, err = rust.WithExec([]string{"cargo", "test"}).Stdout(ctx)
	if err != nil {
		fmt.Println("❌ Tests failed:", err)
		os.Exit(1)
	}

	fmt.Println("✅ All checks passed!")
}