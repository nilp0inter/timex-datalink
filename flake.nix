{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
    };
  };

  description = "A Timex Datalink Protocol implementation in Rust";

  outputs = inputs @ {
    self,
    nixpkgs,
    flake-parts,
  }:
    flake-parts.lib.mkFlake {inherit inputs;} {

      systems = ["x86_64-linux" "aarch64-linux"];

      perSystem = {
        self',
        config,
        pkgs,
        ...
      }: {
        devShells.default = pkgs.mkShell {
          # nativeBuildInputs = with pkgs; [
          # ];
          buildInputs = with pkgs; [
            cargo
            rustc
            rustfmt
            clippy
          ];
        };

        packages.timex-datalink = pkgs.rustPlatform.buildRustPackage {
          pname = "timex-datalink";
          version = "0.1.0";

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          # buildInputs = with pkgs; [
          # ];

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          meta = with nixpkgs.lib; {
            description = "A Timex Datalink Protocol implementation in Rust";
            homepage = "https://github.com/nilp0inter/timex-datalink";
            license = licenses.gpl3;
            platforms = platforms.linux;
            maintainers = with maintainers; [nilp0inter];
            mainProgram = "timex-datalink";
          };
        };

        packages.default = self'.packages.qkeypie;

        formatter = pkgs.alejandra;
      };
    };
}
