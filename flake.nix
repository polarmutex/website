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
          default = self'.packages.app;
          app = let
            pnpm = pkgs.pnpm_10;
            packageJson = builtins.fromJSON (builtins.readFile ./package.json);
          in
            pkgs.stdenvNoCC.mkDerivation (finalAttrs: {
              pname = packageJson.name;
              version = packageJson.version;

              src = ./.;

              pnpmDeps = pnpm.fetchDeps {
                inherit (finalAttrs) pname version src pnpmInstallFlags;
                hash = "sha256-jMmkppy2NaXElqEXuvwXrLLwXxH/qQd+goF1VRiVdWQ=";
              };

              nativeBuildInputs = with pkgs; [
                makeWrapper
                nodejs
                pnpm.configHook
              ];

              # install dev dependencies as well, for rollup
              pnpmInstallFlags = ["--prod=false"];

              env.NODE_ENV = "production";

              buildPhase = ''
                runHook preBuild
                pnpm run build
                runHook postBuild
              '';

              entrypointPath = "index.js";

              checkPhase = ''
                runHook preCheck
                pnpm test
                runHook postCheck
              '';

              doCheck = false;

              installPhase = let
                binPath = lib.makeBinPath [
                  pkgs.nodejs
                ];
              in ''
                runHook preInstall

                mkdir -p $out
                cp -r build $out/${finalAttrs.pname}
                cp -r node_modules $out

                makeWrapper ${lib.getExe pkgs.nodejs} $out/bin/${finalAttrs.pname} \
                  --add-flags "$out/${finalAttrs.pname}/${finalAttrs.entrypointPath}" \
                  --set NODE_PATH $out/node_modules \
                  --prefix PATH : ${binPath}

                runHook postInstall
              '';

              meta = with lib; {
                changelog = "https://github.com/umami-software/umami/releases/tag/v${finalAttrs.version}";
                description = "Simple, easy to use, self-hosted web analytics solution";
                homepage = "https://umami.is/";
                license = with lib.licenses; [
                  mit
                ];
                platforms = lib.platforms.linux;
                # mainProgram = "umami-server";
                maintainers = with maintainers; [polarmutex];
              };
            });
        };

        apps = {
          dev = {
            type = "app";
            program = pkgs.writeShellApplication {
              name = "app-dev-server";
              runtimeInputs = [
                pkgs.nodejs
                pkgs.nodejs.pkgs.pnpm
              ];
              text = ''
                pnpm install
                pnpm run dev
              '';
            };
          };
          preview = {
            type = "app";
            program = pkgs.writeShellApplication {
              name = "preview-app";
              runtimeInputs = [pkgs.nodejs];
              runtimeEnv = {
                PORT = 5173;
              };
              text = ''
                node ${self'.packages.app}
              '';
            };
          };
          default = self'.apps.preview;
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
