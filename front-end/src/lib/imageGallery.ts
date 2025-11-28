import FilterManager from "./filterManager"
import qs from "qs"

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
  
  //private static cursor: string = "https://civitai.com/api/v1/images?limit=20&sort=Most Reactions&period=Week&nsfw=false"
  
  static cursor: string | null = null
  
  static async load_next_images() {
    window.dispatchEvent(new CustomEvent("StartedLoadingImages"))
    
    let params = FilterManager.filters
    
    let period = params["period"] || "Day"
    let sort = params["sort"] || "Most Reactions" 
    let baseModel = Array.from((params['baseModel'] || "").split(",")).filter((i) => i.length > 0)
    let types = Array.from((params['mediaType'] || "").split(",")).filter((i) => i.length > 0)
    
    let urlParams = {
      "json": {
        "period": "Year",
        "periodMode": "published",
        "sort": "Most Reactions",
        "types": [],
        "withMeta": false,
        "fromPlatform": false,
        "hideAutoResources": false,
        "hideManualResources": false,
        "notPublished": false,
        "scheduled": false,
        "hidden": false,
        "remixesOnly": false,
        "nonRemixesOnly": false,
        "requiringMeta": false,
        "useIndex": true,
        "browsingLevel": 1,
        "include": ["cosmetics"],
        "excludedTagIds": [],
        "disablePoi": true,
        "disableMinor": false,
        "cursor": null
      },
      "meta": { "values": { "cursor": ["undefined"] } }
    }
    
    const stringParams = encodeURI(JSON.stringify(urlParams));;
    
    const response = await fetch(`https://civitai.com/api/trpc/image.getInfinite?input=${stringParams}`, {
      method: "GET",
      mode: "no-cors"
    })
    const data = await response.json()
    
    console.log(data)
    /*
    for (let i in data.items) {
      const img = data.items[i]
      if (!ImageGallery.images.includes(img.url)) {
        const image: Image = {url: img.url, index: ImageGallery.counter, width: img.width, height: img.height, media_type: checkFileType(img.url)}

        ImageGallery.counter += 1
        ImageGallery.images.push(image)
      }
      }*/
    
    //ImageGallery.cursor = data.metadata.nextPage
    
    //window.dispatchEvent(new CustomEvent("UpdatedImagesList"))
  }
}

