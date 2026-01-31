export default class FilterManager {
  static filters: {[key: string]: string} = {}
  
  static update_filter(name: string, values: string[]) {
    FilterManager.filters[name] = values.join(",")
    
    debouncedFunction();
  }
  
  static debounce(func: Function, delay = 800) {
    let timeoutId: null | number;
  
    //@ts-ignore
    return function (...args) {
      // Clear the previous timeout, if any
      if (timeoutId) {
        clearTimeout(timeoutId);
      }
  
      // Set a new timeout
      timeoutId = setTimeout(() => {
        //@ts-ignore
        func.apply(this, args);
      }, delay);
    };
  }
}

const update_gallery = () => {
  window.dispatchEvent(new CustomEvent("UpdatedFilters"))
};

const debouncedFunction = FilterManager.debounce(update_gallery);
