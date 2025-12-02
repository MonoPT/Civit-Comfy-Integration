export default class API {
  static endpoint = "http://127.0.0.1:3090"
  
  static user_data(token: string) {
    return `${this.endpoint}/user_data?token=${token}`
  }
  
  static infinite_images(token: string, params: string) {
    return `${this.endpoint}/infinite_images?${params}&token=${token}`
  }
}