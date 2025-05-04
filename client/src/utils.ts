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

export function clamp(value: number, min: number, max: number) {
  return Math.min(Math.max(value, min), max);
}

/**
 * Maps a value from one range to another.
 */
export function mapToRange(
  value: number,
  inputRange: [number, number],
  outputRange: [number, number]
) {
  const [inputMin, inputMax] = inputRange;
  const [outputMin, outputMax] = outputRange;
  const clampedValue = clamp(value, inputMin, inputMax);
  const normalizedValue = (clampedValue - inputMin) / (inputMax - inputMin);
  return outputMin + (outputMax - outputMin) * normalizedValue;
}
