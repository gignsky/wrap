# Wrapd - Rust CLI Tarball Utility

Wrapd is a command-line utility written entirely in Rust that creates tarballs from folders in the current working directory and optionally removes the folders that created those tarballs.

**Always reference these instructions first and fallback to search or bash commands only when you encounter unexpected information that does not match the info here.**

---

## Working Effectively

### Required Tools

- **Nix** with flakes support (primary workflow)
- **Rust toolchain** (stable channel) — Check with `rustc --version` and `cargo --version`

---

## Development Workflow

### 1. Enter the Development Environment

All development should be done inside the Nix flakes environment to guarantee reproducibility and correct dependencies.

    nix develop

This command will drop you into a shell with all necessary tools and environment configured.

---

### 2. Building and Validating with Nix

**Show available outputs:**

    nix flake show

Lists all buildable packages and outputs defined in `flake.nix`.

**Build all outputs:**

    nix build

**Build a specific package (e.g., the wrap CLI):**

    nix build .#wrap

Resulting binary is at `./result/bin/wrap`.

**Run all flake checks (tests, lints, etc.):**

    nix flake check

**Format code (if enabled):**

    nix fmt
    # or, if defined:
    nix run .#fmt

---

### 3. Parallel Cargo Workflow (Inside Nix Shell)

The cargo workflow can be run in parallel inside the Nix environment. This is useful for quick local builds and checks, but always validate with Nix before committing.

**Build debug version:**

    cargo build

Produces binary at `./target/debug/wrap`

**Build release version:**

    cargo build --release

Produces optimized binary at `./target/release/wrap`

**Run tests:**

    cargo test

**Linting:**

    cargo clippy
    cargo fmt --check

---

### 4. Running the Application

**Use the release binary for testing to ensure optimal performance:**

- `./target/release/wrap --help` (cargo build)
- `./result/bin/wrap --help` (nix build)

**Other usage examples:**

- `./target/release/wrap --version`
- `./target/release/wrap -v`
- `./target/release/wrap --dry-run -v`
- `./target/release/wrap --remove -v`
- `./target/release/wrap [TARGET_DIR]`

---

## Validation and Testing

### Manual Validation Steps

**Always test functionality after making changes with these scenarios:**

1. **Basic tarball creation:**

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

2. **Remove functionality:**

        # Continuing from above setup
        rm *.tar  # Remove previous tarballs
        ./path/to/wrap --remove -v

        # Verify folders were removed and tarballs created
        ls -la  # Should show only .tar files, no folders

3. **Edge cases:**

        # Test empty directory
        mkdir -p /tmp/empty-test && cd /tmp/empty-test
        ./path/to/wrap -v  # Should show empty hashmap and complete successfully

        # Test nonexistent target directory
        ./path/to/wrap /nonexistent  # Should panic with clear error message

---

### Pre-commit Validation

**Always run these before committing changes:**

- `nix build .#wrap` — Ensure Nix build succeeds
- `nix flake check` — Ensure all Nix checks pass
- `cargo build --release` — Ensure cargo build works (inside nix develop)
- `cargo clippy` — Check for linting issues (warnings OK)
- `cargo fmt` — Auto-format code
- Manual functionality test with the validation scenarios above

---

## Project Structure

    .
    ├── src/main.rs              # Single source file with all application logic  
    ├── Cargo.toml               # Rust project configuration and dependencies
    ├── Cargo.lock               # Locked dependency versions
    ├── justfile                 # Task runner commands (legacy, use Nix directly)
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

---

## Key Dependencies

- **clap 4.5**: Command-line argument parsing with derive macros
- **tar 0.4**: Tarball creation and manipulation

---

## Common Tasks

### Adding New CLI Options

1. Modify the `Args` struct in `src/main.rs`
2. Add clap derive attributes for the new option
3. Update the `main()` function to pass the new option to relevant functions
4. Test with `--help` to verify option appears correctly

### Error Handling

- The application uses `panic!` for unrecoverable errors (e.g., nonexistent target directory)
- File operations use `.unwrap()` for simplicity — consider this when making changes
- The `remove_dir` function has robust retry logic for busy/permission-denied scenarios

### Build Timing Expectations

- **First build:** ~30 seconds (debug), ~10 seconds (release)
- **Incremental builds:** ~1-2 seconds
- **Clippy linting:** ~3 seconds  
- **Testing:** <1 second (no tests currently exist)
- **Formatting check:** <1 second

---

## CI/CD Information

- **Main CI:** `.github/workflows/ci-nix.yml` using Nix and omnix tool
- **Cargo CI:** `.github/workflows/check.yml` using standard Rust toolchain
- CI runs on: main, develop, release/*, hotfix/*, bugfix/* branches and all PRs
- Release process: Automated via `.github/workflows/release.yml`

---

## Tips

- Application works on any directory with folders — creates `.tar` files for each subdirectory
- Use `--dry-run` flag extensively when testing to avoid creating unwanted files
- Verbose mode (`-v`) is essential for debugging and understanding application behavior
- The binary name in builds is always `wrap` regardless of the package name
- Use release builds for performance testing — debug builds are significantly slower for large directories

---

## Summary Table

| Task         | Nix Flake Command           | Cargo Command (in nix develop)   |
|--------------|-----------------------------|----------------------------------|
| Enter Shell  | `nix develop`               | —                                |
| Build        | `nix build`                 | `cargo build`                    |
| Build (Rel)  | `nix build .#wrap`          | `cargo build --release`          |
| Run Checks   | `nix flake check`           | `cargo test`, `cargo clippy`     |
| Format       | `nix fmt` / `nix run .#fmt` | `cargo fmt --check`              |

---

This guide ensures reproducible builds and checks using Nix flakes. For any questions, always refer to the sections above before searching elsewhere.
