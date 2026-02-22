'use client'

import { motion } from 'framer-motion'
import {
  ArrowRight,
  BarChart3,
  BookOpen,
  Braces,
  Bug,
  Check,
  ChevronRight,
  Code2,
  Cpu,
  FileCode,
  Gauge,
  Heart,
  Layers,
  LineChart,
  MessageSquare,
  Play,
  Settings2,
  Star,
  Terminal,
  X,
} from 'lucide-react'
import { Highlight } from 'prism-react-renderer'
import { useState } from 'react'

import CopyButton from '@/components/CopyButton'
import Footer from '@/components/Footer'
import Header from '@/components/Header'
import { useCodeTheme } from '@/lib/use-code-theme'
import '../lib/prism-bench'

const fadeUp = {
  hidden: { opacity: 0, y: 20 },
  visible: (i: number) => ({
    opacity: 1,
    y: 0,
    transition: { delay: i * 0.06, duration: 0.45 },
  }),
}

const fadeIn = {
  hidden: { opacity: 0 },
  visible: { opacity: 1, transition: { duration: 0.6 } },
}

const benchFileCode = `use std::charting

suite keccakBench {
    description: "Keccak256 benchmark - Rust should win here"
    warmup: 100ms
    baseline: "go"
    mode: "auto"
    targetTime: 3000ms

    after {
        charting.drawBarChart(
            title: "Keccak256 Performance",
            xlabel: "Time (ns)"
        )
    }

    setup go {
        import "golang.org/x/crypto/sha3"

        helpers {
            func keccak256Go(data []byte) []byte {
                h := sha3.NewLegacyKeccak256()
                h.Write(data)
                return h.Sum(nil)
            }
        }
    }

    setup ts {
        import {
            import { keccak256 } from 'viem';
        }

        helpers {
            function keccak256Ts(data: Uint8Array): Uint8Array {
                return keccak256(data, 'bytes')
            }
        }
    }

    setup rust {
        import {
            use tiny_keccak::{Hasher, Keccak};
        }

        helpers {
            fn keccak256_rust(data: &[u8]) -> [u8; 32] {
                let mut hasher = Keccak::v256();
                let mut output = [0u8; 32];
                hasher.update(data);
                hasher.finalize(&mut output);
                output
            }
        }
    }

    fixture data {
        hex: "68656c6c6f20776f726c64"
    }

    bench keccak256Bench {
        go: keccak256Go(data)
        ts: keccak256Ts(data)
        rust: keccak256_rust(&data)
    }
}`

const outputTable = `Summary: keccak256Bench
─────────────────────────────────────────────
│ Lang │ Mean (ns/op) │ Std Dev │ Ops/s     │
├──────┼──────────────┼─────────┼───────────┤
│ rust │ 993          │ ± 1.8%  │ 1,006,752 │
│ go   │ 1,216        │ ± 2.1%  │ 822,714   │
│ ts   │ 15,667       │ ± 3.1%  │ 63,839    │

Comparison (baseline: go):
  rust: 1.22x faster
  ts:   12.88x slower`

const problems = [
  {
    icon: X,
    text: 'No unified cross-language benchmarking — separate harnesses, different APIs, different methodologies',
  },
  {
    icon: X,
    text: 'Fair comparisons are hard — different iteration counts, warmup strategies, and data inputs',
  },
  {
    icon: X,
    text: "Results are scattered — each language's output looks different, side-by-side comparisons are tedious",
  },
]

const solutions = [
  {
    icon: Check,
    text: 'Define once, run everywhere — a custom DSL (.bench files) compiles to native code in each language',
  },
  {
    icon: Check,
    text: 'Unified measurement — same iterations, same data, same warmup for apples-to-apples comparisons',
  },
  {
    icon: Check,
    text: 'Rich output — console tables, markdown, JSON for CI, and SVG charts from a single run',
  },
]

const pipeline = [
  {
    step: '01',
    title: 'Parse',
    desc: 'Your .bench file → Lexer → AST → Semantic validation',
    icon: FileCode,
    color: 'from-red-500 to-red-400',
  },
  {
    step: '02',
    title: 'Lower to IR',
    desc: 'Resolve fixtures, merge config defaults, normalize imports',
    icon: Braces,
    color: 'from-orange-500 to-amber-400',
  },
  {
    step: '03',
    title: 'Generate',
    desc: 'Emit native Go, TypeScript, and Rust benchmark code',
    icon: Code2,
    color: 'from-violet-500 to-indigo-400',
  },
  {
    step: '04',
    title: 'Execute',
    desc: 'Isolated subprocesses — no FFI, full native speed per language',
    icon: Play,
    color: 'from-blue-500 to-cyan-400',
  },
  {
    step: '05',
    title: 'Report',
    desc: 'Console tables, markdown docs, JSON, SVG charts',
    icon: LineChart,
    color: 'from-emerald-500 to-green-400',
  },
]

const features = [
  {
    icon: FileCode,
    title: 'Custom DSL',
    desc: 'Clean, declarative syntax with language-specific setup blocks',
  },
  {
    icon: Gauge,
    title: 'Auto-calibration',
    desc: 'Automatically adjusts iteration counts to reach target execution time',
  },
  {
    icon: Cpu,
    title: 'Memory Profiling',
    desc: 'Track allocations across Go, TypeScript, and Rust',
  },
  {
    icon: Layers,
    title: 'Portable Fixtures',
    desc: 'Share hex-encoded test data across all language implementations',
  },
  {
    icon: BarChart3,
    title: 'Chart Generation',
    desc: 'SVG bar charts, pie charts, and line charts from results',
  },
  {
    icon: Settings2,
    title: 'LSP Support',
    desc: 'Full editor integration with diagnostics, completions, and formatting',
  },
]

const crates = [
  { name: 'poly-bench-dsl', desc: 'Lexer, parser, AST, formatter, validator' },
  {
    name: 'poly-bench-ir',
    desc: 'Intermediate representation — normalized benchmark structures',
  },
  {
    name: 'poly-bench-runtime',
    desc: 'Language-specific code generation and execution',
  },
  {
    name: 'poly-bench-executor',
    desc: 'Orchestrates runs, calibration, measurements',
  },
  { name: 'poly-bench-reporter', desc: 'Console, markdown, JSON, SVG output' },
  {
    name: 'poly-bench-lsp',
    desc: 'Language Server Protocol for editor support',
  },
  {
    name: 'poly-bench-stdlib',
    desc: 'Standard library — anvil, charting, constants',
  },
  {
    name: 'poly-bench-project',
    desc: 'Project init, dependency management, manifests',
  },
]

const runtimeDetails = [
  {
    lang: 'Go',
    runtime: 'Plugin-based execution or subprocess',
    memory: 'runtime.ReadMemStats',
    accent: '#00ADD8',
  },
  {
    lang: 'TypeScript',
    runtime: 'Node.js subprocess',
    memory: 'process.memoryUsage()',
    accent: '#3178C6',
  },
  {
    lang: 'Rust',
    runtime: 'Cargo build + subprocess',
    memory: 'Custom allocator tracking',
    accent: '#DEA584',
  },
]

const installScriptUrl = 'https://install.evm-tooling.tools'
const installCommand = `curl -L ${installScriptUrl} | bash`

export default function Home() {
  const [activeTab, setActiveTab] = useState<'problem' | 'solution'>('problem')
  const codeTheme = useCodeTheme()

  return (
    <>
      <Header />
      <main className="relative overflow-x-clip">
        {/* ── HERO — asymmetric split with .bench file ── */}
        <section className="relative w-full overflow-hidden !border-b !border-border bg-background">
          <div className="w-full max-w-7xl mx-auto px-6 py-28 lg:py-36">
            <div className="grid lg:grid-cols-[1fr_1.1fr] gap-10 lg:gap-16 items-start">
              <motion.div
                initial="hidden"
                animate="visible"
                variants={{
                  visible: { transition: { staggerChildren: 0.08 } },
                }}
              >
                <motion.div
                  variants={fadeUp}
                  custom={0}
                  className="inline-flex items-center gap-2 text-xs font-medium px-3 py-1 mt-2 mb-6 rounded-full !border !border-border text-foreground-muted"
                >
                  <span className="w-1.5 h-1.5 rounded-full bg-emerald-400 animate-pulse" />
                  Built in Rust · Open Source · MIT
                </motion.div>
                <motion.h1
                  variants={fadeUp}
                  custom={1}
                  className="text-5xl md:text-6xl lg:text-7xl font-black tracking-tight leading-[0.92] text-foreground"
                >
                  Benchmark{' '}
                  <span className="text-transparent bg-clip-text bg-gradient-to-br from-primary to-tertiary">
                    every
                  </span>
                  <br />
                  runtime.
                </motion.h1>
                <motion.p
                  variants={fadeUp}
                  custom={2}
                  className="mt-6 text-lg max-w-md leading-relaxed text-foreground-secondary"
                >
                  Write benchmarks once in a simple DSL. Run them across{' '}
                  <strong className="text-foreground">Go</strong>,{' '}
                  <strong className="text-foreground">TypeScript</strong>, and{' '}
                  <strong className="text-foreground">Rust</strong>. Get
                  unified, statistically sound results.
                </motion.p>
                <motion.div
                  variants={fadeUp}
                  custom={3}
                  className="mt-8 flex flex-wrap gap-3"
                >
                  <a
                    href="#get-started"
                    className="inline-flex items-center gap-2 px-6 py-3 rounded-xl font-medium text-primary-foreground bg-primary hover:bg-primary-hover transition-all hover:scale-[1.02]"
                  >
                    Get Started <ArrowRight className="w-4 h-4" />
                  </a>
                  <a
                    href="#how-it-works"
                    className="inline-flex items-center gap-2 px-6 py-3 rounded-xl font-medium text-foreground-secondary !border !border-border bg-secondary/50 hover:bg-secondary transition-colors"
                  >
                    How it works
                  </a>
                </motion.div>

                {/* Runtime badges */}
                <motion.div
                  variants={fadeUp}
                  custom={4}
                  className="mt-8 flex gap-3"
                >
                  {runtimeDetails.map((r) => (
                    <span
                      key={r.lang}
                      className="inline-flex items-center gap-1.5 text-xs rounded-lg px-4 py-2 bg-secondary !border !border-border text-foreground-secondary"
                    >
                      <span
                        className="w-2 h-2 rounded-full"
                        style={{ backgroundColor: r.accent }}
                      />
                      {r.lang}
                    </span>
                  ))}
                </motion.div>
              </motion.div>

              <motion.div
                initial={{ opacity: 0, y: 30 }}
                animate={{ opacity: 1, y: 0 }}
                transition={{ delay: 0.4, duration: 0.6 }}
                className="flex flex-col gap-4 w-full min-w-0 lg:max-w-2xl"
              >
                {/* .bench file */}
                <div className="rounded-2xl overflow-hidden !border !border-border bg-code-bg shadow-xl dark:shadow-none">
                  <div className="flex items-center gap-2 px-4 py-3 bg-background-elevated !border-b !border-border">
                    <div className="flex gap-1.5">
                      <span className="w-3 h-3 rounded-full bg-red-400" />
                      <span className="w-3 h-3 rounded-full bg-amber-400" />
                      <span className="w-3 h-3 rounded-full bg-emerald-400" />
                    </div>
                    <span className="text-xs ml-2 font-mono text-foreground-muted">
                      keccak.bench
                    </span>
                  </div>
                  <Highlight
                    theme={codeTheme}
                    code={benchFileCode.trim()}
                    language="bench"
                  >
                    {({ tokens, getLineProps, getTokenProps }) => (
                      <pre className="p-4 sm:p-5 text-[12px] sm:text-[13px] leading-relaxed font-mono overflow-x-auto max-h-[260px] sm:max-h-[380px] overflow-y-auto">
                        <code>
                          {tokens.map((line, i) => (
                            <div key={i} {...getLineProps({ line })}>
                              {line.map((token, key) => (
                                <span key={key} {...getTokenProps({ token })} />
                              ))}
                            </div>
                          ))}
                        </code>
                      </pre>
                    )}
                  </Highlight>
                </div>
                {/* Install terminal */}
                <div className="rounded-2xl overflow-hidden !border !border-border bg-code-bg shadow-xl dark:shadow-none">
                  <div className="flex items-center justify-between gap-2 px-4 py-1 bg-background-elevated !border-b !border-border">
                    <div className="flex items-center gap-2">
                      <span className="w-3 h-3 rounded-full bg-red-400" />
                      <span className="w-3 h-3 rounded-full bg-amber-400" />
                      <span className="w-3 h-3 rounded-full bg-emerald-400" />
                      <span className="text-xs ml-2 font-mono text-foreground-muted">
                        Terminal
                      </span>
                    </div>
                    <CopyButton text={installCommand} />
                  </div>
                  <pre className="p-4 sm:p-5 text-[12px] sm:text-[13px] leading-relaxed font-mono overflow-x-auto w-full min-w-0">
                    <code>
                      <span className="text-foreground-muted">$</span>{' '}
                      <span className="text-primary">curl</span>{' '}
                      <span className="text-tertiary">-L</span>{' '}
                      <span className="text-amber-400 break-all">
                        {installScriptUrl}
                      </span>{' '}
                      <span className="text-foreground-muted">|</span>{' '}
                      <span className="text-success">bash</span>
                    </code>
                  </pre>
                </div>
              </motion.div>
            </div>
          </div>
        </section>

        {/* ── THE PROBLEM / SOLUTION — alt variant: dark theme in light & dark ── */}
        <section
          id="how-it-works"
          className="py-28 bg-background-tertiary-alt text-foreground-alt !border-t !border-border-alt"
        >
          <div className="max-w-5xl mx-auto px-6">
            <motion.div
              initial="hidden"
              whileInView="visible"
              viewport={{ once: true }}
              variants={fadeIn}
              className="text-center mb-12"
            >
              <h2 className="text-3xl md:text-5xl font-black tracking-tight text-foreground-alt">
                Why Poly Bench?
              </h2>
              <p className="mt-4 max-w-2xl mx-auto text-foreground-muted-alt">
                A multi-language benchmarking framework for fair cross-language
                comparisons.
              </p>
            </motion.div>

            <div className="flex justify-center gap-2 mb-10">
              <button
                onClick={() => setActiveTab('problem')}
                className={`text-sm font-medium px-5 py-2.5 rounded-lg transition-all ${
                  activeTab === 'problem'
                    ? 'bg-destructive-alt/20 text-destructive-alt !border !border-destructive-alt/30'
                    : 'text-foreground-muted-alt !border !border-transparent hover:text-foreground-secondary-alt'
                }`}
              >
                The Problem
              </button>
              <button
                onClick={() => setActiveTab('solution')}
                className={`text-sm font-medium px-5 py-2.5 rounded-lg transition-all ${
                  activeTab === 'solution'
                    ? 'bg-success-alt/20 text-success-alt !border !border-success-alt/30'
                    : 'text-foreground-muted-alt !border !border-transparent hover:text-foreground-secondary-alt'
                }`}
              >
                The Solution
              </button>
            </div>

            <div className="grid md:grid-cols-3 gap-5">
              {(activeTab === 'problem' ? problems : solutions).map(
                (item, i) => (
                  <motion.div
                    key={item.text}
                    initial="hidden"
                    whileInView="visible"
                    viewport={{ once: true }}
                    custom={i}
                    variants={fadeUp}
                    className={`rounded-xl p-6 !border !border-border-alt ${
                      activeTab === 'problem'
                        ? '!border-destructive-alt/20 bg-destructive-alt/10'
                        : '!border-success-alt/20 bg-success-alt/10'
                    }`}
                  >
                    <item.icon
                      className={`w-5 h-5 mb-3 ${activeTab === 'problem' ? 'text-destructive-alt' : 'text-success-alt'}`}
                    />
                    <p className="text-sm leading-relaxed text-foreground-alt">
                      {item.text}
                    </p>
                  </motion.div>
                ),
              )}
            </div>
          </div>
        </section>

        {/* ── OUTPUT PREVIEW — alt variant: dark theme in light & dark ── */}
        <section className="py-28 bg-background-tertiary-alt !border-t !border-border-alt">
          <div className="max-w-5xl mx-auto px-6">
            <div className="grid lg:grid-cols-2 gap-12 items-center">
              <div>
                <span className="text-xs font-bold uppercase tracking-[0.2em] mb-4 block text-tertiary-alt">
                  Unified Output
                </span>
                <h3 className="text-3xl md:text-4xl font-black tracking-tight text-foreground-alt">
                  One run.
                  <br />
                  Every language.
                  <br />
                  <span className="text-foreground-muted-alt">Compared.</span>
                </h3>
                <p className="mt-4 leading-relaxed text-foreground-secondary-alt">
                  Run{' '}
                  <code className="px-1.5 py-0.5 rounded text-sm bg-tertiary-alt/10 text-tertiary-alt">
                    poly-bench run
                  </code>{' '}
                  and get a statistical comparison across all three languages.
                  Includes mean, standard deviation, ops/sec, and speedup
                  ratios.
                </p>
                <div className="mt-6 flex flex-wrap gap-3">
                  {['Console', 'Markdown', 'JSON', 'SVG Charts'].map((fmt) => (
                    <span
                      key={fmt}
                      className="text-xs rounded-full px-3 py-1 !border !border-border-alt text-foreground-muted-alt"
                    >
                      {fmt}
                    </span>
                  ))}
                </div>
              </div>
              <div className="rounded-2xl overflow-hidden !border !border-border-alt bg-code-bg-deep-alt">
                <div className="flex items-center gap-2 px-4 py-3 !border-b !border-border-alt">
                  <Terminal className="w-4 h-4 text-foreground-muted-alt" />
                  <span className="text-xs font-mono text-foreground-muted-alt">
                    poly-bench run keccak.bench
                  </span>
                </div>
                <pre className="p-5 text-[13px] leading-relaxed font-mono overflow-x-auto text-terminal-output-alt">
                  <code>{outputTable}</code>
                </pre>
              </div>
            </div>
          </div>
        </section>

        {/* ── PIPELINE — horizontal stepped flow ── */}
        <section className="py-28 bg-background-elevated">
          <div className="max-w-7xl mx-auto px-6">
            <div className="text-center mb-16">
              <span className="text-xs font-bold uppercase tracking-[0.2em] mb-4 block text-primary">
                Architecture
              </span>
              <h2 className="text-3xl md:text-5xl font-black tracking-tight text-foreground">
                How benchmarks flow
              </h2>
              <p className="mt-4 max-w-lg mx-auto text-foreground-secondary">
                From .bench file to results — every benchmark follows the same
                five-stage pipeline.
              </p>
            </div>
            <div className="grid grid-cols-1 md:grid-cols-5 gap-4">
              {pipeline.map((p, i) => (
                <motion.div
                  key={p.step}
                  initial="hidden"
                  whileInView="visible"
                  viewport={{ once: true }}
                  custom={i}
                  variants={fadeUp}
                  className="relative group"
                >
                  <div className="rounded-2xl p-6 h-full transition-all hover:-translate-y-1 !border !border-border bg-card hover:shadow-lg">
                    <div
                      className={`w-10 h-10 rounded-xl bg-gradient-to-br ${p.color} text-white flex items-center justify-center mb-4`}
                    >
                      <p.icon className="w-5 h-5" />
                    </div>
                    <span className="text-[10px] font-bold uppercase tracking-widest text-foreground-muted">
                      {p.step}
                    </span>
                    <h4 className="font-bold mt-1 text-foreground">
                      {p.title}
                    </h4>
                    <p className="text-xs mt-2 leading-relaxed text-foreground-secondary">
                      {p.desc}
                    </p>
                  </div>
                  {i < pipeline.length - 1 && (
                    <ChevronRight className="absolute -right-3 top-1/2 -translate-y-1/2 w-5 h-5 hidden md:block text-foreground-muted" />
                  )}
                </motion.div>
              ))}
            </div>
          </div>
        </section>

        {/* ── RUNTIME ISOLATION — horizontal cards ── */}
        <section className="py-20 !border-t !border-border bg-background">
          <div className="max-w-5xl mx-auto px-6">
            <div className="text-center mb-12">
              <span className="text-xs font-bold uppercase tracking-[0.2em] mb-4 block text-tertiary">
                Isolated Runtimes
              </span>
              <h2 className="text-2xl md:text-3xl font-black tracking-tight text-foreground">
                Each language runs at full native speed
              </h2>
              <p className="mt-3 text-foreground-secondary">
                No interpretation, no FFI — isolated subprocesses with
                per-language memory tracking.
              </p>
            </div>
            <div className="grid md:grid-cols-3 gap-5">
              {runtimeDetails.map((r, i) => (
                <motion.div
                  key={r.lang}
                  initial="hidden"
                  whileInView="visible"
                  viewport={{ once: true }}
                  custom={i}
                  variants={fadeUp}
                  className="rounded-xl p-6 relative overflow-hidden !border !border-border bg-card"
                >
                  <div
                    className="absolute top-0 left-0 w-full h-1"
                    style={{ backgroundColor: r.accent }}
                  />
                  <h4 className="font-bold text-lg" style={{ color: r.accent }}>
                    {r.lang}
                  </h4>
                  <dl className="mt-4 space-y-3 text-sm">
                    <div>
                      <dt className="text-xs uppercase tracking-widest text-foreground-muted">
                        Runtime
                      </dt>
                      <dd className="mt-0.5 text-foreground-secondary">
                        {r.runtime}
                      </dd>
                    </div>
                    <div>
                      <dt className="text-xs uppercase tracking-widest text-foreground-muted">
                        Memory
                      </dt>
                      <dd className="mt-0.5 font-mono text-xs text-foreground-secondary">
                        {r.memory}
                      </dd>
                    </div>
                  </dl>
                </motion.div>
              ))}
            </div>
          </div>
        </section>

        {/* ── FEATURES — staggered 3×2 grid with sticky left ── */}
        <section id="features" className="py-28 bg-background-secondary">
          <div className="max-w-6xl mx-auto px-6">
            <div className="grid lg:grid-cols-[1fr_2fr] gap-16 items-start">
              <div className="lg:sticky lg:top-28">
                <span className="text-xs font-bold uppercase tracking-[0.2em] mb-4 block text-primary">
                  Key Features
                </span>
                <h2 className="text-3xl md:text-4xl font-black tracking-tight leading-tight text-foreground">
                  Everything you need for rigorous benchmarking
                </h2>
                <p className="mt-4 leading-relaxed text-foreground-secondary">
                  poly-bench provides a complete toolkit — from DSL parsing to
                  chart generation — built in Rust for speed and reliability.
                </p>
              </div>
              <div className="grid sm:grid-cols-2 gap-4">
                {features.map((f, i) => (
                  <motion.div
                    key={f.title}
                    initial="hidden"
                    whileInView="visible"
                    viewport={{ once: true }}
                    custom={i}
                    variants={fadeUp}
                    className="rounded-xl p-6 transition-all hover:shadow-lg !border !border-border bg-card"
                  >
                    <f.icon className="w-5 h-5 mb-3 text-foreground-muted" />
                    <h3 className="font-semibold text-foreground">{f.title}</h3>
                    <p className="text-sm mt-1 leading-relaxed text-foreground-secondary">
                      {f.desc}
                    </p>
                  </motion.div>
                ))}
              </div>
            </div>
          </div>
        </section>

        {/* ── ARCHITECTURE — crate list ── */}
        <section id="architecture" className="py-28 bg-background-elevated">
          <div className="max-w-5xl mx-auto px-6">
            <div className="text-center mb-12">
              <span className="text-xs font-bold uppercase tracking-[0.2em] mb-4 block text-tertiary">
                Modular Workspace
              </span>
              <h2 className="text-3xl md:text-4xl font-black tracking-tight text-foreground">
                Rust workspace, 8 crates
              </h2>
              <p className="mt-3 text-foreground-secondary">
                Each crate handles a specific responsibility. Use them
                individually or together.
              </p>
            </div>
            <div className="grid sm:grid-cols-2 gap-3">
              {crates.map((c, i) => (
                <motion.div
                  key={c.name}
                  initial="hidden"
                  whileInView="visible"
                  viewport={{ once: true }}
                  custom={i}
                  variants={fadeUp}
                  className="flex items-start gap-4 rounded-xl px-5 py-4 transition-colors !border !border-border bg-card"
                >
                  <code className="text-xs font-mono px-2 py-1 rounded shrink-0 mt-0.5 bg-destructive/10 text-destructive">
                    {c.name}
                  </code>
                  <p className="text-sm text-foreground-secondary">{c.desc}</p>
                </motion.div>
              ))}
            </div>
          </div>
        </section>

        {/* ── STATISTICS ── */}
        <section className="py-28 bg-background-tertiary-alt !border-t !border-border-alt">
          <div className="max-w-5xl mx-auto px-6">
            <div className="grid lg:grid-cols-2 gap-16 items-center">
              <div>
                <span className="text-xs font-bold uppercase tracking-[0.2em] mb-4 block text-tertiary">
                  Statistics Done Right
                </span>
                <h3 className="text-3xl md:text-4xl font-black tracking-tight text-foreground-alt">
                  No shortcuts on
                  <br />
                  measurement.
                </h3>
                <p className="mt-4 leading-relaxed text-foreground-muted-alt">
                  poly-bench includes proper statistical methodology so you can
                  trust the numbers.
                </p>
              </div>
              <div className="grid grid-cols-2 gap-4">
                {[
                  {
                    title: 'Warmup Removal',
                    desc: 'Discard initial iterations before timing',
                  },
                  {
                    title: 'Multi-run',
                    desc: 'Run benchmarks N times for reliable stats',
                  },
                  {
                    title: 'Outlier Detection',
                    desc: 'IQR-based outlier removal',
                  },
                  {
                    title: 'Auto-calibration',
                    desc: 'Hit target execution time automatically',
                  },
                ].map((s, i) => (
                  <motion.div
                    key={s.title}
                    initial="hidden"
                    whileInView="visible"
                    viewport={{ once: true }}
                    custom={i}
                    variants={fadeUp}
                    className="rounded-xl p-5 !border !border-border/40 bg-bg-card-alt"
                  >
                    <h4 className="font-semibold text-sm text-foreground">
                      {s.title}
                    </h4>
                    <p className="text-xs mt-1 leading-relaxed text-foreground-muted">
                      {s.desc}
                    </p>
                  </motion.div>
                ))}
              </div>
            </div>
          </div>
        </section>

        {/* ── GET STARTED — install ── */}
        <section id="get-started" className="py-28 bg-background-elevated">
          <div className="max-w-3xl mx-auto px-6 text-center">
            <h2 className="text-3xl md:text-5xl font-black tracking-tight text-foreground">
              Ready to{' '}
              <span className="text-transparent bg-clip-text bg-gradient-to-br from-primary to-tertiary">
                benchmark
              </span>
              ?
            </h2>
            <p className="mt-4 max-w-lg mx-auto text-foreground-secondary">
              Install poly-bench and write your first .bench file in under a
              minute.
            </p>
            <div className="mt-8 inline-block rounded-xl px-6 py-4 font-mono text-sm !border !border-border bg-background-secondary">
              <span className="text-foreground-muted">$</span>{' '}
              <span className="text-primary">curl</span>{' '}
              <span className="text-tertiary">-sSL</span>{' '}
              <span className="text-amber-400">{installScriptUrl}</span>{' '}
              <span className="text-foreground-muted">|</span>{' '}
              <span className="text-success">bash</span>
            </div>
            <div className="mt-8 flex justify-center gap-4">
              <a
                href="https://github.com/evm-tooling/poly-bench"
                target="_blank"
                rel="noreferrer"
                className="inline-flex items-center gap-2 px-6 py-3 rounded-xl font-medium text-primary-foreground bg-primary hover:bg-primary-hover transition-all hover:scale-[1.02]"
              >
                View on GitHub <ArrowRight className="w-4 h-4" />
              </a>
              <a
                href="/docs/getting-started"
                className="inline-flex items-center gap-2 px-6 py-3 rounded-xl font-medium text-foreground-secondary !border !border-border bg-secondary/50 hover:bg-secondary transition-colors"
              >
                Read the Docs
              </a>
            </div>
          </div>
        </section>

        {/* ── COMMUNITY ── */}
        <section className="py-20 !border-t !border-border bg-background">
          <div className="max-w-5xl mx-auto px-6">
            <div className="grid lg:grid-cols-[1fr_1fr] gap-12">
              <div>
                <span className="text-xs font-bold uppercase tracking-[0.2em] mb-4 block text-primary">
                  Community
                </span>
                <h3 className="text-2xl md:text-3xl font-black tracking-tight text-foreground">
                  Get involved
                </h3>
                <p className="mt-3 text-foreground-secondary">
                  poly-bench is open source and actively developed.
                  Contributions welcome.
                </p>
                <div className="mt-6 flex flex-wrap gap-3">
                  <a
                    href="https://github.com/sponsors/evm-tooling"
                    target="_blank"
                    rel="noreferrer"
                    className="inline-flex items-center gap-2 px-5 py-2.5 rounded-lg text-tertiary-foreground bg-tertiary hover:opacity-90 text-sm font-medium transition-colors"
                  >
                    <Heart className="w-4 h-4" /> Sponsor
                  </a>
                  <a
                    href="https://github.com/evm-tooling/poly-bench"
                    target="_blank"
                    rel="noreferrer"
                    className="inline-flex items-center gap-2 px-5 py-2.5 rounded-lg text-sm font-medium text-foreground-secondary !border !border-border bg-secondary/50 hover:bg-secondary transition-colors"
                  >
                    <Star className="w-4 h-4" /> Star on GitHub
                  </a>
                </div>
              </div>
              <div className="flex flex-col gap-3">
                {[
                  {
                    icon: MessageSquare,
                    title: 'Discussions',
                    desc: 'Ask questions and share ideas',
                    href: 'https://github.com/evm-tooling/poly-bench/discussions',
                  },
                  {
                    icon: Bug,
                    title: 'Issues',
                    desc: 'Report bugs or request features',
                    href: 'https://github.com/evm-tooling/poly-bench/issues',
                  },
                  {
                    icon: BookOpen,
                    title: 'Docs',
                    desc: 'Full documentation and guides',
                    href: '/docs/getting-started',
                  },
                ].map((item) => (
                  <a
                    key={item.title}
                    href={item.href}
                    target={item.href.startsWith('http') ? '_blank' : undefined}
                    rel={
                      item.href.startsWith('http') ? 'noreferrer' : undefined
                    }
                    className="flex items-center gap-4 rounded-xl px-5 py-4 transition-all group hover:shadow-md !border !border-border bg-card"
                  >
                    <item.icon className="w-5 h-5 shrink-0 text-foreground-muted" />
                    <div className="flex-1 min-w-0">
                      <h4 className="font-semibold text-sm text-foreground">
                        {item.title}
                      </h4>
                      <p className="text-xs text-foreground-secondary">
                        {item.desc}
                      </p>
                    </div>
                    <ArrowRight className="w-4 h-4 shrink-0 group-hover:translate-x-1 transition-all text-foreground-muted" />
                  </a>
                ))}
              </div>
            </div>
          </div>
        </section>
      </main>
      <Footer />
    </>
  )
}
