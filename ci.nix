{ pkgs, saber-pkgs }:
pkgs.buildEnv {
  name = "ci";
  paths = with pkgs;
    with saber-pkgs;
    (pkgs.lib.optionals pkgs.stdenv.isLinux ([ udev ])) ++ [
      anchor-0_24_2
      cargo-workspaces
      solana-basic

      # sdk
      nodejs
      yarn
      python3

      pkgconfig
      openssl
      jq
      gnused

      libiconv
    ] ++ (pkgs.lib.optionals pkgs.stdenv.isDarwin [
      pkgs.darwin.apple_sdk.frameworks.AppKit
      pkgs.darwin.apple_sdk.frameworks.IOKit
      pkgs.darwin.apple_sdk.frameworks.Foundation
    ]);
}
