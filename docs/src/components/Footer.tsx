import Image from 'next/image'
import Link from 'next/link'
import lockupHDark from '../assets/logo-lockup-horizontal-dark-trans-crop.png'
import lockupHLight from '../assets/logo-lockup-horizontal-light-trans-crop.png'

export default function Footer() {
  return (
    <footer className="w-full mt-8 border-t border-border bg-background-secondary/40">
      <div className="max-w-7xl mx-auto px-6 py-12 sm:py-14">
        <div className="grid grid-cols-1 lg:grid-cols-[1.3fr_1fr] gap-10 lg:gap-14">
          <div>
            <Link href="/" className="relative inline-block mb-4">
              <span className="relative block h-7 w-[168px] sm:w-[182px]">
                <Image
                  src={lockupHDark}
                  alt="poly-bench"
                  fill
                  sizes="(max-width: 640px) 168px, 182px"
                  className="dark-only object-contain object-left"
                  priority
                />
                <Image
                  src={lockupHLight}
                  alt="poly-bench"
                  fill
                  sizes="(max-width: 640px) 168px, 182px"
                  className="light-only object-contain object-left"
                  priority
                />
              </span>
            </Link>
            <p className="text-sm text-foreground-secondary max-w-md">
              A polyglot benchmarking framework for fair, repeatable performance
              comparisons across runtimes.
            </p>
            <p className="text-xs mt-2 text-foreground-muted">
              Built in Rust · Open Source · MIT License
            </p>
          </div>

          <div className="grid grid-cols-2 sm:grid-cols-3 gap-8 sm:gap-10">
            <div>
              <h5 className="text-xs font-bold uppercase tracking-[0.18em] mb-3 text-foreground-muted">
                Docs
              </h5>
              <ul className="space-y-2 text-sm text-foreground-secondary">
                <li>
                  <Link href="/docs/introduction" className="hover:text-primary transition-colors">
                    Introduction
                  </Link>
                </li>
                <li>
                  <Link href="/docs/getting-started" className="hover:text-primary transition-colors">
                    Getting Started
                  </Link>
                </li>
                <li>
                  <Link href="/docs/tools/cli" className="hover:text-primary transition-colors">
                    CLI Reference
                  </Link>
                </li>
              </ul>
            </div>

            <div>
              <h5 className="text-xs font-bold uppercase tracking-[0.18em] mb-3 text-foreground-muted">
                Project
              </h5>
              <ul className="space-y-2 text-sm text-foreground-secondary">
                <li>
                  <Link href="/docs/examples" className="hover:text-primary transition-colors">
                    Examples
                  </Link>
                </li>
                <li>
                  <Link href="/docs/reporting" className="hover:text-primary transition-colors">
                    Reporting
                  </Link>
                </li>
                <li>
                  <Link href="/docs/core/architecture" className="hover:text-primary transition-colors">
                    Architecture
                  </Link>
                </li>
              </ul>
            </div>

            <div className="col-span-2 sm:col-span-1">
              <h5 className="text-xs font-bold uppercase tracking-[0.18em] mb-3 text-foreground-muted">
                Community
              </h5>
              <ul className="space-y-2 text-sm text-foreground-secondary">
                <li>
                  <a
                    href="https://github.com/evm-tooling/poly-bench"
                    target="_blank"
                    rel="noreferrer"
                    className="hover:text-primary transition-colors"
                  >
                    GitHub
                  </a>
                </li>
                <li>
                  <a
                    href="https://github.com/evm-tooling/poly-bench/discussions"
                    target="_blank"
                    rel="noreferrer"
                    className="hover:text-primary transition-colors"
                  >
                    Discussions
                  </a>
                </li>
                <li>
                  <a
                    href="https://github.com/evm-tooling/poly-bench/issues"
                    target="_blank"
                    rel="noreferrer"
                    className="hover:text-primary transition-colors"
                  >
                    Issues
                  </a>
                </li>
              </ul>
            </div>
          </div>
        </div>

        <div className="mt-10 pt-6 border-t border-border/70 flex flex-col sm:flex-row items-start sm:items-center justify-between gap-2 text-xs text-foreground-muted">
          <p>Released under the MIT License.</p>
          <p>© {new Date().getFullYear()} poly-bench.</p>
        </div>
      </div>
    </footer>
  )
}
