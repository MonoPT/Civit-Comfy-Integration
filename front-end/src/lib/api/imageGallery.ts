import FilterManager from "./filterManager"
import api from "$lib/api"

export enum MediaType {
  Image,
  Video
}

export interface Image {
  url: string,
  index: number,
  width: number,
  height: number,
  media_type: MediaType
}

const imageExtensions = ['.jpg', '.jpeg', '.png', '.gif', '.bmp', '.tiff'];
const videoExtensions = ['.mp4', '.avi', '.mkv', '.mov', '.flv', '.webm'];

function checkFileType(filename: string): MediaType {
    // Get the file extension (case-insensitive)
    const ext = filename.slice(((filename.lastIndexOf(".") - 1) >>> 0) + 2).toLowerCase();

    if (imageExtensions.includes(`.${ext}`)) {
        return MediaType.Image;
    } else if (videoExtensions.includes(`.${ext}`)) {
        return MediaType.Video;
    } else {
        return MediaType.Image;
    }
}

export class ImageGallery {
  static images: Image[] = []
  private static counter = 0;
    
  static cursor: string | null = null
  
  static async load_next_images(user_token: string) {
    window.dispatchEvent(new CustomEvent("StartedLoadingImages"))
    let params = FilterManager.filters
    
    let period = params["period"] || "Day"
    let sort = params["sort"] || "Most Reactions" 
    let baseModel = Array.from((params['baseModel'] || "").split(",")).filter((i) => i.length > 0)
    let type = Array.from((params['mediaType'] || "").split(",")).filter((i) => i.length > 0)
    
    let param_string = `period=${period}&sort=${sort}&baseModel=${baseModel}&type=${type}&cursor=${ImageGallery.cursor}`;
    
    const response = await fetch(`${api.infinite_images(user_token, param_string)}`)
    const data = await response.json()
    
    ImageGallery.cursor = data[1].replaceAll('"', "");
    let image_data = data[0]
        
    for (let i in image_data) {
      const img = image_data[i]
      if (!ImageGallery.images.includes(img.img_url)) {
        const image: Image = {url: img.img_url, index: ImageGallery.counter, width: img.width, height: img.height, media_type: checkFileType(img.url)}

        ImageGallery.counter += 1
        ImageGallery.images.push(image)
      }
    }
    
    
    
    window.dispatchEvent(new CustomEvent("UpdatedImagesList"))
  }
}

