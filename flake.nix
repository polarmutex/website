{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    rust-overlay.url = "github:oxalica/rust-overlay";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-parts.url = "github:hercules-ci/flake-parts";

    cargo-leptos-src = {
      url = "github:leptos-rs/cargo-leptos?tag=v0.2.17";
      flake = false;
    };
    obsidian-export-src = {
      url = "github:zoni/obsidian-export?tag=v23.12.0";
      flake = false;
    };
    obsidian-notes-src = {
      url = "github:polarmutex/website";
      flake = false;
    };

    nix-filter.url = "github:numtide/nix-filter";

    advisory-db = {
      url = "github:rustsec/advisory-db";
      flake = false;
    };
    pre-commit.url = "github:cachix/pre-commit-hooks.nix";
  };

  outputs = {
    self,
    advisory-db,
    cargo-leptos-src,
    crane,
    flake-parts,
    nix-filter,
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
            inputs.rust-overlay.overlays.default
          ];
        };

        toolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
          toolchain.default.override {
            extensions = ["rust-src" "rust-analyzer"];
            targets = ["wasm32-unknown-unknown"];
          });

        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

        src = nix-filter {
          root = ./.;
          include = [
            (nix-filter.lib.matchExt "toml")
            ./Cargo.lock
            ./src
            ./styles
          ];
        };

        common-args = {
          inherit src;
          buildInputs =
            [
              cargo-leptos
              pkgs.pkg-config
              pkgs.openssl
              pkgs.binaryen
              pkgs.cargo-generate
              pkgs.tailwindcss
            ]
            ++ lib.optionals pkgs.stdenv.isDarwin [
              # Additional darwin specific inputs can be set here
              pkgs.libicon
            ];
        };

        leptos-options = pkgs.lib.traceVal (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.metadata.leptos;

        cargo-leptos = (import ./nix/cargo-leptos.nix) {
          inherit pkgs craneLib;
          cargo-leptos = cargo-leptos-src;
        };

        style-js-deps = (import ./nix/style-js-deps.nix) {
          inherit pkgs nix-filter;

          source-root = ./.;
        };

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        site-deps = craneLib.buildDepsOnly (common-args
          // {
            cargoExtraArgs = " --target x86_64-unknown-linux-gnu";
            doCheck = false;
            # Needed to enable build-std inside Crane
            cargoVendorDir = craneLib.vendorMultipleCargoDeps {
              inherit (craneLib.findCargoFiles src) cargoConfigs;
              cargoLockList = [
                ./Cargo.lock

                # Unfortunately this approach requires IFD (import-from-derivation)
                # otherwise Nix will refuse to read the Cargo.lock from our toolchain
                # (unless we build with `--impure`).
                #
                # Another way around this is to manually copy the rustlib `Cargo.lock`
                # to the repo and import it with `./path/to/rustlib/Cargo.lock` which
                # will avoid IFD entirely but will require manually keeping the file
                # up to date!
                "${toolchain.passthru.availableComponents.rust-src}/lib/rustlib/src/rust/Cargo.lock"
              ];
            };
          });

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        site-server = craneLib.buildPackage (common-args
          // {
            pname = "brianryall_xyz_leptos";

            # Needed to enable build-std inside Crane
            cargoVendorDir = craneLib.vendorMultipleCargoDeps {
              inherit (craneLib.findCargoFiles src) cargoConfigs;
              cargoLockList = [
                ./Cargo.lock

                # Unfortunately this approach requires IFD (import-from-derivation)
                # otherwise Nix will refuse to read the Cargo.lock from our toolchain
                # (unless we build with `--impure`).
                #
                # Another way around this is to manually copy the rustlib `Cargo.lock`
                # to the repo and import it with `./path/to/rustlib/Cargo.lock` which
                # will avoid IFD entirely but will require manually keeping the file
                # up to date!
                "${toolchain.passthru.availableComponents.rust-src}/lib/rustlib/src/rust/Cargo.lock"
              ];
            };

            buildPhaseCargoCommand = "cargo leptos build --release -vvv";
            installPhaseCommand = ''
              mkdir -p $out/bin
              cp target/x86_64-unknown-linux-gnu/release/${leptos-options.output-name} $out/bin/
              cp -r target/site $out/bin/
            '';
            # Prevent cargo test and nextest from duplicating tests
            doCheck = false;
            cargoArtifacts = site-deps;
            # ALL CAPITAL derivations will get forwarded to mkDerivation and will set the env var during build
            SQLX_OFFLINE = "true";
            LEPTOS_BIN_TARGET_TRIPLE = "x86_64-unknown-linux-gnu"; # Adding this allows -Zbuild-std to work and shave 100kb off the WASM
            LEPTOS_BIN_PROFILE_RELEASE = "release";
            LEPTOS_LIB_PROFILE_RELEASE = "release-wasm-size";
          });

        site-server-container = let
          image = {
            name = "brianryall-xyz-leptos";
            registry = "ghcr.io";
            owner = "polarmutex";
            repo = "website";
          };
        in
          pkgs.dockerTools.buildLayeredImage {
            name = "${image.registry}/${image.owner}/${image.repo}/${image.name}";
            tag = "0.1.1";
            created = "now";
            contents = [
              site-server
              pkgs.cacert
              # pkgs.bashInteractive #debug
              # pkgs.coreutils #debug
            ];
            config = {
              # runs the executable with tini: https://github.com/krallin/tini
              # this does signal forwarding and zombie process reaping
              Entrypoint = ["${pkgs.tini}/bin/tini" "site-server" "--"];
              # Entrypoint = ["${pkgs.bash}/bin/bash"]; #debug
              WorkingDir = "${site-server}/bin";
              # we provide the env variables that we get from Cargo.toml during development
              # these can be overridden when the container is run, but defaults are needed
              Env = [
                "LEPTOS_OUTPUT_NAME=${leptos-options.output-name}"
                "LEPTOS_SITE_ROOT=site"
                "LEPTOS_SITE_PKG_DIR=${leptos-options.site-pkg-dir}"
                "LEPTOS_SITE_ADDR=0.0.0.0:3000"
                "LEPTOS_RELOAD_PORT=${builtins.toString leptos-options.reload-port}"
                "LEPTOS_ENV=PROD"
                # "LEPTOS_HASH_FILES=${builtins.toJSON leptos-options.hash-files}"
              ];
            };
          };
      in {
        checks = {
          # Build the crate as part of `nix flake check` for convenience
          inherit site-server;

          # Run clippy (and deny all warnings) on the crate source,
          # again, resuing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          site-clippy = craneLib.cargoClippy (common-args
            // {
              cargoArtifacts = site-deps;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            });

          site-doc = craneLib.cargoDoc (common-args
            // {
              cargoArtifacts = site-deps;
            });

          # Check formatting
          site-fmt = craneLib.cargoFmt {
            inherit src;
          };

          # Audit dependencies
          site-audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };

          # # audit licenses
          # site-server-deny = craneLib.cargoDeny {
          #   pname = common_args.pname;
          #   version = common_args.version;
          #   inherit src;
          # };

          # # run tests
          # site-server-nextest = craneLib.cargoNextest (common-args
          #   // {
          #     cargoArtifacts = site-server-deps;
          #     partitions = 1;
          #     partitionType = "count";
          #   });
        };

        pre-commit = {
          settings = {
            hooks.alejandra.enable = true;
            hooks.rustfmt.enable = true;
            # hooks.cargo-check.enable = true;
          };
        };

        packages = {
          default = site-server;
          server = site-server;
          container = site-server-container;
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs =
            (with pkgs; [
              toolchain # cargo and such from crane
              just # command recipes
              dive # docker images
              cargo-leptos # main leptos build tool
              flyctl # fly.io
              bacon # cargo check w/ hot reload
              cargo-deny # license checking
              # nodePackages.tailwindcss
              tailwindcss
              openssl
              pkg-config
              wasm-pack
            ])
            ++ common-args.buildInputs
            ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
              pkgs.darwin.Security
            ];
          RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
          shellHook = ''
            ${config.pre-commit.installationScript}
          '';
        };
      };
    };
}
