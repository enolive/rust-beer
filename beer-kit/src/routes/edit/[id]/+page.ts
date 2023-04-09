import {readBeer} from '../../../lib/api'
import {error} from '@sveltejs/kit'
import type {PageLoad} from './$types'

// noinspection JSUnusedGlobalSymbols
export const load: PageLoad = async ({params}) => {
  const id = params.id
  const beer = await readBeer(id)
  if (beer === null) {
    throw error(404, 'Beer not found')
  }
  return {
    beer,
  }
}
