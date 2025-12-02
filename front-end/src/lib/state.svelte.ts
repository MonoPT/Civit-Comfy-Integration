import api from "./api";

export const userState: any = $state({});
export const loading_user = $state({loading: false, error: ""})
export const user_token = $state({token: ""})


export const loginUser = async (token: string) => {
  loading_user.loading = true
  loading_user.error = ""
  user_token.token = ""
  
  let resp = await fetch(api.user_data(token))
  
  loading_user.loading = false
  
  if (resp.status != 200) {
    loading_user.error = "Token is not valid"
    setCookie("user_token", token, -3)
    return
  }
  
  let response = await resp.json()
  
  user_token.token = token
  
  for (const key of Object.keys(response)) {  
    //@ts-ignore
    userState[key] = response[key]
  }
  
  setCookie("user_token", token, 61)
  window.dispatchEvent(new CustomEvent("FinishloadingUser"));
}

export function setCookie(name: string, value: string, days: number = 7): void {
  const expires = new Date(Date.now() + days * 24 * 60 * 60 * 1000).toUTCString();
  document.cookie = `${encodeURIComponent(name)}=${encodeURIComponent(value)}; expires=${expires}; path=/`;
}

export function getCookie(name: string): string | null {
  const encoded = encodeURIComponent(name) + "=";
  const cookies = document.cookie.split(";");

  for (let c of cookies) {
    c = c.trim();
    if (c.startsWith(encoded)) {
      return decodeURIComponent(c.substring(encoded.length));
    }
  }
  return null;
}