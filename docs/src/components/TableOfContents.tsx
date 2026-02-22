'use client'

import { useEffect, useRef, useState } from 'react'
import type { TocEntry } from '@/lib/mdx'

export default function TableOfContents({
  headings = [],
}: {
  headings?: TocEntry[]
}) {
  const [activeId, setActiveId] = useState<string>('')
  const [mobileOpen, setMobileOpen] = useState(false)
  const observerRef = useRef<IntersectionObserver | null>(null)
  const mobileRef = useRef<HTMLDivElement | null>(null)
  const safeHeadings = headings ?? []

  function scrollToHeading(id: string) {
    const el = document.getElementById(id)
    if (!el) return

    const mainEl = document.querySelector('main')
    if (mainEl) {
      mainEl.scrollTo({
        top: el.offsetTop - 32,
        behavior: 'smooth',
      })
    } else {
      el.scrollIntoView({ behavior: 'smooth' })
    }

    setActiveId(id)
    history.pushState(null, '', `#${id}`)
    setMobileOpen(false)
  }

  useEffect(() => {
    const elements = safeHeadings
      .map((h) => document.getElementById(h.id))
      .filter(Boolean) as HTMLElement[]

    if (elements.length === 0) return

    const scrollRoot = document.querySelector('main') || null
    observerRef.current = new IntersectionObserver(
      (entries) => {
        for (const entry of entries) {
          if (entry.isIntersecting) {
            setActiveId(entry.target.id)
            break
          }
        }
      },
      {
        root: scrollRoot,
        rootMargin: '-80px 0px -70% 0px',
        threshold: 0,
      },
    )

    for (const el of elements) {
      observerRef.current.observe(el)
    }

    return () => observerRef.current?.disconnect()
  }, [safeHeadings])

  useEffect(() => {
    const scrollContainer = document.querySelector('main') || window

    function onScroll() {
      const scrollTop =
        scrollContainer instanceof HTMLElement
          ? scrollContainer.scrollTop
          : window.scrollY
      const offset = scrollTop + 120
      let current = ''
      for (const h of safeHeadings) {
        const el = document.getElementById(h.id)
        if (el && el.offsetTop <= offset) {
          current = h.id
        }
      }
      if (current) setActiveId(current)
    }
    scrollContainer.addEventListener('scroll', onScroll, { passive: true })
    onScroll()
    return () => scrollContainer.removeEventListener('scroll', onScroll)
  }, [safeHeadings])

  useEffect(() => {
    function onClickOutside(event: MouseEvent) {
      if (!mobileRef.current) return
      if (!mobileRef.current.contains(event.target as Node)) {
        setMobileOpen(false)
      }
    }

    if (mobileOpen) {
      document.addEventListener('mousedown', onClickOutside)
    }

    return () => {
      document.removeEventListener('mousedown', onClickOutside)
    }
  }, [mobileOpen])

  if (safeHeadings.length === 0) return null

  return (
    <>
      <div
        className="xl:hidden sticky top-0 z-30 -mt-8 pb-1 mb-10 border-b border-border/60"
        ref={mobileRef}
      >
        <button
          type="button"
          onClick={() => setMobileOpen((prev) => !prev)}
          className="w-full flex items-center bg-transparent px-1 py-2.5 text-left text-foreground-secondary"
          aria-expanded={mobileOpen}
          aria-label="Toggle table of contents"
        >
          <span className="flex items-center gap-1.5 text-[0.875rem] font-medium">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="14"
              height="14"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
              className="opacity-80"
              aria-hidden="true"
            >
              <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
              <polyline points="14 2 14 8 20 8" />
              <line x1="16" y1="13" x2="8" y2="13" />
              <line x1="16" y1="17" x2="8" y2="17" />
              <line x1="10" y1="9" x2="8" y2="9" />
            </svg>
            <span>On this page</span>
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="13"
              height="13"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
              className={`transition-transform ${mobileOpen ? 'rotate-90' : ''}`}
              aria-hidden="true"
            >
              <polyline points="9 18 15 12 9 6" />
            </svg>
          </span>
        </button>

        {mobileOpen && (
          <div className="absolute left-0 right-0 top-full z-40 mt-0 rounded-xl border border-border bg-background-secondary/95 backdrop-blur-sm shadow-lg">
            <nav className="py-3 max-h-[55vh] overflow-y-auto">
              {safeHeadings.map((heading, i) => {
                const isActive = activeId === heading.id
                return (
                  <a
                    key={`${heading.id}-${i}`}
                    href={`#${heading.id}`}
                    onClick={(e) => {
                      e.preventDefault()
                      scrollToHeading(heading.id)
                    }}
                    className={`block no-underline py-1.5 px-4 transition-colors ${
                      heading.depth >= 3 ? 'pl-8 text-[0.95rem]' : 'text-[1rem]'
                    } ${
                      isActive
                        ? 'text-primary'
                        : 'text-foreground-muted hover:text-foreground'
                    }`}
                  >
                    {heading.text}
                  </a>
                )
              })}
            </nav>
          </div>
        )}
      </div>

      <aside className="hidden xl:block xl:order-2 w-[280px] shrink-0 sticky top-6 self-start h-[calc(100dvh-1.5rem)]">
        <div className="pl-10 pr-2 overflow-y-auto h-full">
          <p className="text-[0.8125rem] font-semibold text-foreground uppercase tracking-wider mb-2">
            On this page
          </p>
          <nav className="flex flex-col gap-0">
            {safeHeadings.map((heading) => {
              const isActive = activeId === heading.id
              return (
                <a
                  key={heading.id}
                  href={`#${heading.id}`}
                  onClick={(e) => {
                    e.preventDefault()
                    scrollToHeading(heading.id)
                  }}
                  className={`block text-[0.875rem] leading-snug no-underline py-1.5 transition-all duration-150 border-l-2 ${
                    heading.depth === 3 ? 'pl-12' : 'pl-8 text-[0.98rem]'
                  } ${
                    isActive
                      ? 'text-primary !border-primary'
                      : 'text-foreground-muted border-transparent hover:text-foreground hover:border-foreground-muted'
                  }`}
                >
                  {heading.text}
                </a>
              )
            })}
          </nav>
        </div>
      </aside>
    </>
  )
}
