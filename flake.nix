{
    description = "Rust flake";
    inputs =
        {
            nixpkgs.url = "github:nixos/nixpkgs/25.11";
            flake-utils.url = "github:numtide/flake-utils";
            # flake-utils.follows = "nixpkgs";
        };
    
    outputs = { self, nixpkgs, ... }@inputs:
        inputs.flake-utils.lib.eachDefaultSystem (system: let
            pkgs = nixpkgs.legacyPackages.${system};
        in
          {
              devShells.default = pkgs.mkShell
                  {
                      packages = with pkgs; [
                          cargo
                          rustc
                          rust-analyzer
                      ];
                  };
          });
}
