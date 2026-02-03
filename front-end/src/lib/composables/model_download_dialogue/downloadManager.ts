import API from "$lib/api"
import { user_token } from "$lib/state.svelte"

export function start_download(downloads: {id: number, type: string}[]) {
  downloads.forEach((model) => {
    fetch(API.download_model_by_id(user_token.token, model.id, model.type))
  })
}