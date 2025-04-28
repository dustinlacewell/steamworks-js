import { fileURLToPath } from 'url'
import { dirname, join } from 'path'
import { copyFileSync, existsSync, mkdirSync } from 'fs'

const __dirname = dirname(fileURLToPath(import.meta.url))

// Platform/arch detection (as in original build.js)
const targets = [
    { source: 'win64', dest: 'win32-x64-msvc', files: ['steam_api64.dll', 'steam_api64.lib'] },
    { source: 'linux64', dest: 'linux-x64-gnu', files: ['libsteam_api.so'] },
    { source: 'osx', dest: 'darwin-x64', files: ['libsteam_api.dylib'] },
]

for (const target of targets) {
    const redist = join(__dirname, '../redist', target.source)
    const dist = join(__dirname, '../npm', target.dest)

    console.log('Redist:', redist)
    console.log('Dist:', dist)

    if (!existsSync(dist))
        throw new Error(`Dist directory does not exist: ${dist}`)

    for (const file of target.files) {
        const src = join(redist, file)
        const dest = join(dist, file)

        if (!existsSync(src))
            throw new Error(`Redist file does not exist: ${src}`)

        copyFileSync(src, dest)
    }
}