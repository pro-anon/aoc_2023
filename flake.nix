{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    utils,
  }:
    utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
    in {
      devShell = with pkgs;
        mkShell {
          buildInputs = [
            cargo
            rustc
            rustfmt
            rust-analyzer
            rustPackages.clippy

            vscode-extensions.vadimcn.vscode-lldb
          ];

          RUST_SRC_PATH = rustPlatform.rustLibSrc;

          env.CODELLDB_PATH = vscode-extensions.vadimcn.vscode-lldb;
        };
    });
}
