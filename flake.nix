{
  description = "nobbz.dev - Website";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    d2n.url = "github:nix-community/dream2nix";
    d2n.inputs.all-cabal-json.follows = "nixpkgs";
    flake-parts.url = "github:hercules-ci/flake-parts";
    nix-filter.url = "github:numtide/nix-filter";
    obsidian-export = {
      url = "github:zoni/obsidian-export/v22.11.0";
      flake = false;
    };
    obsidian-second-brain = {
      url = "git+ssh://git@git.brianryall.xyz/polarmutex/obsidian-second-brain.git";
      flake = false;
    };
    pre-commit.url = "github:cachix/pre-commit-hooks.nix";
  };

  outputs = {
    self,
    crane,
    d2n,
    nixpkgs,
    flake-parts,
    pre-commit,
    ...
  } @ inputs: let
    systems = ["x86_64-linux" "aarch64-linux" "aarch64-darwin"];
  in
    flake-parts.lib.mkFlake {inherit inputs;} {
      inherit systems;

      imports = [
        d2n.flakeModuleBeta
        pre-commit.flakeModule
        ./nix/site.nix
        ./nix/hooks.nix
      ];

      #dream2nix.config.projectRoot = ./.;

      perSystem = {
        config,
        pkgs,
        self',
        inputs',
        system,
        ...
      }: {
        packages = {
          obsidian-export = let
            craneLib = crane.lib.${system};
          in
            craneLib.buildPackage {
              #src = craneLib.cleanCargoSource inputs.obsidian-export;
              src = inputs.obsidian-export;
              buildInputs = [
                # Add additional build inputs here
              ];
              doChek = false;
              doInstallCheck = false;
            };
        };

        devShells.default = pkgs.mkShell {
          packages = builtins.attrValues {
            inherit (pkgs) nodejs;
            inherit (config.packages) obsidian-export;
          };
          shellHook = config.pre-commit.installationScript;
        };
      };
    };
}
