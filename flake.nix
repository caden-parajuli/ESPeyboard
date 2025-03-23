{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs =
    { nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        nixpkgs-esp-dev = builtins.fetchGit {
          url = "https://github.com/mirrexagon/nixpkgs-esp-dev.git";
          rev = "31ee58005f43e93a6264e3667c9bf5c31b368733";
        };
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import "${nixpkgs-esp-dev}/overlay.nix") ];
        };
      in
      with pkgs;
      {
        devShell = mkShell {
          buildInputs = with pkgs; [
            # (esp-idf-full.override {
            #   rev = "release/v5.4";
            #   sha256 = "sha256-308d0gaHH9WiRawiPC3TKne6T+P1AZUA25OXO53WFtc=";
            # })
            esp-idf-esp32s3
            python312Packages.psutil
          ];
        };
      }
    );
}
