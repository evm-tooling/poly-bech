import { readFileSync } from 'fs'
import type { NextConfig } from 'next'
import { join } from 'path'

function getPolybenchVersion(): string {
  try {
    const cargoPath = join(process.cwd(), '..', 'Cargo.toml')
    const cargo = readFileSync(cargoPath, 'utf-8')
    const match = cargo.match(/^version\s*=\s*"([^"]+)"/m)
    return match ? match[1] : '0.1.0'
  } catch {
    return '0.1.0'
  }
}

const polybenchVersion = getPolybenchVersion()

const nextConfig: NextConfig = {
  env: {
    NEXT_PUBLIC_POLYBENCH_VERSION: polybenchVersion,
  },
}

export default nextConfig
