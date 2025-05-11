import { napi } from "./package.json"
import fs from "fs"

const targets = [
    "darwin-arm64",
    "darwin-x64",
    "linux-x64-gnu",
    "linux-arm64-gnu",
    "win32-x64-msvc"
]


function deleteFile() {
    targets.forEach(target => {
        // delete .node files
        try {
            const nodeFileName = `${napi.binaryName}.${target}.node`
            if (fs.existsSync(`./npm/${target}/${nodeFileName}`)) {
                fs.unlinkSync(`./npm/${target}/${nodeFileName}`)
                console.log(`Deleted file: ./npm/${target}/${nodeFileName}`)
            } else { 
                console.log(`./npm/${target}/${nodeFileName} is not found!`) 
            }
        } catch (err) {
            console.error(`Error: ${err}`)
        }
    })

    // delete index.js and index.d.ts
    try {
        if (fs.existsSync(`./index.js`)) {
            fs.unlinkSync(`./index.js`)
            console.log(`Deleted file: ./index.js`)
        } else {
            console.log(`./index.js is not found!`)
        }

        if (fs.existsSync(`./index.d.ts`)) {
            fs.unlinkSync(`./index.d.ts`)
            console.log(`Deleted file: ./index.d.ts`)
        } else {
            console.log(`./index.d.ts is not found!`)
        }
    } catch (err) {
        console.error(`Error: ${err}`)
    }
}

deleteFile()
