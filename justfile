set positional-arguments

default:
    @just --list

# Run a Rust-specific recipe with rust/ as its working directory.
rust *args:
    just --justfile rust/justfile "$@"

# Generate language-neutral descriptors and the available language bindings.
generate: descriptors
    just --justfile rust/justfile generate

descriptors:
    mkdir -p build
    buf build -o build/ipma-descriptor.bin

fmt:
    buf format -w
    just --justfile rust/justfile fmt

fmt-check:
    buf format --diff --exit-code
    just --justfile rust/justfile fmt-check

proto-check:
    buf lint
    buf build -o /dev/null

proto-breaking against='.git#ref=HEAD':
    buf breaking --against "$1"

ping address='127.0.0.1:50051':
    buf curl --schema proto --protocol grpc --http2-prior-knowledge --data '{}' "http://$1/ipma.proxy.v1.ProxyService/Ping"

# Compatibility shortcut; the implementation belongs to the Rust recipes.
smoke address='http://127.0.0.1:50051' extra_calls='0':
    just --justfile rust/justfile smoke "$1" "$2"

verify: proto-check
    buf format --diff --exit-code
    just --justfile rust/justfile verify
