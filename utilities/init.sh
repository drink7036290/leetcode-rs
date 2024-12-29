
###   Ubuntu   ###


apt update
apt install lsb-release sudo


# install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env"


# install cargo-binstall
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash


# install cargo-deny, lychee, and prefligit
cargo binstall --no-confirm cargo-deny lychee prefligit


# (Option 1) install cargo-spellcheck from source
apt install -y clang libclang-dev llvm
clang_lib_so="$(
  find /usr/lib/llvm-* \
    \( -type f -o -type l \) \
    \( -name 'libclang.so' -o -name 'libclang-*.so' \) \
    2>/dev/null \
  | sort -V \
  | tail -n 1
)"
echo "Detected clang_lib_so: $clang_lib_so"

clang_lib_dir="$(dirname "$clang_lib_so")"
echo "Detected clang_lib_dir: $clang_lib_dir"

LIBCLANG_PATH=$clang_lib_dir cargo binstall --no-confirm cargo-spellcheck # --locked caused yanked warnings

# (Option 2) install cargo-spellcheck from binary

wget https://github.com/drahnr/cargo-spellcheck/releases/download/v0.14.0/cargo-spellcheck-v0.14.0-x86_64-unknown-linux-gnu
chmod +x cargo-spellcheck-v0.14.0-x86_64-unknown-linux-gnu
mv cargo-spellcheck-v0.14.0-x86_64-unknown-linux-gnu cargo-spellcheck


# install commitlint
apt install -y nodejs npm
npm install --save-dev @commitlint/{cli,config-conventional}