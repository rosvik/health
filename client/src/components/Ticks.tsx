import { createResource, For } from "solid-js";
import { API_BASE_URL, Check } from "../utils";
import styles from "./Ticks.module.css";

export default function Ticks({ endpointName }: { endpointName: string }) {
  const [checks] = createResource(endpointName, fetchChecks, {
    initialValue: [],
  });
  return (
    <div class={styles.container}>
      <For each={checks()}>{(check) => <Tick check={check} />}</For>
    </div>
  );
}

function Tick({ check }: { check: Check }) {
  return (
    <div
      class={`${styles.check}`}
      title={`${check.status}, ${check.responseTime}, ${check.createdAt}`}
      style={{
        "background-color": getStatus(check.responseTime, check.status),
      }}
    />
  );
}

function getStatus(responseTime: number, status: number) {
  if (status !== 200) {
    return "rgb(58, 47, 215)";
  }

  // 0ms -> green (hue 120)
  // 1000ms -> red (hue 0)
  return `hsl(${Math.max(0, 120 - responseTime / 10)}, 100.00%, 50%)`;
}

async function fetchChecks(endpointName: string): Promise<Check[]> {
  let response = await fetch(
    `${API_BASE_URL}/checks/${endpointName}?limit=300`
  );
  let data = await response.json();
  return data.checks;
}
