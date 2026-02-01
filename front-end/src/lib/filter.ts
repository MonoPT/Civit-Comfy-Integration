export type Filter = {
  name: string,
  value: string
}

export type FilterOption = {
  [key: string]: {
    selected: Filter,
    options: Filter[]
  }
}