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
          nativeBuildInputs = with pkgs; [
            pkg-config
            lld
            llvmPackages.bintools
          ];
          buildInputs = with pkgs; [
            cargo
            rustc
            rustfmt
            clippy
            wasm-pack
            wasm-bindgen-cli
            nodejs
            python3
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
        
        # Add td150 command as a separate package
        packages.td150 = pkgs.rustPlatform.buildRustPackage {
          pname = "td150";
          version = "0.1.0";

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          src = ./.;
          
          # Only build and install the td150 binary
          buildPhase = ''
            runHook preBuild
            cargo build --release --bin td150
            runHook postBuild
          '';
          
          installPhase = ''
            runHook preInstall
            mkdir -p $out/bin
            cp target/release/td150 $out/bin/
            runHook postInstall
          '';

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          meta = with nixpkgs.lib; {
            description = "Timex Datalink 150 protocol 3 data transfer tool";
            homepage = "https://github.com/nilp0inter/timex-datalink";
            license = licenses.gpl3;
            platforms = platforms.linux;
            maintainers = with maintainers; [nilp0inter];
            mainProgram = "td150";
          };
        };

        packages.default = self'.packages.timex-datalink;

        formatter = pkgs.alejandra;
      };
    };
}
