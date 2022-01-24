{
  description = "sand programming language";

  inputs = {
    nixpkgs.url = github:nixos/nixpkgs;
  };

  outputs = inputs:
    with inputs;
    let
      systems = [
        "aarch64-linux"
        "i686-linux"
        "x86_64-linux"
      ];

      config = system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        sand = pkgs.rustPlatform.buildRustPackage {
          name = "sand";
          version = "0.1.0";
          src = ./.;
          cargoSha256 = "sha256-qDnewGwfc+0G0PrFQnSnJEyEhGXdq86fObpsTcv+U7E=";
        };
      in {
        defaultPackage.${system} = sand;

        devShell.${system} = pkgs.mkShell {
          buildInputs = with pkgs; [ rustc cargo rustfmt cargo-watch ];
        };
      };
    in builtins.foldl' (acc: system: acc // (config system)) { } systems;
}
