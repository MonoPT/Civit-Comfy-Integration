import API from "$lib/api";
import {user_token} from "$lib/state.svelte"

type return_assets = {
  status: number,
  assets: object[]
}

export default class load_mansonary {
  static cursor = ""
    
  static reset() {
    load_mansonary.cursor = ""
  }
  
  static default_payload() {
    return {
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
      tags: ""
    }
  }
  
  static async fetch_assets(): Promise<return_assets> {
    let payload = load_mansonary.default_payload()
    let endpoint = API.infinite_images(user_token.token, new URLSearchParams(payload).toString())
    
    let response = await fetch(endpoint)
    
    if (response.status === 200) {
      let res = await response.json() 
      
      let images = (res[0] as object[]).map((asset_item) => {
        //@ts-ignore
        const original_image_url = asset_item.img_url
        
        // Replace otimization settings
        
        const new_url = original_image_url.replace("/transcode=true,original=true,quality=90/", "/transcode=true,width=400,original=false,optimized=true/")
        const new_poster_url = original_image_url.replace("/transcode=true,original=true,quality=90/", "/anim=false,transcode=true,width=400,original=false,optimized=true/")

        
        
        //@ts-ignore
        asset_item.optimized_asset_url = new_url
        
        //@ts-ignore
        asset_item.optimized_poster_img_url = new_poster_url
        
        asset_item.ratio = getAspectRatio(asset_item.width || 0, asset_item.height || 0)
        
        return asset_item
      })
      load_mansonary.cursor = (res[1] as string).replaceAll("\"", "")
      
      return {status: response.status, assets: images}
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