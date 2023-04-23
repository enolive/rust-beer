import type {Beer} from './types'
import {guardNotNull} from './types'

export const getApi = (url: string) => {
  const apiUrl = url
  return {
    async readAllBeers(): Promise<Beer[]> {
      const response = await fetch(apiUrl)
      return await response.json() as Beer[]
    },
    async createBeer(beer: Beer): Promise<Beer> {
      const response = await fetch(apiUrl,
        {
          method: 'POST',
          body: JSON.stringify(beer),
          headers: {'Content-Type': 'application/json'},
        })
      return await response.json() as Beer
    },
    async updateBeer(beer: Beer): Promise<Beer> {
      const id = guardNotNull(beer._id).$oid
      const response = await fetch(`${apiUrl}/${id}`,
        {
          method: 'PUT',
          body: JSON.stringify(beer),
          headers: {'Content-Type': 'application/json'},
        })
      return await response.json() as Beer
    },
    async readBeer(id: string): Promise<Beer | null> {
      const response = await fetch(`${apiUrl}/${id}`)
      if (response.status === 204) {
        return null
      }
      return await response.json() as Beer
    },
    async deleteBeer(id: string): Promise<void> {
      await fetch(`${apiUrl}/${id}`, {method: 'DELETE'})
    }
  }
}



