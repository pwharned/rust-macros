export RUSTUP_USE_CURL=1
script=$(dirname "$0")
if ls $script/target/classes/server > /dev/null; then 
  echo "server has already been built"
  rm -rf "$1"
  exit 0
fi
if ! which cargo > /dev/null; then
    echo "Cargo not on path"
    export RUSTUP_HOME=$1
    export CARGO_HOME=$RUSTUP_HOME/cargo
    export PATH=$PATH:$CARGO_HOME/bin > /dev/null 2>&1
    if ! which cargo > /dev/null; then
    echo "Cargo not found, installing"
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh  -s -- --default-toolchain="stable-x86_64-unknown-linux-gnu" -y
    $RUSTUP_HOME/cargo/bin/rustup default stable
    fi
else
  echo "Rust is already installed"
fi

