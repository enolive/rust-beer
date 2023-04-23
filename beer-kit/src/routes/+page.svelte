<script lang="ts">
  import {Accordion, AccordionItem} from '@skeletonlabs/skeleton'
  import EditBeer from '../lib/EditBeer.svelte'
  import type {Beer} from '../lib/types'
  import BeerTable from '../lib/BeerTable.svelte'
  import {guardNotNull} from '../lib/types.js'
  import type {PageData} from './$types'
  import {createQuery, useQueryClient} from '@tanstack/svelte-query'
  import {getApi} from '../lib/api'
  import {env} from '$env/dynamic/public'

  const client = useQueryClient()

  const api = getApi(env.PUBLIC_API_URL)

  export let data: PageData

  const query = createQuery({
    queryKey: ['beers'],
    queryFn: api.readAllBeers,
    initialData: data.beers,
  })

  async function handleCreateBeer(e: CustomEvent) {
    const beer: Beer = e.detail
    await api.createBeer(beer)
    await client.invalidateQueries({queryKey: ['beers']})
  }

  async function handleDeleteBeer(e: CustomEvent) {
    const beer: Beer = e.detail
    await api.deleteBeer(guardNotNull(beer._id).$oid)
    await client.invalidateQueries({queryKey: ['beers']})
  }
</script>

<Accordion>
  <AccordionItem>
    <svelte:fragment slot="summary">
      <h2>Create a beer</h2>
    </svelte:fragment>
    <svelte:fragment slot="content">
      <EditBeer on:saved={handleCreateBeer}/>
    </svelte:fragment>
  </AccordionItem>
  <AccordionItem open>
    <svelte:fragment slot="summary">
      <h2>Here are some beers</h2>
    </svelte:fragment>
    <svelte:fragment slot="content">
      <BeerTable beers={$query.data} on:deleted={handleDeleteBeer}/>
    </svelte:fragment>
  </AccordionItem>
</Accordion>
