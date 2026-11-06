{
  description = "Offline Fields of Mistria palette experiments";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-26.05";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { nixpkgs, fenix, ... }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
    in {
      packages.x86_64-linux.default = import ./nix/package.nix {
        pkgs = import nixpkgs { system = "x86_64-linux"; };
        fenixPkgs = fenix.packages.x86_64-linux;
      };
      devShells = nixpkgs.lib.genAttrs systems (system:
        let pkgs = import nixpkgs { inherit system; };
        in {
          default = import ./shell.nix {
            inherit pkgs;
            fenixPkgs = fenix.packages.${system};
          };
        });
    };
}
