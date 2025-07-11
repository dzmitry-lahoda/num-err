{
  description = "Bikesheding the error type for num traits";
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

      packages.${system} = {
        check = pkgs.writeShellApplication {
          name = "check";
          runtimeInputs = with pkgs; [
            rust-bin.stable.latest.default
            cargo-hack
          ];
          text = ''
            cargo hack check --feature-powerset --no-dev-deps
            cargo hack test --each-feature
            cargo fmt --all -- --check --color always        
            cargo hack clippy --each-feature
          '';
        };

        publish = pkgs.writeShellApplication {
          name = "publish";
          runtimeInputs = with pkgs; [
            rust-bin.stable.latest.default
          ];
          text = ''
            # Source .env file if it exists
            if [ -f .env ]; then
              set -a
              # shellcheck disable=1091
              source .env
              set +a
              echo "Loaded .env file"
            fi

            # Check if CARGO_REGISTRY_TOKEN is set
            if [ -z "$CARGO_REGISTRY_TOKEN" ]; then
              echo "Error: CARGO_REGISTRY_TOKEN not set in environment or .env file"
              exit 1
            fi

            # Publish to crates.io
            cargo publish --token "$CARGO_REGISTRY_TOKEN"
          '';
        };
      };
    };
}
