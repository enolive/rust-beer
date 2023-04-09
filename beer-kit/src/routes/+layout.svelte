<script lang='ts'>
  // The ordering of these imports is critical to your app working properly
  import '@skeletonlabs/skeleton/themes/theme-skeleton.css'
  // If you have source.organizeImports set to true in VSCode, then it will auto change this ordering
  import '@skeletonlabs/skeleton/styles/all.css'
  // Most of your app wide CSS should be put in this file
  import '../app.postcss'
  import {AppBar, Modal} from '@skeletonlabs/skeleton'
  import {QueryClient, QueryClientProvider} from '@tanstack/svelte-query'
  import {browser} from '$app/environment'

  const client = new QueryClient({
    defaultOptions: {
      queries: {
        enabled: browser,
      },
    },
  })
</script>

<QueryClientProvider {client}>
  <Modal/>
  <header>
    <AppBar>
      <h1><a href="/">🍺 Beer App 🍺</a></h1>
    </AppBar>
  </header>
  <main class="p-4">
    <slot/>
  </main>
</QueryClientProvider>
