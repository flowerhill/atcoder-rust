{
  description = "AtCoder Rust environment (Rust 1.89.0)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.05";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      systems = [ "aarch64-darwin" "x86_64-darwin" "aarch64-linux" "x86_64-linux" ];
      forAllSystems = f: nixpkgs.lib.genAttrs systems (system:
        f (import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        }));
    in
    {
      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          packages = [
            # 各問題ディレクトリの rust-toolchain.toml (channel = "1.89.0") に合わせる
            (pkgs.rust-bin.stable."1.89.0".default.override {
              extensions = [ "rust-src" "rust-analyzer" ];
            })
            pkgs.online-judge-tools
          ];
        };
      });
    };
}
