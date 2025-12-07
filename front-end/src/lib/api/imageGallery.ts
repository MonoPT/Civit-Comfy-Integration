import FilterManager from "./filterManager"
import api from "$lib/api"

type Stats = {
  collectedCountAllTime: number
  commentCountAllTime: number
  cryCountAllTime: number
  dislikeCountAllTime: number
  heartCountAllTime: number
  laughCountAllTime: number
  likeCountAllTime: number
  tippedAmountCountAllTime: number
  viewCountAllTime: number
}

export interface Image {
  url: string,
  index: number,
  width: number,
  height: number,
  media_type: string,
  id: number,
  stats: Stats
}

export class ImageGallery {
  static images: Image[] = []
  private static counter = 0;
    
  static cursor: string | null = null
  
  static isFetching = false
  
  static async load_next_images(user_token: string) {
    if(ImageGallery.isFetching) return
    
    ImageGallery.isFetching = true
    window.dispatchEvent(new CustomEvent("StartedLoadingImages"))
    let params = FilterManager.filters
    
    let period = (params["timePeriod"] || "Day").replace("All Time", "AllTime")
    let sort = params["sort"] || "Most Reactions" 
    let baseModel = Array.from((params['baseModel'] || "").split(",")).filter((i) => i.length > 0)
    let type = Array.from((params['mediaType'] || "").split(",")).filter((i) => i.length > 0)
    let browsingLevel = params["Show NSFW"] ? 31 : 1       
    let techniques = Array.from((params['techniques'] || "").split(",")).filter((i) => i.length > 0).map((i) => parseInt(i)).join(",")
    let requiringMeta = params["Metadata only"] ? true : false
    let madeOnsite = params["Made On-site"] ? true : false
    let originalsOnly = params["Originals Only"] ? true : false
    let remixesOnly = params["Remixes Only"] ? true : false
    let tags = Array.from((params['tags'] || "").split(",")).filter((i) => i.length > 0)
        
    let param_string = `period=${period}&tags=${tags.join(",")}&originalsOnly=${originalsOnly}&remixesOnly=${remixesOnly}&madeOnsite=${madeOnsite}&requiringMeta=${requiringMeta}&sort=${sort}&baseModel=${baseModel}&mediaType=${type}&cursor=${ImageGallery.cursor}&browsingLevel=${browsingLevel}&techniques=${techniques}`;
    
    const response = await fetch(`${api.infinite_images(user_token, param_string)}`)
    ImageGallery.isFetching = false
    const data = await response.json()
    
    ImageGallery.cursor = data[1].replaceAll('"', "");
    let image_data = data[0]
        
    for (let i in image_data) {
      const img = image_data[i]
      if (!ImageGallery.images.includes(img.img_url)) {
        const image: Image = {id: img.id, url: img.img_url, index: ImageGallery.counter, width: img.width, height: img.height, media_type: img.type, stats: img.stats}
        
        ImageGallery.counter += 1
        ImageGallery.images.push(image)
      }
    }
    
    
    
    window.dispatchEvent(new CustomEvent("UpdatedImagesList"))
  }
}

