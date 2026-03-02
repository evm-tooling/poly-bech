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
          { label: 'Installation', slug: 'introduction/installation' },
          { label: 'Platform Compatibility', slug: 'platform-compatibility' },
          { label: 'Getting Started', slug: 'getting-started' },
        ],
      },
    ],
  },
  {
    label: 'Features',
    items: [
      { label: 'Overview', slug: 'features' },
      {
        label: 'Bench Semantics',
        items: [
          { label: 'Suites', slug: 'features/suites' },
          { label: 'Setup Blocks', slug: 'features/setup' },
          { label: 'Benchmarks', slug: 'features/benchmarks' },
          { label: 'Fixtures', slug: 'features/fixtures' },
          { label: 'globalSetup', slug: 'features/global-setup' },
        ],
      },
      {
        label: 'Standard Library',
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
    label: 'Architecture',
    items: [
      { label: 'Overview', slug: 'architecture' },
      {
        label: 'Polybench Runtime',
        items: [
          { label: 'Overview', slug: 'architecture/polybench-runtime' },
          { label: 'Go', slug: 'architecture/polybench-runtime/go' },
          { label: 'TypeScript', slug: 'architecture/polybench-runtime/ts' },
          { label: 'Rust', slug: 'architecture/polybench-runtime/rust' },
          { label: 'Python', slug: 'architecture/polybench-runtime/python' },
          { label: 'C', slug: 'architecture/polybench-runtime/c' },
          { label: 'C#', slug: 'architecture/polybench-runtime/csharp' },
          { label: 'Zig', slug: 'architecture/polybench-runtime/zig' },
        ],
      },
      {
        label: 'Components',
        items: [
          { label: 'DSL', slug: 'architecture/dsl' },
          { label: 'Executor', slug: 'architecture/executor' },
          { label: 'Reporter', slug: 'architecture/reporter' },
          { label: 'LSP v2', slug: 'architecture/lsp-v2' },
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
  {
    label: 'Developers',
    items: [
      { label: 'Overview', slug: 'developers' },
      {
        label: 'Runtime Not Available',
        slug: 'developers/runtime-not-available',
      },
      { label: 'Adding a Runtime', slug: 'developers/adding-a-runtime' },
      { label: 'Case Study: Python', slug: 'developers/case-study-python' },
    ],
  },
]
