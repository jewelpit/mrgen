default:
    just --list

install-deps:
    rustup target add wasm32-unknown-unknown
    cargo install --locked trunk

serve:
    trunk serve

build *ARGS:
    trunk clean
    trunk build --release {{ARGS}}
