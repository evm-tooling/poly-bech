import type { Metadata } from 'next'

/**
 * Site-wide SEO configuration for poly-bench documentation
 */

export const siteConfig = {
  name: 'poly-bench',
  title: 'poly-bench · Write Once, Benchmark Everywhere',
  description:
    'A cross-language benchmarking framework. Write benchmarks once in a clean DSL and run them natively across Go, TypeScript, Rust, and more.',
  url: 'https://poly-bench.rs',
  ogImage: '/og-image.png',
  creator: '@polybench',
  keywords: [
    'benchmarking',
    'cross-language benchmarking',
    'poly-bench',
    'Rust',
    'Go',
    'TypeScript',
    'performance',
    'DSL',
    'benchmark DSL',
    'multi-language',
    'developer tools',
    'CLI',
    'EVM',
    'performance testing',
    'code benchmarks',
    'statistical analysis',
    'native performance',
  ] as string[],
  authors: [
    {
      name: 'poly-bench',
      url: 'https://poly-bench.rs',
    },
  ] as Array<{ name: string; url: string }>,
  links: {
    github: 'https://github.com/evm-tooling/poly-bench',
  },
}

/**
 * Base metadata shared across all pages
 */
export function createBaseMetadata(): Metadata {
  return {
    metadataBase: new URL(siteConfig.url),
    title: {
      default: siteConfig.title,
      template: `%s - ${siteConfig.name}`,
    },
    description: siteConfig.description,
    keywords: siteConfig.keywords,
    authors: siteConfig.authors,
    creator: siteConfig.creator,
    publisher: siteConfig.name,
    formatDetection: {
      email: true,
      address: false,
      telephone: false,
    },
    openGraph: {
      type: 'website',
      locale: 'en_US',
      url: siteConfig.url,
      title: siteConfig.title,
      description: siteConfig.description,
      siteName: siteConfig.name,
      images: [
        {
          url: siteConfig.ogImage,
          width: 1200,
          height: 630,
          alt: `${siteConfig.name} - Write Once, Benchmark Everywhere`,
        },
      ],
    },
    twitter: {
      card: 'summary_large_image',
      title: siteConfig.title,
      description: siteConfig.description,
      images: [siteConfig.ogImage],
      creator: siteConfig.creator,
    },
    robots: {
      index: true,
      follow: true,
      nocache: false,
      googleBot: {
        index: true,
        follow: true,
        noimageindex: false,
        'max-video-preview': -1,
        'max-image-preview': 'large',
        'max-snippet': -1,
      },
    },
    icons: {
      icon: [
        {
          url: '/favicons/bench-favicon.png',
          media: '(prefers-color-scheme: light)',
        },
        {
          url: '/favicons/bench-favicon.png',
          media: '(prefers-color-scheme: dark)',
        },
      ],
      apple: '/favicons/bench-favicon.png',
    },
    manifest: '/manifest.json',
    alternates: {
      canonical: siteConfig.url,
    },
    category: 'technology',
  }
}

/**
 * Generate metadata for documentation pages
 */
export function createDocsMetadata({
  title,
  description,
  slug,
}: {
  title: string
  description?: string
  slug: string
}): Metadata {
  const pageTitle = title
  const pageDescription = description || siteConfig.description
  const pageUrl = `${siteConfig.url}/docs/${slug}`

  return {
    title: pageTitle,
    description: pageDescription,
    openGraph: {
      type: 'article',
      title: `${pageTitle} - ${siteConfig.name}`,
      description: pageDescription,
      url: pageUrl,
      siteName: siteConfig.name,
      images: [
        {
          url: siteConfig.ogImage,
          width: 1200,
          height: 630,
          alt: `${pageTitle} - ${siteConfig.name}`,
        },
      ],
    },
    twitter: {
      card: 'summary_large_image',
      title: `${pageTitle} - ${siteConfig.name}`,
      description: pageDescription,
      images: [siteConfig.ogImage],
      creator: siteConfig.creator,
    },
    alternates: {
      canonical: pageUrl,
    },
  }
}

/**
 * JSON-LD structured data for the homepage
 */
export function getHomePageJsonLd() {
  return {
    '@context': 'https://schema.org',
    '@type': 'SoftwareApplication',
    name: siteConfig.name,
    applicationCategory: 'DeveloperApplication',
    operatingSystem: 'Cross-platform',
    description: siteConfig.description,
    url: siteConfig.url,
    author: {
      '@type': 'Organization',
      name: siteConfig.name,
      url: siteConfig.url,
    },
    offers: {
      '@type': 'Offer',
      price: '0',
      priceCurrency: 'USD',
    },
    programmingLanguage: 'Rust',
  }
}

/**
 * JSON-LD structured data for documentation pages
 */
export function getDocsPageJsonLd({
  title,
  description,
  slug,
  dateModified,
}: {
  title: string
  description?: string
  slug: string
  dateModified?: Date
}) {
  return {
    '@context': 'https://schema.org',
    '@type': 'TechArticle',
    headline: title,
    description: description || siteConfig.description,
    url: `${siteConfig.url}/docs/${slug}`,
    author: {
      '@type': 'Organization',
      name: siteConfig.name,
      url: siteConfig.url,
    },
    publisher: {
      '@type': 'Organization',
      name: siteConfig.name,
      url: siteConfig.url,
    },
    ...(dateModified && {
      dateModified: dateModified.toISOString(),
    }),
    inLanguage: 'en-US',
    isAccessibleForFree: true,
    about: {
      '@type': 'ComputerLanguage',
      name: 'Rust',
    },
  }
}

/**
 * JSON-LD for the documentation website
 */
export function getWebsiteJsonLd() {
  return {
    '@context': 'https://schema.org',
    '@type': 'WebSite',
    name: siteConfig.name,
    url: siteConfig.url,
    description: siteConfig.description,
    potentialAction: {
      '@type': 'SearchAction',
      target: {
        '@type': 'EntryPoint',
        urlTemplate: `${siteConfig.url}/docs?search={search_term_string}`,
      },
      'query-input': 'required name=search_term_string',
    },
  }
}
