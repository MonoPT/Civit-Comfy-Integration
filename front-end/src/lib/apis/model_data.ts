import API from "$lib/api"

import { user_token } from "$lib/state.svelte"

export type ModelData = {[key: string]: any};

export async function get_model_data(model_param: string) : Promise<{status: number, data: ModelData}> {
  let model_data = {}
  
  if (model_param.length < 1) return {status: 400, data: model_data}
    
  let model_id = 0
  
  if (isNumberString(model_param)) {
    model_id = parseInt(model_param)
  } else {
    const sp = model_param.split("/models/")
    
    if (sp.length < 2) return {status: 400, data: model_data} 
    
    model_id = parseInt(sp[1].split("/")[0])
  }
  
  try {
    const res = await fetch(API.get_model_by_id(user_token.token, model_id))
    
    if (res.status !== 200) return {status: 400, data: model_data}
    
    const data = await res.json()
    
    return {status: 200, data: data}
  } catch (e) {
    return {status: 400, data: {}}
  }
}

function isNumberString(str: string) {
  return typeof str === "string" && str.trim() !== "" && !Number.isNaN(Number(str));
}