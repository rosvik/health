export type Endpoint = {
  name: string;
  url: string;
};

export type Check = {
  status: number;
  responseTime: number;
  createdAt: string;
};

export type ChecksResponse = {
  name: string;
  url: string;
  interval: number;
  statusCode: number;
  checks: Check[];
};
