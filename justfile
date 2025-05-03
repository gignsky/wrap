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
	just om show .

dont-fuck-my-build:
	git ls-files --others --exclude-standard -- '*.nix' | xargs -r git add -v | lolcat
	echo "No chance your build is fucked! 👍" | lolcat

om *ARGS:
	nix run github:juspay/omnix -- {{ ARGS }}

health:
	just dont-fuck-my-build
	just om health .

clean:
	rm -rfv result
	cargo clean

update:
	just dont-fuck-my-build
	just cargo-update
	nix flake update --commit-lock-file

update-no-commit:
	just dont-fuck-my-build
	just cargo-update-no-commit
	nix flake update

cargo-update-no-commit:
	cargo update

cargo-update:
	cargo update
	git add Cargo.lock
	git commit -m "updated Cargo.lock dependecies via justfile (just cargo-update)"
	
build *ARGS:
	nix build {{ ARGS }}
	quick-results

check *ARGS:
	just dont-fuck-my-build
	nix flake check --impure --no-build {{ ARGS }}
	nix-shell -p lolcat --run 'echo "[CHECK] Finished." | lolcat'
