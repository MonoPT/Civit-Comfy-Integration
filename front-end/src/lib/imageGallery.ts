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
  
  private static cursor: string = "https://civitai.com/api/v1/images?limit=20&sort=Most Reactions&period=Week&nsfw=false"
  
  static async load_next_images() {
    window.dispatchEvent(new CustomEvent("StartedLoadingImages"))
    
    const response = await fetch(ImageGallery.cursor)
    const data = await response.json()
    
    for (let i in data.items) {
      const img = data.items[i]
      if (!ImageGallery.images.includes(img.url)) {
        const image: Image = {url: img.url, index: ImageGallery.counter, width: img.width, height: img.height, media_type: checkFileType(img.url)}

        ImageGallery.counter += 1
        ImageGallery.images.push(image)
      }
    }
    
    ImageGallery.cursor = data.metadata.nextPage
    
    window.dispatchEvent(new CustomEvent("UpdatedImagesList"))
  }
}