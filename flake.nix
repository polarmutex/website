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
            packageJSON = lib.importJSON ./package.json;
          in
            pkgs.stdenvNoCC.mkDerivation (finalAttrs: {
              pname = packageJSON.name;
              inherit (packageJSON) version;

              nativeBuildInputs = with pkgs; [
                makeWrapper
                nodejs
                pnpm.configHook
              ];

              src = ./.;

              # install dev dependencies as well, for rollup
              pnpmInstallFlags = ["--prod=false"];

              pnpmDeps = pnpm.fetchDeps {
                inherit
                  (finalAttrs)
                  pname
                  pnpmInstallFlags
                  version
                  src
                  ;
                hash = "sha256-YrQVEQBcdwx7LhbzYaLLlx0wg1RbseMJ850m0NbefCc=";
              };

              env.NODE_ENV = "production";

              buildPhase = ''
                runHook preBuild
                pnpm build
                runHook postBuild
              '';

              checkPhase = ''
                runHook preCheck
                pnpm test
                runHook postCheck
              '';

              doCheck = false;

              installPhase = ''
                runHook preInstall
                mkdir -p $out
                cp -r ./build/* $out
                # Run database migrations before starting umami.
                # Add openssl to PATH since it is required for prisma to make SSL connections.
                # Force working directory to $out because umami assumes many paths are relative to it (e.g., prisma and geolite).
                # makeWrapper {nodejs}/bin/node $out/bin/umami-server  \
                #   --set NODE_ENV production \
                #   --set NEXT_TELEMETRY_DISABLED 1 \
                #   --set PRISMA_QUERY_ENGINE_LIBRARY "{prisma-engines'}/lib/libquery_engine.node" \
                #   --set PRISMA_SCHEMA_ENGINE_BINARY "{prisma-engines'}/bin/schema-engine" \
                #   --prefix PATH : {
                #   lib.makeBinPath [
                #     openssl
                #     nodejs
                #   ]
                # } \
                #   --chdir $out \
                #   --run "$out/node_modules/.bin/prisma migrate deploy" \
                #   --add-flags "$out/server.js"
                runHook postInstall
              '';

              meta = with lib; {
                changelog = "https://github.com/umami-software/umami/releases/tag/v${finalAttrs.version}";
                description = "Simple, easy to use, self-hosted web analytics solution";
                homepage = "https://umami.is/";
                license = with lib.licenses; [
                  mit
                  cc-by-40 # geocities
                ];
                platforms = lib.platforms.linux;
                mainProgram = "umami-server";
                maintainers = with maintainers; [diogotcorreia];
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
              text = ''
                PORT=5173 node ${self'.packages.app}
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
