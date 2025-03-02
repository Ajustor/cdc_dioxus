{
  # pkgs ? import <nixpkgs> { },
}:
let
  pkgs = import (fetchTarball("channel:nixpkgs-unstable")) {};

  overrides = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));

  buildToolsVersion = "30.0.3";
  androidComposition = pkgs.androidenv.composeAndroidPackages {
    cmdLineToolsVersion = "8.0";
    toolsVersion = "26.1.1";
    platformToolsVersion = "35.0.2";
    buildToolsVersions = [ buildToolsVersion ];
    includeEmulator = true;
    emulatorVersion = "35.4.6";
    platformVersions = [ "28" "29" "30" ];
    includeSources = false;
    includeSystemImages = false;
    systemImageTypes = [ "google_apis_playstore" ];
    abiVersions = [ "armeabi-v7a" "arm64-v8a" ];
    cmakeVersions = [ "3.10.2" ];
    includeNDK = true;
    ndkVersions = ["22.0.7026061"];
    useGoogleAPIs = false;
    useGoogleTVAddOns = false;
    includeExtras = [
      "extras;google;gcm"
    ];
  };
  ANDROID_HOME = "${androidComposition.androidsdk}/libexec/android-sdk";
in
pkgs.callPackage (
  {
    atkmm,
    cairo,
    gcc,
    gdk-pixbuf,
    glib,
    gtk3,
    mkShell,
    openssl,
    pango,
    pkg-config,
    rustup,
    rustPlatform,
    stdenv,
    webkitgtk_4_1,  # for javascriptcoregtk-rs-sys
    xdotool,        # for libxdo
  }:
  mkShell {
    strictDeps = true;
    nativeBuildInputs = [
      gcc
      openssl
      pkg-config
      rustup
      rustPlatform.bindgenHook
    ];
    buildInputs = [
      atkmm
      cairo
      gdk-pixbuf
      glib
      gtk3
      pango
      webkitgtk_4_1
      xdotool
    ];
    GDK_BACKEND = "x11";  # NVIDIA might disagree otherwise.
    PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
    RUSTC_VERSION = overrides.toolchain.channel;
    WEBKIT_DISABLE_DMABUF_RENDERER = 1;  # Again NVIDIA things.

    ANDROID_HOME = ANDROID_HOME;
    ANDROID_NDK_ROOT = "${ANDROID_HOME}/ndk-bundle";
    NDK_HOME = "${ANDROID_HOME}/ndk/22.0.7026061";
    # Use the same buildToolsVersion here
    GRADLE_OPTS = "-Dorg.gradle.project.android.aapt2FromMavenOverride=${ANDROID_HOME}/build-tools/${buildToolsVersion}/aapt2";

    # https://github.com/rust-lang/rust-bindgen#environment-variables
    shellHook = ''
      export PATH="''${CARGO_HOME:-~/.cargo}/bin":"$PATH"
      export PATH="''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-${stdenv.hostPlatform.rust.rustcTarget}/bin":"$PATH"
     '';
  }
) { }