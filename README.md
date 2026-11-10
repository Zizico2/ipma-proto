# ipma-proto

Shared API contract for [ipma-proxy](https://github.com/Zizico2/ipma-proxy)
and [ipma-gateway](https://github.com/Zizico2/ipma-gateway). Both implement
`ipma.proxy.v1.ProxyService`; clients select either endpoint.

`proto/` is the language-neutral source of truth. The repository root contains
schemas, Buf configuration and orchestration recipes. Each language owns its
package manifests, lockfiles, toolchain, generators and recipes in its directory.

```text
proto/                  # shared .proto definitions
buf.yaml                # schema lint/build/breaking rules
justfile                # schema tools and language orchestration
rust/
  Cargo.toml            # single ipma-proto crate; no Cargo workspace
  Cargo.lock
  rust-toolchain.toml
  justfile              # Rust build/check/lint/test/smoke recipes
  build.rs              # protox + tonic-prost-build generation
  src/
  examples/
```

Future language bindings belong in sibling language directories such as `go/`
or `typescript/`, with generators reading `proto/`. Application/cache policy
stays in the service repositories.

The Rust build script reads `../proto` and generates messages, clients, server
traits and reflection descriptors into `OUT_DIR`. Generated Rust files are not
committed; protoc is not required. The full Git checkout contains the shared
schemas outside the crate directory.

```toml
[dependencies]
ipma-proto = { git = "https://github.com/Zizico2/ipma-proto.git", rev = "<full commit SHA>" }
```

Cargo discovers the crate under `rust/` in Git dependencies; consumers do not
need a root Cargo.toml. Import `ipma_proto::v1::*`,
`ipma_proto::FILE_DESCRIPTOR_SET` and `ipma_proto::MAX_MESSAGE_BYTES`. Pin a
commit and commit Cargo.lock so both services consume the same API revision.

Category wrappers use `value` for their enum and retain the source code/text.
Parent fields carry domain names such as `warning_level` and `weather_type`.
Reference-table entries combine a `classification` oneof with their original
code and bilingual descriptions. See [category semantics](docs/categories.md)
for missing values, unknown categories and compatibility rules.

From the root:

- `just proto-check` runs schema lint/build checks.
- `just proto-breaking <baseline>` compares schemas to a Git revision/repository
  or Buf image. Evolve additively and reserve removed names/numbers.
- `just descriptors` writes `build/ipma-descriptor.bin` without requiring Rust.
- `just generate` writes descriptors and generates Rust bindings.
- `just verify` runs schema checks and the Rust verification recipe.
- `just rust <recipe>` runs a Rust recipe with `rust/` as its working directory,
  so rustup discovers that directory's pinned toolchain.

You can also run `cd rust && just verify` or `cd rust && cargo build --locked`.
The nested justfile contains the Rust commands. Just and Buf are required for
all root developer checks; the Rust toolchain is pinned in
`rust/rust-toolchain.toml`.

`just ping <host:port>` uses the local schema. `just smoke <http://host:port>`
(or `just rust smoke ...`) exercises all 26 data selections. Its optional second
argument adds forecast calls; leave it zero for the adapter because every data
RPC contacts IPMA.

`SourceMetadata.fetched_at` reports the adapter's fetch time, not measurement
time. `expires_at` is optional: the adapter leaves it unset unless IPMA supplies
an expiry, while the gateway can populate cache expiry and preserve original
source URLs, fetch/update times and attribution.

Extracted from ipma-proxy commit c47bd44. No data field numbers or RPC wire paths
changed in the repository/layout split.
