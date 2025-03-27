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

        # WebAssembly webapp package
        packages.webapp = pkgs.rustPlatform.buildRustPackage {
          pname = "timex-datalink-webapp";
          version = "0.1.0";
          
          src = ./.;
          
          cargoLock = {
            lockFile = ./Cargo.lock;
          };
          
          nativeBuildInputs = with pkgs; [
            wasm-pack
            wasm-bindgen-cli
            nodejs
            lld
            llvmPackages.bintools
            binaryen
          ];
          
          # Skip the default check and install phases
          doCheck = false;
          dontInstall = true;
          
          buildPhase = ''
            # Create a home directory for cargo
            export HOME=$TMPDIR
            
            # Create output directory
            mkdir -p webapp/dist
            
            # Build the WebAssembly package
            wasm-pack build \
              --target web \
              --out-name timex_datalink_wasm \
              --out-dir webapp/dist \
              --no-default-features \
              --features "wasm"
            
            # Copy only the essential web files to dist directory
            cp webapp/public/*.{html,js,css} webapp/dist/
            
            # Remove TypeScript definition files and other unnecessary files
            rm -f webapp/dist/*.d.ts
            rm -f webapp/dist/package.json
            rm -f webapp/dist/README.md
            
            # Create the final output directory
            mkdir -p $out
            cp -r webapp/dist/* $out/
          '';
          
          meta = with nixpkgs.lib; {
            description = "WebAssembly interface for Timex Datalink watches";
            homepage = "https://github.com/nilp0inter/timex-datalink";
            license = licenses.gpl3;
            platforms = platforms.linux;
            maintainers = with maintainers; [nilp0inter];
            mainProgram = "timex-datalink-webapp";
          };
        };

        packages.default = self'.packages.timex-datalink;

        formatter = pkgs.alejandra;
      };
    };
}
