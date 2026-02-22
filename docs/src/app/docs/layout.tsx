import DocsSidebar from '@/components/DocsSidebar'
import Header from '@/components/Header'
import { SidebarProvider } from '@/components/SidebarContext'

export default function DocsLayout({
  children,
}: {
  children: React.ReactNode
}) {
  return (
    <SidebarProvider>
      <div className="h-dvh overflow-hidden bg-background">
        <Header />
        <div className="flex h-[calc(100dvh-3.5rem)]">
          <DocsSidebar />
          <main className="flex-1 min-w-0 overflow-y-auto overscroll-contain bg-background-tertiary/60 relative z-10 lg:rounded-tl-3xl lg:border-l lg:border-t lg:border-[1px] lg:border-border/40">
            <div className="py-8 px-6 lg:px-12 lg:pr-16">{children}</div>
          </main>
        </div>
      </div>
    </SidebarProvider>
  )
}
