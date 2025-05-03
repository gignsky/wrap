{ inputs, ... }:
{
  perSystem = { config, self', pkgs, lib, ... }: {
    devShells.default = pkgs.mkShell {
      name = "recursive-tarballs-shell";
      inputsFrom = [
        self'.devShells.rust
        config.treefmt.build.devShell
        config.pre-commit.devShell # See ./nix/modules/pre-commit.nix
      ];
      packages = with pkgs; [
        nixd # Nix language server
        cargo-watch
        config.process-compose.cargo-doc-live.outputs.package
        nil
        bacon
        wslu
        lolcat
        # cargo-generate

        # dotfiles programs
        inputs.dotfiles.packages.${system}.quick-results
        inputs.dotfiles.packages.${system}.upjust
        inputs.dotfiles.packages.${system}.cargo-update
      ];
    };
  };
}
