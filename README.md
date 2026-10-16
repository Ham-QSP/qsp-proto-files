# qsp-proto-files

Shared Protocol Buffers files for QSP projects, with Rust and TypeScript packaging.

## Structure

- `proto/` contains the source protobuf schemas.
- `packages/rust/` exposes a Rust crate generated with `prost`.
- `packages/typescript/` exposes a TypeScript package generated with `@bufbuild/protobuf`.
- `testdata/` holds golden encoded messages; the Rust and TypeScript tests must both produce and decode these exact bytes.
- `.github/workflows/` validates generation and publishes packages on tag.

## Local Prerequisites

- Stable Rust with Cargo.
- Node.js 24+ and npm.

The protobuf tools (`buf`, `protoc-gen-es`, `protoc`) are installed through the project dependencies.

## Commands

```sh
npm ci                 # install the locked dependencies (package-lock.json)
npm run lint           # buf lint
npm run format         # buf format -w (CI runs npm run format:check)
npm run generate
npm run build
npm test               # TypeScript tests (needs generate + build)
cargo test --workspace
```

When a message changes, update the golden bytes in `testdata/agent_control_messages.txt` together with both tests.

## Publish

1. Set the new version in `packages/rust/Cargo.toml` and `packages/typescript/package.json`, and commit.
2. Push a `v<version>` tag (for example `git tag v0.1.8 && git push origin v0.1.8`).

The `.github/workflows/release.yml` workflow then publishes the TypeScript package to GitHub Packages and creates the GitHub release with generated notes. Do not create the release manually beforehand.

## License

[MIT](LICENSE)
