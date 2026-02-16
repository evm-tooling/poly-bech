/**
 * One-off: make white/near-white background transparent in logo PNGs
 * Run: node scripts/make-logo-transparent.mjs
 */

import { dirname, join } from 'node:path'
import { fileURLToPath } from 'node:url'
import sharp from 'sharp'

const __dirname = dirname(fileURLToPath(import.meta.url))
const assetsDir = join(__dirname, '../src/assets')

const files = [
  'logo-lockup-horizontal-light.png',
  'logo-lockup-horizontal-dark.png',
]

const WHITE_THRESHOLD = 250 // treat RGB >= this as background

for (const name of files) {
  const path = join(assetsDir, name)
  const img = sharp(path)
  const meta = await img.metadata()
  const { data, info } = await img
    .ensureAlpha()
    .raw()
    .toBuffer({ resolveWithObject: true })
  const { width, height, channels } = info

  for (let i = 0; i < data.length; i += channels) {
    const r = data[i]
    const g = data[i + 1]
    const b = data[i + 2]
    if (r >= WHITE_THRESHOLD && g >= WHITE_THRESHOLD && b >= WHITE_THRESHOLD) {
      data[i + 3] = 0 // alpha = 0
    }
  }

  await sharp(data, { raw: { width, height, channels } }).png().toFile(path)
  console.log('Updated:', name)
}
