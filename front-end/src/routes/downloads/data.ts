export type Table = {
  id: string;
  thumbnail: string;
  status: "pending" | "processing" | "success" | "failed";
  email: string;
  info: object;
};

export const data: Table[] = [
  {
    id: "728ed52f",
    thumbnail: "https://image.civitai.com/xG1nkqKTMzGDvpLrqFT7WA/890ad6b4-1c27-4901-8940-1054c3be0146/anim=false,width=450,optimized=true/pixai-1829059562672552554-0.jpeg",
    status: "pending",
    email: "m@example.com",
    info: {
      
    }
  },
  {
    id: "489e1d42",
    thumbnail: "https://image.civitai.com/xG1nkqKTMzGDvpLrqFT7WA/2e2d5117-98e8-4fe6-acd0-60a6704069ef/anim=false,width=450,optimized=true/1892489752_20260106113541.jpeg",
    status: "processing",
    email: "example@gmail.com",
    info: {
      
    }
  },
  // ...
];
