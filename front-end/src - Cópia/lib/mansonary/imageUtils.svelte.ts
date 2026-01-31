import {collection_list} from "$lib/state.svelte"
import api from "$lib/api"
import { user_token, userState } from "$lib/state.svelte"
import { create_loader } from "./utils"

export const load_collection_data_by_media_id = async (id: number) => {
  const target = document.querySelector(`.image-wrapper[data-id="${id}"]`)
  if(!target) return
  
  let res = await fetch(api.collections_with_media(user_token.token, id))
  if (res.status !== 200) alert("Error getting collections of " + id) 
  
  let data = await res.json() as any
  
  let ids = data.map((el: any) => el.collectionId);
              
  collection_list[id] = ids;
  
  const loader = target.querySelector(".loader")
  if (loader) loader.remove()
}

export const favoriteMedia = async (id: number, favorite = true) => {
  const target = document.querySelector(`.image-wrapper[data-id="${id}"]`)
  if(!target) return
  
  let loader = target.querySelector(".loader")
  if (loader) loader.remove()
  
  const fastactions = target.querySelector(".fastActions")
  if (fastactions) fastactions.remove()
  
  let spinner = create_loader(20, 3)
  
  target.appendChild(spinner)
  
  if (favorite) {
    await fetch(api.favorite_media(user_token.token, id))
  } else {
    await fetch(api.unfavorite_media(user_token.token, id))
  }
}

export const fastItemsImage = (id: number) => {
  const target = document.querySelector(`.image-wrapper[data-id="${id}"]`)
  if(!target) return
  
  const existing_controls = target.querySelector(".fastActions")
  
  if(existing_controls) {
    existing_controls.remove()
  }
  
  const favorite_collection_id = userState.collections.find((collection: any) => collection.name === "comfyui_civit_favorites").id || -1
  const in_collections = collection_list[id]
      
  // Remove existing if its the case
  const existing = target.querySelector(".fastActions")
  if (existing) existing.remove()
  
  const container = document.createElement("div")
  container.classList.add("fastActions")
  
  let favoriteIcon = document.createElement("span")
  favoriteIcon.classList.add("favoriteIcon")
  
  favoriteIcon.innerHTML = `<svg class="MuiSvgIcon-root MuiSvgIcon-fontSizeMedium css-1phnduy" focusable="false" aria-hidden="true" viewBox="0 0 24 24"><path d="m12 21.35-1.45-1.32C5.4 15.36 2 12.28 2 8.5 2 5.42 4.42 3 7.5 3c1.74 0 3.41.81 4.5 2.09C13.09 3.81 14.76 3 16.5 3 19.58 3 22 5.42 22 8.5c0 3.78-3.4 6.86-8.55 11.54z"></path></svg>`
  
  if (in_collections.includes(favorite_collection_id)) {
    favoriteIcon.classList.add("favorited")
    favoriteIcon.addEventListener("click", async () => {
      await favoriteMedia(id, false)
      await load_collection_data_by_media_id(id)
      fastItemsImage(id)
    })
  } else {
    favoriteIcon.addEventListener("click", async () => {
      await favoriteMedia(id)
      await load_collection_data_by_media_id(id)
      fastItemsImage(id)
    })
  }
  
  let collectionsButton = document.createElement("span")
  collectionsButton.classList.add("Collections")
  collectionsButton.innerHTML = `<svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="#2F9E44" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round" class="tabler-icon tabler-icon-bookmark "><path d="M18 7v14l-6 -4l-6 4v-14a4 4 0 0 1 4 -4h4a4 4 0 0 1 4 4z"></path></svg>`
        
  collectionsButton.addEventListener("click", () => window.dispatchEvent(new CustomEvent("openCollectionSelector", {
    detail: {id, media_type: "Image"}
  })))
  
  container.appendChild(favoriteIcon)
  container.appendChild(collectionsButton)
  target.appendChild(container)
}