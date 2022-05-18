default:
    just --list

install-deps:
    rustup target add wasm32-unknown-unknown
    cargo install trunk

serve:
    trunk serve
