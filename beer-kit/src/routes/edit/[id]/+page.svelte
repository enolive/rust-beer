<script lang="ts">
  import type {Beer} from '../../../lib/types'
  import EditBeer from '../../../lib/EditBeer.svelte'
  import {updateBeer} from '../../../lib/api'
  import {goto} from '$app/navigation'
  import type {PageData} from './$types'

  export let data: PageData;

  let beer: Beer
  $: beer = data.beer

  async function handleEditBeer(e: CustomEvent) {
    const beer: Beer = e.detail
    await updateBeer(beer)
    await goto('/')
  }
</script>

<h2>
  Edit {beer.name}
</h2>

<EditBeer {beer} on:saved={handleEditBeer}/>
