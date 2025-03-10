// @ts-expect-error .node file
import { ncm2mp3 } from "../lib"

const result = ncm2mp3("/Users/Shared/ncm","/Users/Shared/mp3")

console.log(result)