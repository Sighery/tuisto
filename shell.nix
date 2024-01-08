with import <nixpkgs> {};

stdenv.mkDerivation {
  name = "rustlang";
  buildInputs = [
    # System requirements.
    nixpkgs-fmt
    rnix-lsp
    docker-client
    gnumake

    rustc
    cargo
    rustfmt
    clippy
    rustup
  ];
  src = null;
  #shellHook = ''
  #  # Allow the use of wheels.
  #  SOURCE_DATE_EPOCH=$(date +%s)
  #
  #  # Augment the dynamic linker path
  #  export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:${stdenv.cc.cc.lib}/lib/:/run/opengl-driver/lib/:${R}/lib/R/lib:${readline}/lib
  #'';
}
