{
  description = "Rust dev env";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-25.05";
  };

  outputs = { self, nixpkgs }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in
    {
      devShells.x86_64-linux.default = pkgs.mkShell {
        buildInputs = [
          pkgs.cargo
          pkgs.rustc
          pkgs.rustfmt
          pkgs.clippy
          pkgs.rust-analyzer
          pkgs.lldb
        ];

        # used by rust-analyzer
        env.RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };

      devShells.x86_64-linux.coverage = pkgs.mkShell {
        buildInputs = [
          pkgs.rustup
          pkgs.cargo-llvm-cov
        ];
      };

      formatter.x86_64-linux = pkgs.nixpkgs-fmt;
    };
}
