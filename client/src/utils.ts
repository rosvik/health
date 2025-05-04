export const API_BASE_URL = "/api/health/v1";

export type Endpoint = {
  name: string;
  url: string;
};

export type Check = {
  status: number;
  responseTime: number;
  createdAt: string;
};
