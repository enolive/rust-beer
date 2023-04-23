import {error} from '@sveltejs/kit'
import type {PageServerLoad} from './$types'
import {getApi} from '../../../lib/api'
import {env} from '$env/dynamic/private'

// noinspection JSUnusedGlobalSymbols
export const load: PageServerLoad = async ({params}) => {
  const id = params.id
  const beer = await getApi(env.API_URL).readBeer(id)
  if (beer === null) {
    throw error(404, 'Beer not found')
  }
  return {
    beer,
  }
}
