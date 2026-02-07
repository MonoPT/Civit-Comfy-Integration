import API from "./api";
import {type CollectionType} from "./api"

import { user_token } from "./state.svelte";

type UpdateCollectionResp = {
  collections: any[];
  media_in_collections: any[],
  isFavorite: boolean
}

export async function update_collections(media_id: number, collection_type: CollectionType): Promise<UpdateCollectionResp> { 
  let media_in_collections: any[] = []
  let isFavorite = false
  
  const [res1, res2] = await Promise.all([
      fetch(API.collections_with_media(user_token.token ,media_id, collection_type)),
      fetch(API.get_collections_by_media_type(user_token.token, collection_type)),
  ]);
        
  let collections: any[] = []
  
  if (res1.status !== 200 || res2.status !== 200) return {
    collections,
    media_in_collections,
    isFavorite
  };
  
  collections = await res1.json() as any[];
  const allCollections = await res2.json() as any[];
    
  let favorite_coll_id = 0
  
  if (collection_type === "Model") {
    favorite_coll_id = allCollections.find((coll) => coll.name === "Liked Models").id as number;
  } else {
    favorite_coll_id = allCollections.find((coll) => coll.name === "comfyui_civit_favorites").id as number;
  }
    
  if (collections.find((coll) => coll.collectionId === favorite_coll_id)) {
    isFavorite = true
  }
  
  media_in_collections = collections
  
  return {
    collections,
    media_in_collections,
    isFavorite
  }
}

export async function favorite_media(media_id: number, isFavorite: boolean, collection_type: CollectionType) : Promise<UpdateCollectionResp> {
  
  let result;
  
  if (isFavorite) {
    result = await fetch(API.unfavorite_media(user_token.token, media_id, collection_type))
  } else {
    result = await fetch(API.favorite_media(user_token.token, media_id, collection_type))
  }
  
  if (result.status !== 200) return {
    collections: [],
    media_in_collections: [],
    isFavorite: isFavorite
  };
  
  return await update_collections(media_id, collection_type)
}