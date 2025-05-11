# nemassler-lib

Node.js library of nemassler, which to transform `.ncm` audio files to `.mp3`

# Currently Supported OS

> [!IMPORTANT]
> This module is Rust-based, it uses `.node` binary file, so it is only compatible with these following systems.

Supported:
- macOS(Apple Silicon)
- Linux(GNU, x64)
- Linux(GNU, arm64)

# Installation

```shell
npm i @vince-gamer/nemassler-lib
```

# Usage

```ts
import { ncm2mp3 } from "@vince-gamer/nemassler-lib"

const msg = ncm2mp3("/path/to/ncm", "/path/to/mp3")

console.log(msg) // [ 'Output file: /path/to/xxx.mp3' ]
```