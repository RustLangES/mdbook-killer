{
  crane,
  cranix,
  fenix,
}: {
  config,
  lib,
  pkgs,
  ...
}:
with lib; let
  mdbookKiller = import ./. {
    inherit crane cranix fenix pkgs lib;
    system = pkgs.system;
  };
  cfg = config.programs.mdbookKiller;
in {
  options.programs.mdbookKiller = {
    enable = mkEnableOption "tools to generate books";
  };

  config = mkIf cfg.enable {
    environment.systemPackages = [mdbookKiller.packages.default];
  };
}
