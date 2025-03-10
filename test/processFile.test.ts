// @ts-expect-error .node file
import { processFile } from "../lib"

const result = processFile("/Users/Shared/ncm","/Users/Shared/mp3", "test.ncm")

console.log(result)