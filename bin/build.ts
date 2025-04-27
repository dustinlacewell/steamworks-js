import { execSync } from 'child_process'
import { copyFileSync, mkdirSync, readdirSync, existsSync } from 'fs'
import { join, dirname } from 'path'

// Platform/arch detection (as in original build.js)
const targets: Record<string, { folder: string, files: string[] }> = {
    'win32-x64': { folder: 'win64', files: ['steam_api64.dll', 'steam_api64.lib'] },
    'linux-x64': { folder: 'linux64', files: ['libsteam_api.so'] },
    'darwin-x64': { folder: 'osx', files: ['libsteam_api.dylib'] },
    'darwin-arm64': { folder: 'osx', files: ['libsteam_api.dylib'] }
}
const key = `${process.platform}-${process.arch}`
const target = targets[key]
if (!target) throw new Error('Unsupported platform/arch: ' + key)

// Build with napi CLI for a valid .node file
execSync('npx napi build --release', { stdio: 'inherit' })

// Prepare dist/<platform>/
const dist = join(__dirname, 'dist', target.folder)
mkdirSync(dist, { recursive: true })

// Copy napi_rs.node (output from napi CLI) to dist/napi_rs.node
const indexNode = join(__dirname, 'napi_rs.node')
const distNode = join(dist, 'napi_rs.node')
if (existsSync(indexNode)) {
    copyFileSync(indexNode, distNode)
}
