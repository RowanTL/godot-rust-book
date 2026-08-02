{
  description = "The godot-rust book — mdbook toolchain";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/b6018f87da91d19d0ab4cf979885689b469cdd41";

  outputs = { self, nixpkgs }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
      forAllSystems = f:
        nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
    in
    {
      packages = forAllSystems (pkgs: {
        inherit (pkgs) mdbook;
        default = pkgs.mdbook;
      });

      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          packages = [
            pkgs.mdbook
            # Preprocessors required by book.toml.
            pkgs.mdbook-toc
            pkgs.mdbook-admonish
          ];
        };
      });
    };
}
