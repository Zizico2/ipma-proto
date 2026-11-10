set positional-arguments

default:
    @just --list

build:
    cargo build --locked --workspace

# Build Rust bindings and a language-neutral descriptor image.
generate: build
    buf build -o target/ipma-descriptor.bin

fmt:
    cargo fmt --all
    buf format -w

fmt-check:
    cargo fmt --all --check
    buf format --diff --exit-code

check:
    cargo check --locked --workspace --all-targets

lint:
    cargo clippy --locked --workspace --all-targets -- -D warnings

test:
    cargo test --locked --workspace

proto-check:
    buf lint
    buf build -o /dev/null

proto-breaking against='.git#ref=HEAD':
    buf breaking --against "$1"

ping address='127.0.0.1:50051':
    buf curl --schema proto --protocol grpc --http2-prior-knowledge --data '{}' "http://$1/ipma.proxy.v1.ProxyService/Ping"

# All supported selections; extra forecast requests are explicitly opt-in.
smoke address='http://127.0.0.1:50051' extra_calls='0':
    cargo run --locked -p ipma-proto --example smoke -- "$1" "$2"

verify: fmt-check check lint test proto-check
