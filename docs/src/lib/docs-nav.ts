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
          { label: 'Reporting', slug: 'reporting' },
          { label: 'Examples', slug: 'examples' },
        ],
      },
    ],
  },
  {
    label: 'Guides',
    items: [
      {
        label: 'Usage',
        items: [
          { label: 'Charting', slug: 'guides/charting' },
          { label: 'Anvil & EVM', slug: 'guides/anvil' },
          { label: 'Fixtures', slug: 'guides/fixtures' },
          { label: 'CLI Recipes', slug: 'guides/cli-recipes' },
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
          { label: 'Architecture', slug: 'core/architecture' },
          { label: 'DSL Reference', slug: 'core/dsl-reference' },
          { label: 'Standard Library', slug: 'core/standard-library' },
        ],
      },
      {
        label: 'Tools',
        items: [
          { label: 'CLI', slug: 'tools/cli' },
          { label: 'LSP / Editor', slug: 'tools/lsp' },
        ],
      },
    ],
  },
]
