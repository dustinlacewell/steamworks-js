import { fileURLToPath } from 'url'
import { dirname, join } from 'path'
import { copyFileSync, existsSync } from 'fs'

const __dirname = dirname(fileURLToPath(import.meta.url))

// Platform/arch detection (as in original build.js)
const targets ={
    'win32-x64': { folder: 'win64', files: ['steam_api64.dll', 'steam_api64.lib'] },
    'linux-x64': { folder: 'linux64', files: ['libsteam_api.so'] },
    'darwin-x64': { folder: 'osx', files: ['libsteam_api.dylib'] },
    'darwin-arm64': { folder: 'osx', files: ['libsteam_api.dylib'] }
}
const key = `${process.platform}-${process.arch}`
const target = targets[key]
if (!target) throw new Error('Unsupported platform/arch: ' + key)

const redist = join(__dirname, 'redist', target.folder)
const dist = join(__dirname, 'dist', target.folder)
if (existsSync(redist)) {
    copyFileSync(redist, dist)
}