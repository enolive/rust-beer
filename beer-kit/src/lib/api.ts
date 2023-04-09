import type {Beer} from './types'
import {guardNotNull} from './types'

const API_URL = 'http://localhost:8080/beers'

export async function readAllBeers(): Promise<Beer[]> {
  const response = await fetch(API_URL)
  return await response.json() as Beer[]
}

export async function createBeer(beer: Beer): Promise<Beer> {
  const response = await fetch(API_URL,
    {
      method: 'POST',
      body: JSON.stringify(beer),
      headers: {'Content-Type': 'application/json'},
    })
  return await response.json() as Beer
}

export async function updateBeer(beer: Beer): Promise<Beer> {
  const id = guardNotNull(beer._id).$oid
  const response = await fetch(`${API_URL}/${id}`,
    {
      method: 'PUT',
      body: JSON.stringify(beer),
      headers: {'Content-Type': 'application/json'},
    })
  return await response.json() as Beer
}

export async function readBeer(id: string): Promise<Beer | null> {
  const response = await fetch(`${API_URL}/${id}`)
  if (response.status === 204) {
    return null
  }
  return await response.json() as Beer
}

export async function deleteBeer(id: string): Promise<void> {
  await fetch(`${API_URL}/${id}`, {method: 'DELETE'})
}
