/**
 * DocsTable — feature/status table for documentation pages
 *
 * Uses tokens: background-elevated, background-secondary, border,
 * foreground, foreground-secondary, primary
 */

interface DocsTableProps {
  headers: string[]
  rows: (string | React.ReactNode)[][]
}

export default function DocsTable({
  headers = [],
  rows = [],
}: DocsTableProps) {
  const safeHeaders = headers ?? []
  const safeRows = rows ?? []
  return (
    <div className="my-6 overflow-hidden rounded-lg border border-border">
      {/* Header row */}
      <div className="flex bg-background-elevated px-4 py-2.5 border-b border-border">
        {safeHeaders.map((h, i) => (
          <span
            key={i}
            className="flex-1 text-[var(--fs-fine)] font-semibold uppercase tracking-wider text-foreground"
          >
            {h}
          </span>
        ))}
      </div>

      {/* Data rows */}
      {safeRows.map((row, ri) => (
        <div
          key={ri}
          className="flex px-4 py-2.5 border-b border-border/30 last:border-b-0 hover:bg-background-secondary/60 transition-colors"
        >
          {(Array.isArray(row) ? row : []).map((cell, ci) => (
            <span
              key={ci}
              className={`flex-1 text-[var(--fs-small)] ${
                ci === 0
                  ? 'font-medium text-foreground'
                  : 'text-foreground-secondary'
              }`}
            >
              {cell}
            </span>
          ))}
        </div>
      ))}
    </div>
  )
}
