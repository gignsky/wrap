{ inputs, ... }: {
  imports = [ (inputs.git-hooks + /flake-module.nix) ];
  perSystem = { config, self', pkgs, lib, ... }: {
    pre-commit.settings = {
      hooks = {
        nixfmt-classic.enable = true;
        rustfmt.enable = true;
      };
    };
  };
}
