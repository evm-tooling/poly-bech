import { NextResponse } from 'next/server'
import { buildSearchIndex } from '@/lib/search'

export async function GET() {
  const entries = buildSearchIndex()
  return NextResponse.json(entries)
}
