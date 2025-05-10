import { napi } from "./package.json"
import fs from "fs"

const dotNodeFiles = [
    "darwin-arm64",
    "darwin-x64",
    "linux-x64-gnu",
    "win32-x64-msvc"
].map(i => `${napi.binaryName}.${i}.node`)

const buildFileList = [
    "index.js",
    "index.d.ts",
    ...dotNodeFiles
]

function deleteFile() {
    buildFileList.forEach(file => {
        try {
            if(fs.existsSync(`./${file}`)) {
                fs.unlinkSync(`./${file}`)
                console.log(`Deleted file: ./${file}`)
            }
            console.log(`./${file} is not found!`)
        } catch(err) {
            console.error(`Error: ${err}`)
        }
   })
}

deleteFile()
