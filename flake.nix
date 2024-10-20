{
  description = "Record basic dev flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
  };

  outputs = { self, nixpkgs }:
  let 
    system = "x86_64-linux";
    pkgs = import nixpkgs { inherit system; };
  in {
      devShells = {
        ${system} = {
        docs = pkgs.mkShell {
          buildInputs = with pkgs; [
            openssl
          ];

          PKG_CONFIG_PATH="${pkgs.openssl}/lib";
        };
        };
      };
    };
}
