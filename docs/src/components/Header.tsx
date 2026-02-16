'use client'

import Image from 'next/image'
import Link from 'next/link'
import { usePathname } from 'next/navigation'
import { useEffect, useRef, useState } from 'react'
import lockupHDark from '@/assets/logo-lockup-horizontal-dark-trans.png'
import lockupHLight from '@/assets/logo-lockup-horizontal-light-trans.png'
import { Button } from '@/components/ui/button'
import SearchTrigger from './SearchTrigger'
import SidebarToggle from './SidebarToggle'
import ThemeSwitcher from './ThemeSwitcher'

function NavLink({
  href,
  external,
  children,
}: {
  href: string
  external?: boolean
  children: React.ReactNode
}) {
  const pathname = usePathname()
  const isActive = !external && pathname.startsWith(href)

  const base =
    'hidden sm:flex items-center text-sm font-medium no-underline px-3 py-1 transition-all duration-150 relative'
  const active = isActive
    ? 'text-primary after:absolute after:bottom-[-11px] after:left-0 after:right-0 after:h-[2px] after:bg-primary after:rounded-full'
    : 'text-foreground-secondary hover:text-primary'

  if (external) {
    return (
      <a
        href={href}
        target="_blank"
        rel="noopener noreferrer"
        className={`${base} ${active}`}
      >
        {children}
      </a>
    )
  }
  return (
    <Link href={href} className={`${base} ${active}`}>
      {children}
    </Link>
  )
}

export default function Header() {
  const pathname = usePathname()
  return (
    <header className="sticky top-0 z-50 w-full bg-background/95 dark:bg-background backdrop-blur-sm">
      <div className="px-4 sm:px-6 h-14 flex items-center justify-between gap-3">
        {/* Left: logo area + search */}
        <div className="flex items-center min-w-0">
          <div
            className={`flex items-center gap-3 sm:gap-4 shrink-0 ${pathname === '/' ? '' : 'lg:w-[260px] xl:w-[320px] 2xl:w-[355px] xl:pl-12'} overflow-hidden`}
          >
            <SidebarToggle />
            <Link
              href="/"
              className="flex items-center gap-2 shrink-0"
              style={{ height: '1.25rem' }}
            >
              {/* Logo same height as nav text (text-sm line-height) so it doesn't go tiny */}
              <Image
                src={lockupHDark}
                alt="poly-bench"
                className="dark-only w-auto object-contain object-center"
                style={{ height: '12rem', width: '17rem' }}
                priority
              />
              <Image
                src={lockupHLight}
                alt="poly-bench"
                className="light-only w-auto object-contain object-center"
                style={{ height: '12rem', width: '17rem' }}
                priority
              />
            </Link>
          </div>
          {pathname !== '/' && (
            <div className="hidden sm:block">
              <SearchTrigger />
            </div>
          )}
        </div>

        {/* Right: Navigation */}
        <nav className="flex items-center gap-4">
          <NavLink href="/docs/introduction">Docs</NavLink>
          <NavLink href="https://github.com/evm-tooling/poly-bench" external>
            GitHub
          </NavLink>

          <ThemeSwitcher />
          <VersionDropdown />

          <Link
            href="/docs/getting-started"
            className="hidden sm:inline-flex text-sm font-medium px-4 py-2 rounded-lg bg-primary text-primary-foreground transition-colors hover:bg-primary-hover"
          >
            Get Started
          </Link>
        </nav>
      </div>
    </header>
  )
}

function VersionDropdown() {
  const [isOpen, setIsOpen] = useState(false)
  const dropdownRef = useRef<HTMLDivElement>(null)

  // Close dropdown when clicking outside
  useEffect(() => {
    const handleClickOutside = (event: MouseEvent) => {
      if (
        dropdownRef.current &&
        !dropdownRef.current.contains(event.target as Node)
      ) {
        setIsOpen(false)
      }
    }

    if (isOpen) {
      document.addEventListener('mousedown', handleClickOutside)
    }

    return () => {
      document.removeEventListener('mousedown', handleClickOutside)
    }
  }, [isOpen])

  return (
    <div className="relative" ref={dropdownRef}>
      <Button
        variant="secondary"
        size="sm"
        className="h-auto px-3 py-1.5 rounded-md gap-1"
        onClick={() => setIsOpen(!isOpen)}
      >
        v0.1.0
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="12"
          height="12"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          strokeWidth="2"
          strokeLinecap="round"
          strokeLinejoin="round"
          className={`transition-transform ${isOpen ? 'rotate-180' : ''}`}
        >
          <polyline points="6 9 12 15 18 9" />
        </svg>
      </Button>
      <div
        className={`absolute top-full right-0 mt-2 min-w-[160px] bg-card border border-card-border rounded-lg p-1 shadow-xl shadow-black/30 transition-all z-50 ${isOpen ? 'opacity-100 visible translate-y-0' : 'opacity-0 invisible -translate-y-1'}`}
      >
        <Button
          asChild
          variant="ghost"
          size="sm"
          className="w-full justify-between px-3 py-2 h-auto rounded-md text-foreground-secondary hover:text-primary"
        >
          <a
            href="https://github.com/ChefBingbong/viem-go/releases"
            target="_blank"
            rel="noopener noreferrer"
          >
            Releases
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="12"
              height="12"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
              className="opacity-50"
            >
              <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
              <polyline points="15 3 21 3 21 9" />
              <line x1="10" y1="14" x2="21" y2="3" />
            </svg>
          </a>
        </Button>
        <Button
          asChild
          variant="ghost"
          size="sm"
          className="w-full justify-between px-3 py-2 h-auto rounded-md text-foreground-secondary hover:text-primary"
        >
          <a
            href="https://github.com/ChefBingbong/viem-go/tree/main/examples"
            target="_blank"
            rel="noopener noreferrer"
          >
            Examples
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="12"
              height="12"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
              className="opacity-50"
            >
              <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
              <polyline points="15 3 21 3 21 9" />
              <line x1="10" y1="14" x2="21" y2="3" />
            </svg>
          </a>
        </Button>
        <Button
          asChild
          variant="ghost"
          size="sm"
          className="w-full justify-between px-3 py-2 h-auto rounded-md text-foreground-secondary hover:text-primary"
        >
          <a
            href="https://github.com/ChefBingbong/viem-go/blob/main/.github/CONTRIBUTING.md"
            target="_blank"
            rel="noopener noreferrer"
          >
            Contributing
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="12"
              height="12"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="2"
              strokeLinecap="round"
              strokeLinejoin="round"
              className="opacity-50"
            >
              <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
              <polyline points="15 3 21 3 21 9" />
              <line x1="10" y1="14" x2="21" y2="3" />
            </svg>
          </a>
        </Button>
      </div>
    </div>
  )
}
