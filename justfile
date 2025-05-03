default:
    @just --list

# Auto-format the source tree
fmt:
    treefmt

# Run 'cargo run' on the project
run *ARGS:
    just dont-fuck-my-build
    cargo run {{ARGS}}

# Run 'cargo watch' to run the project (auto-recompiles)
watch *ARGS:
    cargo watch -x "run -- {{ARGS}}"

show:
    just dont-fuck-my-build
    om show .

dont-fuck-my-build:
    git ls-files --others --exclude-standard -- '*.nix' | xargs -r git add -v | lolcat
    echo "No chance your build is fucked! 👍" | lolcat

# Run pre-commit hooks on all files, including autoformatting
pre-commit-all:
    pre-commit run --all-files

om *ARGS:
    nix run github:juspay/omnix -- {{ ARGS }}

show:
    just dont-fuck-my-build
    just om show .

health:
    just dont-fuck-my-build
    just om health .

clean:
    rm -rfv results
    cargo clean

update:
	just dont-fuck-my-build
	nix flake update --commit-lock-file

build:
    nix build
    quick-results

check *ARGS:
    just dont-fuck-my-build
    nix flake check --impure --no-build {{ ARGS }}
    nix-shell -p lolcat --run 'echo "[CHECK] Finished." | lolcat'
