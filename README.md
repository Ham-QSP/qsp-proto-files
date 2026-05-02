# qsp-proto-files

Shared Protocol Buffers files for QSP projects, with Rust and TypeScript packaging.

## Structure

- `proto/` contains the source protobuf schemas.
- `packages/rust/` exposes a Rust crate generated with `prost`.
- `packages/typescript/` exposes a TypeScript package generated with `@bufbuild/protobuf`.
- `.github/workflows/` validates generation and publishes packages on tag.

## Local Prerequisites

- Stable Rust with Cargo.
- Node.js 20+ and npm.

The protobuf tools (`buf`, `protoc-gen-es`, `protoc`) are installed through the project dependencies.

## Commands

```sh
npm install
npm run lint
npm run generate
cargo test --workspace
```

## Add a Schema

1. Add the `.proto` file under `proto/qsp/<domain>/v1/`.
2. Declare a stable protobuf `package`, for example `qsp.user.v1`.
3. Add the file to `packages/rust/build.rs`.
4. Run `npm run generate`, then `cargo test --workspace`.

## Publish

The `.github/workflows/release.yml` workflow publishes on a `v*` tag.

Required GitHub secrets:

- `NPM_TOKEN` to publish `@qsp/proto-files`.
- `CARGO_REGISTRY_TOKEN` to publish `qsp-proto-files`.
