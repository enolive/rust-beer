import {readAllBeers} from '../lib/api'
import type {PageLoad} from './$types'

// noinspection JSUnusedGlobalSymbols
export const load: PageLoad = async () => {
  const beers = await readAllBeers()
  return {beers}
}
