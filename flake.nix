{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    flake-parts.url = "github:hercules-ci/flake-parts";

    pre-commit.url = "github:cachix/pre-commit-hooks.nix";
  };

  outputs = {
    # self,
    flake-parts,
    ...
  } @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      imports = [
        inputs.pre-commit.flakeModule
      ];

      systems = ["x86_64-linux"];
      perSystem = {
        config,
        system,
        inputs',
        self',
        lib,
        ...
      }: let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
          ];
        };
      in {
        checks = {
        };

        pre-commit = {
          settings = {
          };
        };

        packages = {
          # default = site-server;
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            bashInteractive
            just # command recipes
            nodejs
            nodejs.pkgs.pnpm # this line changed
          ];
          shellHook = ''
            # {config.pre-commit.installationScript}
          '';
        };
      };
    };
}
