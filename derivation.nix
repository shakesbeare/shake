{ stdenv, rustPlatform }: 
rustPlatform.buildRustPackage rec {
    name = "Shake-${version}";
    version = "0.1.3";
    cargoLock = {
        lockFile = ./Cargo.lock;
    };

    src = ./.;
    nativeBuildInputs = [];
    buildInputs = [];
}
