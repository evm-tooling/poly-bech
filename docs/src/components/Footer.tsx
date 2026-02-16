import Image from 'next/image'
import Link from 'next/link'
import lockupHDark from '../assets/logo-lockup-horizontal-dark-trans.png'
import lockupHLight from '../assets/logo-lockup-horizontal-light-trans.png'

export default function Footer() {
  return (
    <footer className="w-full bg-background border-t border-border pt-16 pb-8 mt-8">
      <div className="max-w-6xl mx-auto px-6">
        <div className="flex flex-col md:flex-row justify-between items-start gap-8">
          <div>
            <Link
              href="/"
              className="flex -ml-12 mb-4 items-center justify-start shrink-0"
              style={{ height: '1.25rem' }}
            >
              {/* Logo same height as nav text (text-sm line-height) so it doesn't go tiny */}
              <Image
                src={lockupHDark}
                alt="poly-bench"
                className="dark-only w-auto object-contain "
                style={{ height: '12rem', width: '17rem' }}
                priority
              />
              <Image
                src={lockupHLight}
                alt="poly-bench"
                className="light-only w-auto object-contain "
                style={{ height: '12rem', width: '17rem' }}
                priority
              />
            </Link>
            <p className="text-sm" style={{ color: '#9ca3af' }}>
              Polyglot Benchmarking Framework
            </p>
            <p className="text-xs mt-1" style={{ color: '#d1d5db' }}>
              Built in Rust · MIT License
            </p>
          </div>
          <div className="flex gap-16">
            <div>
              <h5
                className="text-xs font-bold uppercase tracking-[0.2em] mb-3"
                style={{ color: '#9ca3af' }}
              >
                Resources
              </h5>
              <ul className="space-y-2 text-sm" style={{ color: '#6b7280' }}>
                <li>
                  <a href="#" className="hover:text-gray-900 transition-colors">
                    Documentation
                  </a>
                </li>
                <li>
                  <a href="#" className="hover:text-gray-900 transition-colors">
                    Getting Started
                  </a>
                </li>
                <li>
                  <a href="#" className="hover:text-gray-900 transition-colors">
                    CLI Reference
                  </a>
                </li>
              </ul>
            </div>
            <div>
              <h5
                className="text-xs font-bold uppercase tracking-[0.2em] mb-3"
                style={{ color: '#9ca3af' }}
              >
                Community
              </h5>
              <ul className="space-y-2 text-sm" style={{ color: '#6b7280' }}>
                <li>
                  <a
                    href="https://github.com/evm-tooling/poly-bench"
                    target="_blank"
                    rel="noreferrer"
                    className="hover:text-gray-900 transition-colors"
                  >
                    GitHub
                  </a>
                </li>
                <li>
                  <a href="#" className="hover:text-gray-900 transition-colors">
                    Discussions
                  </a>
                </li>
                <li>
                  <a href="#" className="hover:text-gray-900 transition-colors">
                    Twitter
                  </a>
                </li>
              </ul>
            </div>
          </div>
        </div>
        <div
          className="mt-12 pt-8 text-center"
          style={{ borderTop: '1px solid #e5e7eb' }}
        >
          <p className="text-xs" style={{ color: '#9ca3af' }}>
            Released under the MIT License.
          </p>
        </div>
      </div>
    </footer>
  )
}
