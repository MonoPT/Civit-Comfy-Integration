export type Table = {
  id: string;
  thumbnail: string;
  version: string;
  size: string;
  info: TableModelInfo;
};

export type DownloadInfo = {
  total: number;
  current: number;
  download_speed: number;
}

export type TableModelInfo = {
  name: string;
  author: string;
  base_model: string;
  release_Date: string;
  metrics: {
    like: number;
    downloads: number;
    collected: number;
    donations: number;
  },
  download: DownloadInfo
}

export const data: Table[] = [
  {
    id: "728ed52f",
    thumbnail: "https://image.civitai.com/xG1nkqKTMzGDvpLrqFT7WA/890ad6b4-1c27-4901-8940-1054c3be0146/anim=false,width=450,optimized=true/pixai-1829059562672552554-0.jpeg",
    size: "6.43GB",
    version: "v1.7",
    info: {
      name: "Cat Tower",
      author: "Name of auth",
      base_model: "NoobAI",
      release_Date: "30 jan 2026",
      metrics: {
        like: 0,
        downloads: 12,
        collected: 10,
        donations: 30
      },
      download: {
        total: 300,
        current: 290,
        download_speed: 20
      }
    },
  },
  {
    id: "489e1d42",
    thumbnail: "https://image.civitai.com/xG1nkqKTMzGDvpLrqFT7WA/2e2d5117-98e8-4fe6-acd0-60a6704069ef/anim=false,width=450,optimized=true/1892489752_20260106113541.jpeg",
    size: "575MB",
    version: "v0.7.3",
    info: {
      name: "Example model",
      author: "Amazing dev 1",
      base_model: "SDXL 1.5",
      release_Date: "30 jan 2026",
      metrics: {
        like: 1200,
        downloads: 10000,
        collected: 10,
        donations: 30
      },
      download: {
        total: 900,
        current: 200,
        download_speed: 40
      }
    },
  },
  // ...
];
