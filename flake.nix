{
  description = "Prompt Assist";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, fenix, ... }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      rustToolchain = fenix.packages.${system}.stable.toolchain;
    in
    {
      devShells.${system}.default = pkgs.mkShell
        {
          buildInputs = [
            rustToolchain
            pkgs.rust-analyzer
            pkgs.cargo-expand
            pkgs.cargo-watch
            pkgs.cargo-edit

            pkgs.sqlite
            pkgs.sqlite-interactive
            pkgs.sqlx-cli

            pkgs.pkg-config
            pkgs.git
          ];

          shellHook = ''
            echo "Rust development environment ready!"
            echo "Rust version: $(rustc --version)"
          '';
        };
    };
}
