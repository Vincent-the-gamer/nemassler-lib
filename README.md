# Napi-rs Template

Napi-rs is a framework to build Node.js libraries in Rust.

# Features

- No `ava`, test your library in `.ts` file using `tsx`
- Default using `async` feature in `napi`
- Quick clean debug build files by `yarn clean`
- Simplified CI.
- Local cross build, powered by `cross-rs`

# Dev

## Install Deps:

```shell
cargo check
yarn install
```

## Debug

```shell
yarn dev

# Cross Platform
yarn dev:cross --target x86_64-unknown-linux-gnu
```

# Build & Publish

> [!NOTE]
> Napi-rs will release multiple packages for different OS,
> so it's better to name your package using `npm scope`.

## Locally

> [!CAUTION]
> The `--use-cross` option only supported in **pre-release version** of `@napi-rs/cli` on May 10,
> 2025. Use at your own risk!

Build different binary targets using `cross-rs`.

`cross-rs` requires `Docker` or `Podman` to build this project in different OS images.

```shell
yarn build:cross --target x86_64-unknown-linux-gnu
```

## GitHub Actions

[CI.yml](.github/workflows/CI.yml)

Only:

- `x86_64-apple-darwin`
- `x86_64-pc-windows-msvc`
- `x86_64-unknown-linux-gnu`
- `aarch64-apple-darwin`
- `aarch64-pc-windows-msvc`
