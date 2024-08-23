{

  inputs.nixpkgs = {
    type = "github";
    owner = "NixOS";
    repo = "nixpkgs";
    ref = "nixos-unstable";
  };

  outputs =
    { nixpkgs, self }:
    {
      packages.x86_64-linux =
        let
          pkgs = nixpkgs.legacyPackages.x86_64-linux;
        in
        {
          bot = pkgs.callPackage ./package.nix { };
          default = self.packages.x86_64-linux.bot;
        };
      devShells.x86_64-linux =
        let
          pkgs = nixpkgs.legacyPackages.x86_64-linux;
        in
        {
          default = pkgs.mkShell {
            inputsFrom = [ self.packages.${pkgs.stdenv.system}.default ];
            packages = builtins.attrValues {
              #
              inherit (pkgs) rust-analyzer rustfmt clippy;
            };
          };
        };
    };
}
