{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable-small";
    fenix.url = "github:nix-community/fenix";
  };
  outputs = inputs: let
    supportedSystems = inputs.nixpkgs.lib.systems.flakeExposed;
    forAllSupportedSystems = inputs.nixpkgs.lib.genAttrs supportedSystems;

    nixpkgsForSystem = system:
      import inputs.nixpkgs {
        inherit system;
        overlays = [
          (self: super: {
            fenix = inputs.fenix.packages.${system};
          })
        ];
      };
    nixpkgs = forAllSupportedSystems nixpkgsForSystem;

    devShellsForSystem = system:
      with nixpkgs.${system}; {
        default = mkShell {
          name = "default";
          packages = [
            fenix.complete.toolchain
          ];
        };
      };
  in {
    devShells = forAllSupportedSystems devShellsForSystem;
  };
}
