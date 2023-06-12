<script lang="ts">
  import type {Beer} from './types'
  import {createEventDispatcher} from 'svelte'
  import type {ModalSettings} from '@skeletonlabs/skeleton'
  import {modalStore} from '@skeletonlabs/skeleton'

  const dispatch = createEventDispatcher()

  function handleDelete(beer: Beer) {
    const settings: ModalSettings = {
      type: 'confirm',
      title: 'Delete',
      body: `The beer ${beer.name} will be deleted. Are you sure?`,
      response: (r: boolean) => {
        if (r) {
          dispatch('deleted', beer)
        }
      },
    }
    modalStore.trigger(settings)
  }

  export let beers: Beer[] = []
</script>

<div class="table-container">
  <table class="table table-hover">
    <thead>
    <tr>
      <th>Brand</th>
      <th>Name</th>
      <th>Strength</th>
      <th>Actions</th>
    </tr>
    </thead>
    <tbody>
    {#each beers as beer}
      <tr>
        <td>{beer.brand}</td>
        <td>{beer.name}</td>
        <td>{beer.strength}</td>
        <td>
          <a class="btn variant-filled" href="/edit/{beer?.id}">
            Edit
          </a>
          <button class="btn variant-filled" on:click={_ => handleDelete(beer)}>
            Delete
          </button>
        </td>
      </tr>
    {/each}
    </tbody>
  </table>
</div>
