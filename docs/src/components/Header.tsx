'use client'

import Image from 'next/image'
import Link from 'next/link'
import { usePathname } from 'next/navigation'
import { useEffect, useRef, useState } from 'react'
import benchLogo from '../assets/bench-logo-transparent.png'
import lockupHDark from '../assets/logo-lockup-horizontal-dark-trans-crop.png'
import lockupHLight from '../assets/logo-lockup-horizontal-light-trans-crop.png'
import { Button } from '../components/ui/button'
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
    'hidden md:flex items-center text-sm font-medium no-underline px-2.5 lg:px-3 py-1 transition-all duration-150 relative'
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
  const isHome = pathname === '/'
  return (
    <header className="sticky top-0 z-50 w-full bg-background/95 dark:bg-background backdrop-blur-sm">
      <div className="px-4 sm:px-6 h-14 flex items-center justify-between gap-3">
        {/* Left: logo area + search */}
        <div className="flex items-center min-w-0">
          <div
            className={`flex items-center gap-2 sm:gap-3 shrink-0 ${pathname === '/' ? '' : 'lg:w-[220px] xl:w-[300px] 2xl:w-[355px] xl:pl-12'} overflow-hidden`}
          >
            {isHome ? (
              <>
                <Link
                  href="/"
                  className="relative shrink-0 hidden max-[420px]:block h-10 w-10"
                >
                  <Image
                    src={benchLogo}
                    alt="poly-bench"
                    fill
                    sizes="40px"
                    className="object-contain object-left"
                    priority
                  />
                </Link>
                <Link
                  href="/"
                  className="relative shrink-0 block max-[420px]:hidden"
                >
                  <span className="relative block h-6 w-[146px] md:w-[158px] lg:w-[168px] xl:w-[182px]">
                    <Image
                      src={lockupHDark}
                      alt="poly-bench"
                      fill
                      sizes="(max-width: 768px) 146px, (max-width: 1024px) 158px, (max-width: 1280px) 168px, 182px"
                      className="dark-only object-contain object-left"
                      priority
                    />
                    <Image
                      src={lockupHLight}
                      alt="poly-bench"
                      fill
                      sizes="(max-width: 768px) 146px, (max-width: 1024px) 158px, (max-width: 1280px) 168px, 182px"
                      className="light-only object-contain object-left"
                      priority
                    />
                  </span>
                </Link>
              </>
            ) : (
              <>
                <Link
                  href="/"
                  className="relative shrink-0 block lg:hidden h-12 w-12 mr-6"
                >
                  <Image
                    src={benchLogo}
                    alt="poly-bench"
                    fill
                    sizes="48px"
                    className="object-contain object-left"
                    priority
                  />
                </Link>
                <Link href="/" className="relative shrink-0 hidden lg:block">
                  <span className="relative block h-6 w-[148px] xl:w-[158px]">
                    <Image
                      src={lockupHDark}
                      alt="poly-bench"
                      fill
                      sizes="(max-width: 1280px) 148px, 158px"
                      className="dark-only object-contain object-left"
                      priority
                    />
                    <Image
                      src={lockupHLight}
                      alt="poly-bench"
                      fill
                      sizes="(max-width: 1280px) 148px, 158px"
                      className="light-only object-contain object-left"
                      priority
                    />
                  </span>
                </Link>
              </>
            )}
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
            className="hidden lg:inline-flex text-sm font-medium px-4 py-2 rounded-lg bg-primary text-primary-foreground transition-colors hover:bg-primary-hover"
          >
            Read the Docs
          </Link>
          <SidebarToggle className="ml-1" />
        </nav>
      </div>
    </header>
  )
}

const POLYBENCH_VERSION =
  typeof process !== 'undefined' && process.env.NEXT_PUBLIC_POLYBENCH_VERSION
    ? process.env.NEXT_PUBLIC_POLYBENCH_VERSION
    : '0.1.0'

const GITHUB_BASE = 'https://github.com/evm-tooling/poly-bench'

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
        v{POLYBENCH_VERSION}
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
            href={`${GITHUB_BASE}/releases`}
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
            href={`${GITHUB_BASE}/tree/main/examples`}
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
            href={`${GITHUB_BASE}/blob/main/.github/CONTRIBUTING.md`}
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
