export interface Beer {
  _id?: { $oid: string }
  brand: string
  name: string
  strength: number
}

export const guardNotNull = <T>(it: T | null | undefined): T => {
  if (it == null) {
    throw new Error('param is null or undefined')
  }
  return it
}

