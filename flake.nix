{
  description = "Bikesheding";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    { nixpkgs, rust-overlay, ... }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
        overlays = [ rust-overlay.overlays.default ];
      };
    in
    {
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          rust-bin.stable.latest.default
          cargo-hack
        ];
      };

      packages.${system}.check = pkgs.writeShellApplication {
        name = "check";

        text = ''
          cargo hack check --feature-powerset --no-dev-deps
          cargo hack test --each-feature
          cargo fmt --all -- --check --color always        
          cargo hack clippy --each-feature
        '';
      };
    };
}
