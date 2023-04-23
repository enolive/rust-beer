import {browser} from '$app/environment'
import type {PageServerLoad} from './$types'
import {getApi} from '../lib/api'
import {env} from '$env/dynamic/private'

// noinspection JSUnusedGlobalSymbols
export const load: PageServerLoad = async () => {
  console.log(browser)
  const beers = await getApi(env.API_URL).readAllBeers()
  return {beers}
}
