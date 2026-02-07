import { dev } from '$app/environment';

let port = "3090" // Default dev server

if (typeof window !== 'undefined' && !dev) {
  const url = new URL(window.location.href)
  port = url.port
}

let endpoint = ""

if (dev) {
  endpoint = `http://127.0.0.1:${port}/civit`
} else {
  endpoint = `/civit`
}

type CollectionType = "Image" | "Model" | "Post" | "Article";

export default class API {
  static endpoint = endpoint
    
  static user_data(token: string) {
    return `${this.endpoint}/user_data?token=${token}`
  }
  
  static infinite_images(token: string, params: string) {
    return `${this.endpoint}/infinite_images?${params}&token=${token}`
  }
  
  static infinite_models(token: string, params: string) {
    return `${this.endpoint}/models?${params}&token=${token}`
  }
  
  static popular_tags(token: string) {
    return `${this.endpoint}/tags_with_id?&token=${token}`
  }
  
  static media_data(token: string, model_id: number, user_id: number) {
    return `${this.endpoint}/media_data/${user_id}/${model_id}?&token=${token}`
  }
  
  static get_collections(token: string) {
    return `${this.endpoint}/get_collections?&token=${token}`
  }
  
  static get_collections_by_media_type(token: string, media_type: string) {
    return `${this.endpoint}/get_collections_by_media?&token=${token}&media_type=${media_type}`
  }
  
  static update_media_collections(token: string, media_id: number,positive: number[], negative: number[], collection_type: CollectionType) {
    return `${this.endpoint}/update_collections/${media_id}?&token=${token}&add=${positive.join(",")}&remove=${negative.join(",")}&collection_type=${collection_type}`
  }
  
  static collections_with_media(token: string, media_id: number, collection_type: CollectionType) {
    return `${this.endpoint}/collection_with_media/${media_id}?&token=${token}&collection_type=${collection_type}`
  }
  
  static favorite_media(token: string, media_id: number) {
    return `${this.endpoint}/favorite_media/${media_id}?token=${token}`
  }
  
  static unfavorite_media(token: string, media_id: number) {
    return `${this.endpoint}/unfavorite_media/${media_id}?token=${token}`
  }
  
  static get_base_models(token: string) {
    return `${this.endpoint}/base_models_list?token=${token}`
  }
  
  static get_tools(token: string) {
    return `${this.endpoint}/get_tools_list?token=${token}`
  }
  
  static get_techniques(token: string) {
    return `${this.endpoint}/get_techniques_list?token=${token}`
  }
  
  static get_model_by_id(token: string, id:number) {
    return `${this.endpoint}/get_model/${id}?token=${token}`
  }
  
  static download_model_by_id(token: string, id: number, type: string, cover: string, base_model: string, model_name: string, author_name: string, published_at: string, based_on_model: string, file_name: string, stats: string) {
    return `${this.endpoint}/download_by_id/${id}?token=${token}&model_type=${type}&cover=${cover}&base_model=${base_model}&model_name=${model_name}&published_at=${published_at}&author_name=${author_name}&based_on_model=${based_on_model}&file_name=${file_name}&stats=${stats}"`
  }
  
  static list_downloads(token: string) {
    return `${this.endpoint}/downloads?token=${token}"`
  }
  
  static create_collection(token: string, collection_name: string, collection_type: string, nsfw: boolean, description: string) {
    return `${this.endpoint}/create_collection?token=${token}&collection_name=${collection_name}&collection_type=${collection_type}&nsfw=${nsfw}&description=${description}"`
  }
  
  static get_gen_data(token: string, media_id: number) {
    return `${this.endpoint}/get_generation_data/${media_id}?token=${token}"`
  }
}
