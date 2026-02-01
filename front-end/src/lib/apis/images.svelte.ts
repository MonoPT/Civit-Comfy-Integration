import API from "$lib/api";
import {user_token} from "$lib/state.svelte"

type return_assets = {
  status: number,
  assets: object[]
}

import { type Filter } from "$lib/filter";

export default class load_mansonary {
  static cursor = ""
  static media = ""  
  
  static reset() {
    load_mansonary.cursor = ""
  }
  
  static default_payload(override: Filter[] = []) {
    let filter_list: {[key: string]: string} =  {
      period: "Month",
      sort: "Most Reactions",
      baseModel: "",
      mediaType: "Image",
      cursor: load_mansonary.cursor,
      browsingLevel: "31",
      techniques: "",
      requiringMeta: "false",
      madeOnsite: "false",
      originalsOnly: "false",
      remixesOnly: "false",
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
    let payload = load_mansonary.default_payload(override)
    let endpoint = API.infinite_images(user_token.token, new URLSearchParams(payload).toString())
        
    let response = await fetch(endpoint)
    
    if (response.status === 200) {
      let res = await response.json() 
      
      let assets = (res[0] as object[]).map((asset_item) => {
        //@ts-ignore
        const original_image_url = asset_item.img_url
        
        // Replace otimization settings
        
        const new_url = original_image_url.replace("/transcode=true,original=true,quality=90/", "/transcode=true,width=400,original=false,optimized=true/")
        const new_poster_url = original_image_url.replace("/transcode=true,original=true,quality=90/", "/anim=false,transcode=true,width=400,original=false,optimized=true/")

        //@ts-ignore
        asset_item.optimized_asset_url = new_url
        
        //@ts-ignore
        asset_item.optimized_poster_img_url = new_poster_url
        
        //@ts-ignore
        asset_item.ratio = getAspectRatio(asset_item.width || 0, asset_item.height || 0)
        
        return asset_item
      })
      load_mansonary.cursor = (res[1] as string).replaceAll("\"", "")
      
      console.log(assets)
      
      return {status: response.status, assets: assets}
    }
    
    return {status: response.status, assets: []}
  }
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