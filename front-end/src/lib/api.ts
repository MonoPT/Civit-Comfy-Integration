export default class API {
  static endpoint = "http://127.0.0.1:3090"
  
  static user_data(token: string) {
    return `${this.endpoint}/user_data?token=${token}`
  }
  
  static infinite_images(token: string, params: string) {
    return `${this.endpoint}/infinite_images?${params}&token=${token}`
  }
  
  static popular_tags(token: string) {
    return `${this.endpoint}/tags_with_id?&token=${token}`
  }
  
  static media_data(token: string, model_id: number, user_id: number) {
    return `${this.endpoint}/media_data/${user_id}/${model_id}?&token=${token}`
  }
  
  static collections_with_media(token: string, media_id: number) {
    return `${this.endpoint}/collection_with_media/${media_id}?&token=${token}`
  }
  
  static favorite_media(token: string, media_id: number) {
    return `${this.endpoint}/favorite_media/${media_id}?&token=${token}`
  }
  
}