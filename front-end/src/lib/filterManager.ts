export default class FilterManager {
  static filters: {[key: string]: string} = {}
  
  static update_filter(name: string, values: string[]) {
    FilterManager.filters[name] = values.join(",")
    window.dispatchEvent(new CustomEvent("UpdatedFilters"))
  }
}