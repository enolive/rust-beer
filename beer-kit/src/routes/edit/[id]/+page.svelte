<script lang="ts">
  import type {Beer} from '../../../lib/types'
  import EditBeer from '../../../lib/EditBeer.svelte'
  import {goto} from '$app/navigation'
  import type {PageData} from './$types'
  import {getApi} from '../../../lib/api'
  import {env} from '$env/dynamic/public'

  export let data: PageData;

  let beer: Beer
  $: beer = data.beer

  const api = getApi(env.PUBLIC_API_URL)

  async function handleEditBeer(e: CustomEvent) {
    const beer: Beer = e.detail
    await api.updateBeer(beer)
    await goto('/')
  }
</script>

<h2>
  Edit {beer.name}
</h2>

<EditBeer {beer} on:saved={handleEditBeer}/>
