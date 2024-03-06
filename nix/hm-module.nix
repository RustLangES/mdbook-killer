{ crane
, cranix
, fenix
,
}: { config
   , lib
   , pkgs
   , ...
   }:
with lib; let
  mdbookKiller = import ./. {
    inherit crane cranix fenix pkgs lib;
    system = pkgs.system;
  };
  cfgMDBookKiller = config.programs.mdbookKiller;
  # Temp config
  mdbookKillerPackage = lists.optional cfgMDBookKiller.enable mdbookKiller.packages.default;
in
{
  options.programs = {
    mdbookKiller = {
      enable = mkEnableOption "enable mdbook-killer";
    };
  };

  config = mkIf cfgMDBookKiller.enable {
    home.packages = mdbookKillerPackage;
  };
}
