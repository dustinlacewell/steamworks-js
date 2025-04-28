import { copyFileSync, existsSync } from "fs"
import { join } from "path"

const { platform, arch } = process

const key = `${platform}-${arch}`

const targets = {
    'win32-x64': { source: 'win32-x64-msvc', files: ['steam_api64.dll', 'steam_api64.lib'] },
    'linux-x64': { source: 'linux-x64-gnu', files: ['libsteam_api.so'] },
    'darwin-x64': { source: 'darwin-x64', files: ['libsteam_api.dylib'] },
}

const target = targets[key]

if (target === null) {
    throw new Error(`Unsupported platform/arch: ${platform}-${arch}`)
}

for (const file of target.files) {
    const src = join(process.cwd(), "npm", target.source, file)

    if (!existsSync(src)) {
        throw new Error(`Missing redistributable files for ${platform}-${arch}: ${src}`)
    }

    const dest = join(__dirname, "..", file)
    copyFileSync(src, dest)
}

