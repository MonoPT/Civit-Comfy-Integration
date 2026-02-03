import API from "$lib/api"
import { user_token } from "$lib/state.svelte"

export type ModelDownload = {
  id: number, 
  type: string,
  cover: string,
  base_model: string,
  model_name: string,
  author_name: string,
  published_at: string,
  file_name: string,
  based_on_model: string,
}

export type DownloadProgress = {
  id: string;
  author_name: string;
  base_model:string;
  cover: string;
  download_speed: number;
  downloaded: number;
  file_name: string;
  finished_at: string | null;
  model_name: string;
  model_payload: string;
  published_at: string;
  started_at: string;
  status: string;
  total_size: number;
  based_on_model: string,
}

export function start_download(downloads: ModelDownload[]) {
  downloads.forEach((model) => {
    fetch(API.download_model_by_id(user_token.token, model.id, model.type, model.cover, model.base_model, model.model_name, model.author_name, model.published_at, model.based_on_model, model.file_name))
  })
}