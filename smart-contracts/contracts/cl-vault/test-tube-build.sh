DIRNAME=$(dirname $0)

git submodule update --remote

cd $DIRNAME/../cw-dex-router

cargo build --release --lib --target wasm32-unknown-unknown --target-dir ../cl-vault/test-tube-build/cw-dex-router

cd ../$DIRNAME

cargo build --release --lib --target wasm32-unknown-unknown --target-dir ./test-tube-build/cl-vault
