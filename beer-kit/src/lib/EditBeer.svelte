<script lang="ts">
  import type {Beer} from './types'
  import {createEventDispatcher} from 'svelte'

  export let beer: Beer | undefined

  let brand = beer?.brand ?? ''
  let name = beer?.name ?? ''
  let strength = beer?.strength ?? 5.0

  const dispatch = createEventDispatcher()

  function handleSubmit() {
    const beerToSave: Beer = {
      ...beer,
      brand,
      name,
      strength,
    }
    dispatch('saved', beerToSave)
  }
</script>

<form on:submit|preventDefault={handleSubmit}>
  <div class="mb-4">
    <label class="label">
      <span>Brand</span>
      <input class="input" type="text" bind:value={brand}>
    </label>
    <label class="label">
      <span>Name</span>
      <input class="input" type="text" bind:value={name}>
    </label>
    <label class="label">
      <span>Strength</span>
      <input class="input" type="number" bind:value={strength} step="0.1">
    </label>
  </div>
  <input class="btn variant-filled-primary" type="submit" value="Save">
</form>
