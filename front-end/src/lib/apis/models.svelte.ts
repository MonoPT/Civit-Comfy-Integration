import API from "$lib/api";
import {user_token} from "$lib/state.svelte"

type return_assets = {
  status: number,
  assets: object[]
}

import { type Filter } from "$lib/filter";

export default class load_models {
  static cursor = ""
  static media = ""  
  
  static reset() {
    load_models.cursor = ""
  }
  
  static default_payload(override: Filter[] = []) {
    let filter_list: {[key: string]: string} =  {
      period: "Month",
      sort: "Most Reactions",
      baseModel: "",
      mediaType: "Image",
      cursor: load_models.cursor,
      browsingLevel: "31",
      techniques: "",
      requiringMeta: "false",
      madeOnsite: "false",
      originalsOnly: "false",
      remixesOnly: "false",
      metadataOnly: "false", 
      tags: "",
      tools: ""
    }
    
    for (const key in override) {
      const value = override[key];
      
      const selected_name = value.name
      const selected_value = value.value
            
      const val_to_override = filter_list[selected_name]
      
      if (val_to_override !== undefined) { // If key exists in base list, override it with new value
        filter_list[selected_name] = selected_value
      }
    }
    
    return filter_list
  }
  
  static async fetch_assets(override: Filter[] = []): Promise<return_assets> {
    let payload = load_models.default_payload(override)
    let endpoint = API.infinite_models(user_token.token, new URLSearchParams(payload).toString())
        
    let response = await fetch(endpoint)
    
    if (response.status === 200) {
      let res = await response.json() 
      
      let assets = (res[0] as object[]).map((asset_item) => {
        const images = (asset_item as any).images as any[]
        
        
        const img = find_aspect_hor(images)
                
        ; (asset_item as any).img_url = `https://image.civitai.com/xG1nkqKTMzGDvpLrqFT7WA/${img.url}/transcode=true,original=true,quality=100/00000-319863695%202025-03-20.jpeg`
        
        //@ts-ignore
        const original_image_url = asset_item.img_url
              
        const new_url = original_image_url.replace("/transcode=true,original=true,quality=100/", "/transcode=true,width=400,original=false,optimized=true/")
        const new_poster_url = original_image_url.replace("/transcode=true,original=true,quality=100/", "/anim=false,transcode=true,width=400,original=false,optimized=true/")

        //@ts-ignore
        asset_item.optimized_asset_url = new_url
        
        //@ts-ignore
        asset_item.optimized_poster_img_url = new_poster_url
                
        //@ts-ignore
        asset_item.ratio = getAspectRatio(img.width || 0, Math.max(img.height || 0, 420))
        
        //@ts-ignore
        if (asset_item.ratio.h < asset_item.ratio.w) {
          //@ts-ignore
          let w = asset_item.ratio.w
          //@ts-ignore
          asset_item.ratio.w = 3
          //@ts-ignore
          asset_item.ratio.h = 4
        }
        
        return asset_item
      })
      
      
      load_models.cursor = (res[1] as string).replaceAll("\"", "")
                  
      return { status: response.status, assets: assets }
    }
    
    return {status: response.status, assets: []}
  }
}

function find_aspect_hor(images: any[]) {
  let img = images.find((img) => {
    return img.width < img.height
  })
  
  if (!img) {
    img = images[0]
    console.log("does not have")
  }
  
  return img
}

function getAspectRatio(
  width: number,
  height: number,
  maxDepth = 50
) {
  const gcd = (a: number, b: number, depth: number): number => {
    if (depth >= maxDepth) {
      // fallback: stop reducing further
      return a;
    }
    if (b === 0) {
      return a;
    }
    return gcd(b, a % b, depth + 1);
  };

  const divisor = gcd(width, height, 0);

  return {
    w: width / divisor,
    h: height / divisor
  };
}