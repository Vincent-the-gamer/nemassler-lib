// @ts-expect-error .node file
import { ncm2mp3 } from "../lib"

const result = ncm2mp3("/Users/guifeng/Downloads/ncm","/Users/guifeng/Downloads/mp3")

console.log(result)