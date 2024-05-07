{
  inputs = {
    fenix.url = "github:nix-community/fenix";

    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

    systems.url = "github:nix-systems/default";
  };

  outputs = inputs: let
    supportedSystems = import inputs.systems;

    forEachSystem = inputs.nixpkgs.lib.genAttrs supportedSystems;

    nixkpgsOverlayForSystem = system: (nixpkgs: _: rec {
      cryptopals = {
        shell = nixpkgs.mkShell {
          name = "default";
          packages = [
            nixpkgs.fenix.complete.toolchain
          ];
        };
      };
    });

    nixpkgsForSystem = system:
      import inputs.nixpkgs {
        inherit system;
        overlays = [
          inputs.fenix.overlays.default
          (nixkpgsOverlayForSystem system)
        ];
      };

    nixpkgs = forEachSystem nixpkgsForSystem;
  in {
    devShells = forEachSystem (system: {
      default = nixpkgs.${system}.cryptopals.shell;
    });
  };
}
