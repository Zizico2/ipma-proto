# ipma-proto (Rust)

Single Rust crate providing the `ipma.proxy.v1` messages, generated tonic
client/server bindings and reflection descriptors. The shared schemas live
in `../proto/`; the build script uses protox and tonic-prost-build without
requiring protoc. Cargo generates bindings in OUT_DIR.

Consume this crate through the repository's pinned Cargo Git dependency and
import `ipma_proto::v1::*`. There is no Cargo workspace or root-level Cargo
manifest. Cargo discovers this crate recursively in the Git checkout.

Run `just verify` in this directory for formatting, check, Clippy and tests;
`just generate` builds the generated bindings. From the repository root,
`just rust <recipe>` delegates here with the correct working directory and
pinned toolchain. See the root README for schema tooling and service examples.
