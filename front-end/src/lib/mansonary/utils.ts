export const create_loader = (size: number, tickness: number) => {
  let spinner = document.createElement("span")
  spinner.classList.add("loader")
  spinner.style.setProperty("width", `${size}px`)
  spinner.style.setProperty("height", `${size}px`)
  spinner.style.setProperty("--tickness", `${tickness}px`)
  
  return spinner
}