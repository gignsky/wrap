# Wrap - Rust CLI Tarball Utility

Wrap is a command-line utility written entirely in Rust that creates tarballs from folders in the current working directory and optionally removes the folders that created those tarballs.

Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.

## Working Effectively

### Required Tools
- Rust toolchain (stable channel) - Check with `rustc --version` and `cargo --version`
- Optional: Nix with flakes support for full development environment

### Build and Test the Repository
**NEVER CANCEL these commands - they complete quickly:**

1. **Build debug version:**
   - `cargo build` - takes ~30 seconds on first run, ~1-2 seconds on subsequent runs
   - Produces binary at `./target/debug/wrap`

2. **Build release version:**
   - `cargo build --release` - takes ~10 seconds on first run, ~1-2 seconds on subsequent runs  
   - Produces optimized binary at `./target/release/wrap`

3. **Run tests:**
   - `cargo test` - takes <1 second (currently no tests exist)

4. **Run linting:**
   - `cargo clippy` - takes ~3 seconds, shows format warnings but builds successfully
   - `cargo fmt --check` - takes <1 second, checks code formatting

### Running the Application
**Use the release binary for testing to ensure optimal performance:**
- `./target/release/wrap --help` - Show help and usage information
- `./target/release/wrap --version` - Show version (currently 0.1.4-patch-0.2)
- `./target/release/wrap -v` - Run with verbose output in current directory
- `./target/release/wrap --dry-run -v` - Preview what would be tarballed without creating files
- `./target/release/wrap --remove -v` - Create tarballs and remove source folders
- `./target/release/wrap [TARGET_DIR]` - Run on specific directory instead of current

## Validation and Testing

### Manual Validation Steps
**ALWAYS test functionality after making changes with these complete scenarios:**

1. **Basic tarball creation:**
   ```bash
   # Setup test environment
   mkdir -p /tmp/wrap-test/{folder1,folder2,folder3}
   echo "test content 1" > /tmp/wrap-test/folder1/file1.txt
   echo "test content 2" > /tmp/wrap-test/folder2/file2.txt
   echo "test content 3" > /tmp/wrap-test/folder3/file3.txt
   cd /tmp/wrap-test
   
   # Test dry run
   ./path/to/wrap --dry-run -v
   
   # Test actual creation
   ./path/to/wrap -v
   
   # Verify tarballs exist and contain correct content
   ls -la *.tar
   tar -tf folder1.tar
   ```

2. **Remove functionality:**
   ```bash
   # Continuing from above setup
   rm *.tar  # Remove previous tarballs
   ./path/to/wrap --remove -v
   
   # Verify folders were removed and tarballs created
   ls -la  # Should show only .tar files, no folders
   ```

3. **Edge cases:**
   ```bash
   # Test empty directory
   mkdir -p /tmp/empty-test && cd /tmp/empty-test
   ./path/to/wrap -v  # Should show empty hashmap and complete successfully
   
   # Test nonexistent target directory
   ./path/to/wrap /nonexistent  # Should panic with clear error message
   ```

### Pre-commit Validation
**ALWAYS run these before committing changes:**
- `cargo build --release` - Ensure code compiles
- `cargo clippy` - Check for linting issues (warnings are acceptable)
- `cargo fmt` - Auto-format code
- Manual functionality test with the validation scenarios above

## Development with Nix (Optional)

If Nix is available, you can use the full development environment:

1. **Enter development shell:**
   - `nix develop` - Sets up complete development environment

2. **Use justfile commands:**
   - `just` - Show all available commands
   - `just build` - Build with Nix (produces `./result/bin/wrap`)
   - `just run --help` - Run the application with arguments
   - `just fmt` - Auto-format using treefmt
   - `just check` - Run Nix flake check

## Project Structure

### Key Files and Directories
```
.
├── src/main.rs              # Single source file with all application logic  
├── Cargo.toml               # Rust project configuration and dependencies
├── Cargo.lock               # Locked dependency versions
├── justfile                 # Task runner commands (Nix environment)
├── flake.nix                # Nix flake configuration
├── rust-toolchain.toml      # Rust toolchain specification (stable)
├── .github/workflows/       # CI/CD workflows
│   ├── ci-nix.yml          # Main CI using Nix and omnix
│   ├── check.yml           # Cargo check workflow  
│   └── ...                 # Other workflows (docs, release, etc.)
└── nix/modules/            # Nix configuration modules
    ├── rust.nix            # Rust build configuration
    ├── formatting.nix      # Treefmt formatting setup
    └── ...
```

### Dependencies
- **clap 4.5**: Command-line argument parsing with derive macros
- **tar 0.4**: Tarball creation and manipulation

## Common Tasks

### Adding New CLI Options
1. Modify the `Args` struct in `src/main.rs` 
2. Add clap derive attributes for the new option
3. Update the `main()` function to pass the new option to relevant functions
4. Test with `--help` to verify option appears correctly

### Error Handling
- The application uses `panic!` for unrecoverable errors (e.g., nonexistent target directory)
- File operations use `.unwrap()` for simplicity - consider this when making changes
- The `remove_dir` function has robust retry logic for busy/permission-denied scenarios

### Build Timing Expectations
- **First build**: ~30 seconds (debug), ~10 seconds (release)
- **Incremental builds**: ~1-2 seconds
- **Clippy linting**: ~3 seconds  
- **Testing**: <1 second (no tests currently exist)
- **Formatting check**: <1 second

## CI/CD Information
- **Main CI**: `.github/workflows/ci-nix.yml` using Nix and omnix tool
- **Cargo CI**: `.github/workflows/check.yml` using standard Rust toolchain
- CI runs on: main, develop, release/*, hotfix/*, bugfix/* branches and all PRs
- Release process: Automated via `.github/workflows/release.yml`

## Tips
- Application works on any directory with folders - creates `.tar` files for each subdirectory
- Use `--dry-run` flag extensively when testing to avoid creating unwanted files
- Verbose mode (`-v`) is essential for debugging and understanding application behavior
- The binary name in builds is always `wrap` regardless of the package name
- Use release builds for performance testing - debug builds are significantly slower for large directories