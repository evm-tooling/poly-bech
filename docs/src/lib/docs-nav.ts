export interface NavItem {
  label: string
  slug?: string
  items?: NavItem[]
}

export interface FlatNavPage {
  label: string
  slug: string
}

/** Flatten the nested nav tree into an ordered list of pages (only items with slugs). */
export function getFlatNavPages(nav: NavItem[] = docsNav): FlatNavPage[] {
  const pages: FlatNavPage[] = []
  function walk(items: NavItem[]) {
    for (const item of items) {
      if (item.slug) {
        pages.push({ label: item.label, slug: item.slug })
      }
      if (item.items) {
        walk(item.items)
      }
    }
  }
  walk(nav)
  return pages
}

/** Given a slug, return the previous and next pages in nav order. */
export function getAdjacentPages(slug: string): {
  prev: FlatNavPage | null
  next: FlatNavPage | null
} {
  const pages = getFlatNavPages()
  const index = pages.findIndex((p) => p.slug === slug)
  if (index === -1) return { prev: null, next: null }
  return {
    prev: index > 0 ? pages[index - 1] : null,
    next: index < pages.length - 1 ? pages[index + 1] : null,
  }
}

export const docsNav: NavItem[] = [
  {
    label: 'Introduction',
    items: [
      {
        label: 'Overview',
        items: [
          { label: 'Why poly-bench', slug: 'introduction' },
          { label: 'Getting Started', slug: 'getting-started' },
          { label: 'Reporting', slug: 'performance' },
          { label: 'Examples', slug: 'examples' },
        ],
      },
    ],
  },
  {
    label: 'Reference',
    items: [
      {
        label: 'Core',
        items: [
          { label: 'Architecture', slug: 'clients/intro' },
          { label: 'DSL Reference', slug: 'clients/public' },
          { label: 'Standard Library', slug: 'clients/wallet' },
        ],
      },
      {
        label: 'Tools',
        items: [
          { label: 'CLI', slug: 'clients/transports/http' },
          { label: 'LSP / Editor', slug: 'clients/transports/websocket' },
        ],
      },
    ],
  },
]
