{
  description = "Build a cargo project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    #nixpkgs-leptos.url = "github:benwis/nixpkgs";
    rust-overlay.url = "github:oxalica/rust-overlay";

    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-parts.url = "github:hercules-ci/flake-parts";
    nix-filter.url = "github:numtide/nix-filter";
    obsidian-export = {
      url = "github:zoni/obsidian-export/v22.11.0";
      flake = false;
    };
    cargo-leptos = {
      #url= "github:leptos-rs/cargo-leptos/v1.7";
      #url = "github:polarmutex/cargo-leptos/fix-cache";
      url = "github:benwis/cargo-leptos";
      flake = false;
    };

    flake-utils.url = "github:numtide/flake-utils";

    advisory-db = {
      url = "github:rustsec/advisory-db";
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
    nixpkgs,
    crane,
    flake-utils,
    advisory-db,
    ...
  } @ inputs:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [
          #(_final: prev: {
          #  benwis = import inputs.nixpkgs-leptos {
          #    inherit system;
          #  };
          #})
          inputs.rust-overlay.overlays.default
        ];
      };

      inherit (pkgs) lib;

      craneLib = let
        rust-toolchain =
          pkgs.rust-bin.selectLatestNightlyWith
          (toolchain:
            toolchain.default.override {
              extensions = ["rust-src"];
              targets = ["wasm32-unknown-unknown"];
            });
      in
        (crane.lib.${system}).overrideToolchain rust-toolchain;
      #src = craneLib.cleanCargoSource ./.;
      src = ./.;

      # Common arguments can be set here to avoid repeating them later
      commonArgs = {
        inherit src;

        buildInputs =
          [
            # Add additional build inputs here
            cargo-leptos
            pkgs.binaryen
            pkgs.sass
            pkgs.cargo-generate
            pkgs.openssl
            pkgs.pkg-config
          ]
          ++ lib.optionals pkgs.stdenv.isDarwin [
            # Additional darwin specific inputs can be set here
            pkgs.libiconv
          ];
      };

      # Build *just* the cargo dependencies, so we can reuse
      # all of that work (e.g. via cachix) when running in CI
      cargoArtifacts =
        craneLib.buildDepsOnly commonArgs;

      # Build the actual crate itself, reusing the dependency
      # artifacts from above.
      my-crate = craneLib.mkCargoDerivation (commonArgs
        // {
          buildPhaseCargoCommand = "cargo leptos build --release";
          #cargoBuildCommand = "cargo leptos build --release";
          inherit cargoArtifacts;
          installPhaseCommand = ''
            mkdir -p $out
            cp Cargo.toml $out
            mkdir -p $out/content/posts
            cp -r content/posts/* $out/content/posts/.
          '';
        });
      #my-crate = craneLib.buildPackage (commonArgs
      #  // {
      #    buildPhaseCargoCommand = "RUST_BACKTRACE=1 cargo leptos build --release";
      #    #cargoBuildCommand = "cargo leptos build --release";
      #    inherit cargoArtifacts;
      #    installPhaseCommand = ''
      #      cp target/server/release/brianryall-xyz $out
      #      mkdir $out/site
      #      cp -r target/site $out/site
      #    '';
      #  });

      cargo-leptos = pkgs.rustPlatform.buildRustPackage rec {
        pname = "cargo-leptos";
        #version = "0.1.7";
        version = "0.1.8.1";
        buildFeatures = ["no_downloads"]; # cargo-leptos will try to download Ruby and other things without this feature

        src = inputs.cargo-leptos; #pkgs.fetchFromGitHub {
        #owner = "leptos-rs";
        #owner = "polarmutex";
        #repo = pname;
        #rev = version;
        #rev = "fix-cache";
        #hash = "sha256-5zG4dtrU2yb9tywkLr2U98AGN+yMzIcoVMVr9v6OFY0=";
        #};

        cargoSha256 = "sha256-fi5o8hXDbrgeVG4ctgewH5Ii35TZcZbCKblmsh3Bh6k=";

        nativeBuildInputs = [pkgs.pkg-config pkgs.openssl];

        buildInputs = with pkgs;
          [openssl pkg-config]
          ++ lib.optionals stdenv.isDarwin [
            Security
          ];

        doCheck = false; # integration tests depend on changing cargo config

        meta = with lib; {
          description = "A build tool for the Leptos web framework";
          homepage = "https://github.com/leptos-rs/cargo-leptos";
          changelog = "https://github.com/leptos-rs/cargo-leptos/blob/v${version}/CHANGELOG.md";
          license = with licenses; [mit];
          maintainers = with maintainers; [benwis];
        };
      };
    in {
      checks =
        {
          # Build the crate as part of `nix flake check` for convenience
          inherit my-crate;

          # Run clippy (and deny all warnings) on the crate source,
          # again, resuing the dependency artifacts from above.
          #
          # Note that this is done as a separate derivation so that
          # we can block the CI if there are issues here, but not
          # prevent downstream consumers from building our crate by itself.
          my-crate-clippy = craneLib.cargoClippy (commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            });

          my-crate-doc = craneLib.cargoDoc (commonArgs
            // {
              inherit cargoArtifacts;
            });

          # Check formatting
          my-crate-fmt = craneLib.cargoFmt {
            inherit src;
          };

          # Audit dependencies
          my-crate-audit = craneLib.cargoAudit {
            inherit src advisory-db;
          };

          # Run tests with cargo-nextest
          # Consider setting `doCheck = false` on `my-crate` if you do not want
          # the tests to run twice
          my-crate-nextest = craneLib.cargoNextest (commonArgs
            // {
              inherit cargoArtifacts;
              partitions = 1;
              partitionType = "count";
            });
        }
        // lib.optionalAttrs (system == "x86_64-linux") {
          # NB: cargo-tarpaulin only supports x86_64 systems
          # Check code coverage (note: this will not upload coverage anywhere)
          my-crate-coverage = craneLib.cargoTarpaulin (commonArgs
            // {
              inherit cargoArtifacts;
            });
        };

      packages.default = my-crate;

      apps.default = flake-utils.lib.mkApp {
        drv = my-crate;
      };

      devShells.default = pkgs.mkShell {
        inputsFrom = builtins.attrValues self.checks;

        # Extra inputs can be added here
        nativeBuildInputs = with pkgs; [
          cargo
          (rust-bin.selectLatestNightlyWith
            (toolchain:
              toolchain.default.override {
                extensions = ["rust-src"];
                targets = ["wasm32-unknown-unknown"];
              }))
          cargo-leptos
          sass
          nodejs
          openssl
          pkg-config
          binaryen
          wasm-pack
          nodePackages.tailwindcss
          jq
          cachix
        ];
        packages = [
        ];
      };
    });
}
