## configure these for your environment
PKG="bible"                                  # cargo package name
TARGET="x86_64-unknown-linux-gnu"            # remote target
ASSETS=("Rocket.toml")  # list of assets to bundle
BUILD_DIR="target/${TARGET}/release"         # cargo build directory

## ensure target toolchain is present
rustup target add $TARGET

## cross-compile
cargo zigbuild --target $TARGET --release

## bundle
tar -cvzf "${PKG}.tar.gz" "${ASSETS[@]}" -C "${BUILD_DIR}" "${PKG}"
