# ipma-proto

Shared API contract for [ipma-proxy](https://github.com/Zizico2/ipma-proxy)
and [ipma-gateway](https://github.com/Zizico2/ipma-gateway). Both implement
`ipma.proxy.v1.ProxyService`; clients select either endpoint.

`proto/` is the language-neutral source of truth. `rust/ipma-proto` owns the
protox / tonic-prost-build generation, generated Rust messages, clients, server
traits and the reflection descriptor set. Cargo generates Rust bindings in
`OUT_DIR`; no generated Rust files are committed and protoc is not required.
The Git checkout contains the shared schemas that the Rust build script imports.

```toml
[dependencies]
ipma-proto = { git = "https://github.com/Zizico2/ipma-proto.git", rev = "<full commit SHA>" }
```

Import `ipma_proto::v1::*`, `ipma_proto::FILE_DESCRIPTOR_SET` and
`ipma_proto::MAX_MESSAGE_BYTES`. Consumers should pin a commit and commit their
Cargo.lock so both services use the same contract revision.

`SourceMetadata.fetched_at` reports the adapter's fetch time; it is not the
measurement time. `expires_at` is optional. The adapter leaves it unset unless
IPMA supplies an expiry. The gateway can populate its cache expiry while
preserving the original source URL, fetch/update times and attribution.

Run `just verify` for Rust and schema checks. `just generate` builds the Rust
crate and writes a language-neutral descriptor image to
`target/ipma-descriptor.bin`. `just proto-breaking <baseline>` compares against
a Git revision, repository or Buf image. Evolve schemas additively and reserve
removed names/numbers.

Future language bindings belong in sibling language directories (e.g. `go/`
or `typescript/`), with their own generator configuration reading `proto/`.
The shared schemas are authoritative; application/cache policy stays in the
service repositories.

`just ping <host:port>` uses the local schema. `just smoke <http://host:port>`
exercises all 26 supported data selections. Its optional second argument adds
forecast calls for an explicit performance check; leave it zero for the adapter
because every data RPC contacts IPMA. Install Rust (pinned in
`rust-toolchain.toml`), Just and Buf for all developer commands.

Extracted from ipma-proxy commit c47bd44. No data field numbers or RPC wire
paths changed in this split.
