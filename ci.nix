{ lib, pkgs, ... }: with lib; {
  name = "nixdrv";
  ci.gh-actions.enable = true;
  cache.cachix.arc.publicKey = "arc.cachix.org-1:DZmhclLkB6UO0rc0rBzNpwFbbaeLfyn+fYccuAy7YVY=";

  environment.test = {
    inherit (pkgs.buildPackages) cargo;
    inherit (pkgs.stdenv) cc;
  };

  tasks = {
    build = {
      inputs.build = pkgs.ci.command {
        name = "build";
        displayName = "cargo build";
        command = "cargo build";
        impure = true;
      };
    };
    examples = {
      inputs.parse = pkgs.ci.command {
        name = "parse";
        displayName = "parse pkgs.hello";
        command = ''
          cargo run --example parse -- ${builtins.unsafeDiscardStringContext pkgs.hello.drvPath}
        '';
        impure = true;
      };
    };
  };
}
