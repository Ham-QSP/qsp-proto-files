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
npm run build
cargo test --workspace
```

## Publish
Set the future version and commit in:
- Cargo.toml
- package.json

The create a github release and tag.
The `.github/workflows/release.yml` workflow publishes the TypeScript package to GitHub Packages on a `v*` tag.
