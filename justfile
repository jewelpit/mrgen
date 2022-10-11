default:
    just --list

install-deps *ARGS:
    rustup target add wasm32-unknown-unknown
    cargo install --locked trunk {{ARGS}}

serve:
    trunk serve

build *ARGS:
    trunk clean
    trunk build --release {{ARGS}}
