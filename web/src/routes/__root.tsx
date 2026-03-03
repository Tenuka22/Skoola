import {
  HeadContent,
  Scripts,
  createRootRoute,
  useLoaderData,
} from '@tanstack/react-router'
import { TanStackRouterDevtoolsPanel } from '@tanstack/react-router-devtools'
import { TanStackDevtools } from '@tanstack/react-devtools'
import { NuqsAdapter } from 'nuqs/adapters/tanstack-router'
import appCss from '../styles.css?url'
import { queryClient } from '@/lib/query-client' // Import your queryClient instance
import { ThemeProvider } from '@/components/providers/theme-provider'
import { QueryProvider } from '@/components/providers/query-provider'
import { NotFound } from '@/components/root/not-found'
import { TooltipProvider } from '@/components/ui/tooltip'
import { Toaster } from '@/components/ui/sonner'
import { themes } from '@/lib/themes-data'
import { getThemeServer } from '@/lib/theme-server'

export type RootRouteContext = {
  queryClient: typeof queryClient
}

export const Route = createRootRoute({
  context: () => ({
    queryClient, // Make queryClient available in the router context
  }),
  loader: async () => {
    const theme = await getThemeServer()
    return {
      theme,
    }
  },
  head: () => ({
    meta: [
      {
        charSet: 'utf-8',
      },
      {
        name: 'viewport',
        content: 'width=device-width, initial-scale=1',
      },
      {
        title: 'TanStack Start Starter',
      },
    ],
    links: [
      {
        rel: 'stylesheet',
        href: appCss,
      },
    ],
  }),

  notFoundComponent: NotFound,
  shellComponent: RootDocument,
})

function RootDocument({ children }: { children: React.ReactNode }) {
  const { theme } = useLoaderData({ from: Route.id })
  const themeData = themes[theme]

  return (
    <html lang="en" data-theme={theme}>
      <head>
        <script
          crossOrigin="anonymous"
          src="https://tweakcn.com/live-preview.min.js"
        />
        <HeadContent />
        {themeData && (
          <style
            id="skoola-theme-styles"
            dangerouslySetInnerHTML={{
              __html: `
                :root { ${themeData.root} }
                .dark { ${themeData.dark} }
              `,
            }}
          />
        )}
      </head>
      <body>
        <NuqsAdapter>
          <QueryProvider>
            <ThemeProvider initialTheme={theme}>
              <Toaster />
              <TooltipProvider>{children}</TooltipProvider>
            </ThemeProvider>
          </QueryProvider>
          <TanStackDevtools
            config={{
              position: 'bottom-right',
            }}
            plugins={[
              {
                name: 'Tanstack Router',
                render: <TanStackRouterDevtoolsPanel />,
              },
            ]}
          />
        </NuqsAdapter>
        <Scripts />
      </body>
    </html>
  )
}
