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
      url = "github:leptos-rs/cargo-leptos?tag=v0.2.16";
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

        src = nix-filter {
          root = ./.;
          include = [
            (nix-filter.lib.matchExt "toml")
            ./Cargo.lock
            ./crates
          ];
        };

        toolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
          toolchain.default.override {
            extensions = ["rust-src" "rust-analyzer"];
            targets = ["wasm32-unknown-unknown"];
          });

        leptos-options = builtins.elemAt (builtins.fromTOML (builtins.readFile ./Cargo.toml)).workspace.metadata.leptos 0;

        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;

        cargo-leptos = (import ./nix/cargo-leptos.nix) {
          inherit pkgs craneLib;
          cargo-leptos = cargo-leptos-src;
        };
        # cargo-leptos = craneLib.buildPackage rec {
        #   pname = "cargo-leptos";
        #   version = "v0.1.11";
        #   buildFeatures = ["no_downloads"]; # cargo-leptos will try to download Ruby and other things without this feature
        #
        #   src = inputs.cargo-leptos-src;
        #
        #   nativeBuildInputs = [
        #     pkgs.pkg-config
        #     pkgs.openssl
        #   ];
        #
        #   buildInputs = with pkgs;
        #     [openssl pkg-config]
        #     ++ lib.optionals stdenv.isDarwin [
        #       Security
        #     ];
        #
        #   doCheck = false; # integration tests depend on changing cargo config
        #
        #   meta = with lib; {
        #     description = "A build tool for the Leptos web framework";
        #     homepage = "https://github.com/leptos-rs/cargo-leptos";
        #     changelog = "https://github.com/leptos-rs/cargo-leptos/blob/v${version}/CHANGELOG.md";
        #     license = with licenses; [mit];
        #     maintainers = with maintainers; [benwis];
        #   };
        # };

        style-js-deps = (import ./nix/style-js-deps.nix) {
          inherit pkgs nix-filter;

          source-root = ./.;
        };

        common-args = {
          inherit src;

          pname = leptos-options.bin-package;
          version = "0.1.0";

          doCheck = false;

          nativeBuildInputs =
            [
              # Add additional build inputs here
              cargo-leptos
              pkgs.cargo-generate
              pkgs.binaryen
              pkgs.clang
              pkgs.mold

              # for styling
              pkgs.dart-sass
              pkgs.tailwindcss
              pkgs.yarn
              pkgs.yarn2nix-moretea.fixup_yarn_lock
            ]
            ++ pkgs.lib.optionals (system == "x86_64-linux") [
              # extra packages only for x86_64-linux
              pkgs.nasm
            ]
            ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
              # Additional darwin specific inputs can be set here
              pkgs.libiconv
            ];

          buildInputs = [
            pkgs.pkg-config
            pkgs.openssl
          ];
        };

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        site-server-deps = craneLib.buildDepsOnly (common-args
          // {
            # if work is duplicated by the `server-site` package, update these
            # commands from the logs of `cargo leptos build --release -vvv`
            buildPhaseCargoCommand = ''
              # build the frontend dependencies
              cargo build --package=${leptos-options.lib-package} --lib --target-dir=/build/source/target/front --target=wasm32-unknown-unknown --no-default-features --profile=${leptos-options.lib-profile-release}
              # build the server dependencies
              cargo build --package=${leptos-options.bin-package} --no-default-features --release
            '';
          });

        # Build the actual crate itself, reusing the dependency
        # artifacts from above.
        site-server = craneLib.buildPackage (common-args
          // {
            # link the style packages node_modules into the build directory
            preBuild = ''
              ln -s ${style-js-deps}/node_modules \
                ./crates/site-app/style/tailwind/node_modules
            '';

            buildPhaseCargoCommand = ''
              cargo leptos build --release -vvv
            '';

            installPhaseCommand = ''
              mkdir -p $out/bin
              cp target/release/site-server $out/bin/
              cp target/release/hash.txt $out/bin/
              cp -r target/site $out/bin/
            '';

            doCheck = false;
            cargoArtifacts = site-server-deps;
          });

        site-server-container = let
          image = {
            name = leptos-options.bin-package;
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
                "LEPTOS_OUTPUT_NAME=${leptos-options.name}"
                "LEPTOS_SITE_ROOT=site"
                "LEPTOS_SITE_PKG_DIR=${leptos-options.site-pkg-dir}"
                "LEPTOS_SITE_ADDR=0.0.0.0:3000"
                "LEPTOS_RELOAD_PORT=${builtins.toString leptos-options.reload-port}"
                "LEPTOS_ENV=PROD"
                "LEPTOS_HASH_FILES=${builtins.toJSON leptos-options.hash-files}"
              ];
            };
          };
      in {
        checks = {
          # lint packages
          app-hydrate-clippy = craneLib.cargoClippy (common-args
            // {
              cargoArtifacts = site-server-deps;
              cargoClippyExtraArgs = "-p site-app --features hydrate -- --deny warnings";
            });
          app-ssr-clippy = craneLib.cargoClippy (common-args
            // {
              cargoArtifacts = site-server-deps;
              cargoClippyExtraArgs = "-p site-app --features ssr -- --deny warnings";
            });
          site-server-clippy = craneLib.cargoClippy (common-args
            // {
              cargoArtifacts = site-server-deps;
              cargoClippyExtraArgs = "-p site-server -- --deny warnings";
            });
          site-frontend-clippy = craneLib.cargoClippy (common-args
            // {
              cargoArtifacts = site-server-deps;
              cargoClippyExtraArgs = "-p site-frontend -- --deny warnings";
            });

          # make sure the docs build
          site-server-doc = craneLib.cargoDoc (common-args
            // {
              cargoArtifacts = site-server-deps;
            });

          # check formatting
          site-server-fmt = craneLib.cargoFmt {
            pname = common-args.pname;
            version = common-args.version;

            inherit src;
          };

          # # audit licenses
          # site-server-deny = craneLib.cargoDeny {
          #   pname = common_args.pname;
          #   version = common_args.version;
          #   inherit src;
          # };

          # run tests
          site-server-nextest = craneLib.cargoNextest (common-args
            // {
              cargoArtifacts = site-server-deps;
              partitions = 1;
              partitionType = "count";
            });
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
            ++ common-args.nativeBuildInputs
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
#pkgs = import nixpkgs {
#  inherit system;
#  overlays = [
#    #(_final: prev: {
#    #  benwis = import inputs.nixpkgs-leptos {
#    #    inherit system;
#    #  };
#    #})
#    inputs.rust-overlay.overlays.default
#  ];
#};
#inherit (pkgs) lib;
#craneLib = let
#  rust-toolchain =
#    pkgs.rust-bin.selectLatestNightlyWith
#    (toolchain:
#      toolchain.default.override {
#        extensions = ["rust-src"];
#        targets = ["wasm32-unknown-unknown"];
#      });
#in
#  (crane.lib.${system}).overrideToolchain rust-toolchain;
##src = craneLib.cleanCargoSource ./.;
#src = ./.;
# Common arguments can be set here to avoid repeating them later
#commonArgs = {
#  inherit src;
#  buildInputs =
#    [
#      # Add additional build inputs here
#      cargo-leptos
#      pkgs.binaryen
#      pkgs.sass
#      pkgs.cargo-generate
#      pkgs.openssl
#      pkgs.pkg-config
#    ]
#    ++ lib.optionals pkgs.stdenv.isDarwin [
#      # Additional darwin specific inputs can be set here
#      pkgs.libiconv
#    ];
#};
# Build *just* the cargo dependencies, so we can reuse
# all of that work (e.g. via cachix) when running in CI
#cargoArtifacts =
#  craneLib.buildDepsOnly commonArgs;
# Build the actual crate itself, reusing the dependency
# artifacts from above.
#my-crate = craneLib.mkCargoDerivation (commonArgs
#  // {
#    buildPhaseCargoCommand = "cargo leptos build --release";
#    #cargoBuildCommand = "cargo leptos build --release";
#    inherit cargoArtifacts;
#    installPhaseCommand = ''
#      mkdir -p $out
#      cp Cargo.toml $out
#      mkdir -p $out/content/ideas
#      cp -r content/ideas/* $out/content/ideas/.
#    '';
#  });
#cargo-leptos = pkgs.rustPlatform.buildRustPackage rec {
#  pname = "cargo-leptos";
#  version = "0.1.8";
#  buildFeatures = ["no_downloads"]; # cargo-leptos will try to download Ruby and other things without this feature
#  src = inputs.cargo-leptos; #pkgs.fetchFromGitHub {
#  #owner = "leptos-rs";
#  #owner = "polarmutex";
#  #repo = pname;
#  #rev = version;
#  #rev = "fix-cache";
#  #hash = "sha256-5zG4dtrU2yb9tywkLr2U98AGN+yMzIcoVMVr9v6OFY0=";
#  #};
#  cargoSha256 = "sha256-w/9W4DXbh4G5DZ8IGUz4nN3LEjHhL7HgybHqODMFzHw=";
#  nativeBuildInputs = [pkgs.pkg-config pkgs.openssl];
#  buildInputs = with pkgs;
#    [openssl pkg-config]
#    ++ lib.optionals stdenv.isDarwin [
#      #Security
#    ];
#  doCheck = false; # integration tests depend on changing cargo config
#  meta = with lib; {
#    description = "A build tool for the Leptos web framework";
#    homepage = "https://github.com/leptos-rs/cargo-leptos";
#    changelog = "https://github.com/leptos-rs/cargo-leptos/blob/v${version}/CHANGELOG.md";
#    license = with licenses; [mit];
#    maintainers = with maintainers; [benwis];
#  };
#};
#in {
#  checks =
# {
# Build the crate as part of `nix flake check` for convenience
#      inherit my-crate;
# Run clippy (and deny all warnings) on the crate source,
# again, resuing the dependency artifacts from above.
#
# Note that this is done as a separate derivation so that
# we can block the CI if there are issues here, but not
# prevent downstream consumers from building our crate by itself.
#my-crate-clippy = craneLib.cargoClippy (commonArgs
#  // {
#    inherit cargoArtifacts;
#    cargoClippyExtraArgs = "--all-targets -- --deny warnings";
#  });
#my-crate-doc = craneLib.cargoDoc (commonArgs
#  // {
#    inherit cargoArtifacts;
#  });
# Check formatting
#my-crate-fmt = craneLib.cargoFmt {
#  inherit src;
#};
# Audit dependencies
#my-crate-audit = craneLib.cargoAudit {
#  inherit src advisory-db;
#};
# Run tests with cargo-nextest
# Consider setting `doCheck = false` on `my-crate` if you do not want
# the tests to run twice
#my-crate-nextest = craneLib.cargoNextest (commonArgs
#  // {
#    inherit cargoArtifacts;
#    partitions = 1;
#    partitionType = "count";
#  });
#}
#// lib.optionalAttrs (system == "x86_64-linux") {
# NB: cargo-tarpaulin only supports x86_64 systems
# Check code coverage (note: this will not upload coverage anywhere)
#   my-crate-coverage = craneLib.cargoTarpaulin (commonArgs
#     // {
#       inherit cargoArtifacts;
#     });
# };
#packages.default = my-crate;
#apps.default = flake-utils.lib.mkApp {
#  drv = my-crate;
#};
#devShells.default = pkgs.mkShell {
#  inputsFrom = builtins.attrValues self.checks;
#  # Extra inputs can be added here
#  nativeBuildInputs = with pkgs; [
#    cargo
#    (rust-bin.selectLatestNightlyWith
#      (toolchain:
#        toolchain.default.override {
#          extensions = ["rust-src"];
#          targets = ["wasm32-unknown-unknown"];
#        }))
#    cargo-leptos
#    sass
#    nodejs
#    openssl
#    pkg-config
#    binaryen
#    wasm-pack
#    nodePackages.tailwindcss
#    jq
#    cachix
#  ];
#  packages = [
#  ];
#};

