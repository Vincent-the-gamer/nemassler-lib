# nemassler-lib

A Node.js library of nemassler, built in Rust.

[![npm version][npm-version-src]][npm-version-href]
[![npm downloads][npm-downloads-src]][npm-downloads-href]
[![bundle][bundle-src]][bundle-href]
[![JSDocs][jsdocs-src]][jsdocs-href]
[![License][license-src]][license-href]

> [!NOTE]
> Work in progress.

# Usage

## Install dependency
```shell
npm i nemassler-lib
```

## Use in Node.js
```ts
import { ncm2mp3 } from "nemassler-lib"

const result = ncm2mp3("/xxx/ncm","/xxx/mp3")
```

# Dev
```shell
pnpm i && cargo check

pnpm run debug
```

# License
[MIT License @Vincent-the-gamer 2025-PRESENT](./LICENSE)

[npm-version-src]: https://img.shields.io/npm/v/nemassler-lib?style=flat&colorA=080f12&colorB=1fa669
[npm-version-href]: https://npmjs.com/package/nemassler-lib
[npm-downloads-src]: https://img.shields.io/npm/dm/nemassler-lib?style=flat&colorA=080f12&colorB=1fa669
[npm-downloads-href]: https://npmjs.com/package/nemassler-lib
[bundle-src]: https://img.shields.io/bundlephobia/minzip/nemassler-lib?style=flat&colorA=080f12&colorB=1fa669&label=minzip
[bundle-href]: https://bundlephobia.com/result?p=nemassler-lib
[license-src]: https://img.shields.io/github/license/Vincent-the-gamer/nemassler-lib.svg?style=flat&colorA=080f12&colorB=1fa669
[license-href]: https://github.com/Vincent-the-gamer/nemassler-lib/blob/main/LICENSE
[jsdocs-src]: https://img.shields.io/badge/jsdocs-reference-080f12?style=flat&colorA=080f12&colorB=1fa669
[jsdocs-href]: https://www.jsdocs.io/package/nemassler-lib