{
  crane,
  cranix,
  fenix,
}: final: prev: let
  mdbookKiller = prev.callPackage ./. {inherit crane cranix fenix;};
in {
  mdbookKiller = mdbookKiller.packages.default;
}
