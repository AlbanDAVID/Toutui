{
  description = "Toutui - A TUI Audiobookshelf client for Linux and macOS";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "toutui";
          version = "0.4.2-beta";
          
          src = ./.;
          
          cargoLock.lockFile = ./Cargo.lock;
          
          # Build-time dependencies
          nativeBuildInputs = with pkgs; [ 
            pkg-config 
            perl
            makeWrapper  # Provides wrapProgram
          ];
          
          # Runtime dependencies
          buildInputs = with pkgs; [ 
            openssl
            vlc 
          ];
          
          # netcat needed at runtime
          postInstall = ''
            wrapProgram $out/bin/toutui \
              --prefix PATH : ${pkgs.lib.makeBinPath [ pkgs.netcat ]}
          '';

          meta = with pkgs.lib; {
            description = "TUI Audiobookshelf client for Linux and macOS";
            homepage = "https://github.com/AlbanDAVID/Toutui";
            license = licenses.gpl3Only;
            maintainers = [ ];
            platforms = platforms.unix;
          };
        };

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            cargo
            rustc
            openssl
            vlc
            netcat
            pkg-config
            perl
          ];
        };
      });
}
