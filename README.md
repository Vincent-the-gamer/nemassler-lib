# nemassler-lib

Napi-rs is a framework to build Node.js libraries in Rust.

> [!NOTE]
> Work in progress.

# Installation

```shell
npm i @vince-gamer/nemassler-lib
```

# Usage

```ts
import { ncm2mp3 } from "@vince-gamer/nemassler-lib"

const msg = ncm2mp3("~/Downloads/ncm", "~/Downloads/mp3")

console.log(msg) // [ 'Output file: ~/Downloads/mp3/xxx.mp3' ]
```