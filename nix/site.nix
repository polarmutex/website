{inputs, ...}: let
  inherit (inputs.nix-filter.lib) filter inDirectory matchExt;
in {
  perSystem = {
    config,
    pkgs,
    self',
    inputs',
    system,
    ...
  }: {
    packages = {
      site = pkgs.buildNpmPackage {
        pname = "brianryall.xyz";
        version = inputs.self.sourceInfo.shortRev or "dirty";
        src = filter {
          root = ./..;
          include = [
            (inDirectory "src")
            (inDirectory "public")
            (matchExt "js")
            (matchExt "cjs")
            (matchExt "mjs")
            ../package.json
            ../package-lock.json
          ];
        };
        npmFlags = ["--ignore-scripts"];
        npmDepsHash = "sha256-2/ChB4Ftf98Ci0eL5YQ0s/T8YXHKwNsIQVBRsf6mIiU=";
        installPhase = ''
          mv dist $out
        '';
      };
    };
  };

  #dream2nix.inputs.self = {
  #  source = filter {
  #    root = ./..;
  #    include = [
  #      (inDirectory "src")
  #      (inDirectory "public")
  #      (matchExt "js")
  #      (matchExt "cjs")
  #      (matchExt "mjs")
  #      ../package.json
  #      ../yarn.lock
  #    ];
  #  };
  #  projects.blog = {
  #    name = "blog";
  #    subsystem = "nodejs";
  #    translator = "yarn-lock";
  #    subsystemInfo.nodejs = 18;
  #  };
  #  packageOverrides.blog.copyBlog = {
  #    installPhase = ''
  #      mkdir -p $out
  #      cp -rv ./dist/* $out
  #    '';
  #  };
  #};
}
