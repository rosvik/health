import { A } from "@solidjs/router";
import { createResource, For, type Component } from "solid-js";
import Ticks, { Tick } from "../components/Ticks";
import { API_BASE_URL, Endpoint } from "../utils";
import styles from "./Index.module.css";

const Index: Component = () => {
  const [endpoints] = createResource(fetchEndpoints, {
    initialValue: [],
  });

  return (
    <div class={styles.container}>
      <TickDemo />
      <h1>Health</h1>
      <For each={endpoints()}>
        {(endpoint) => (
          <A href={`/${endpoint.name}`}>
            <p>{endpoint.name}</p>
            <Ticks endpointName={endpoint.name} />
          </A>
        )}
      </For>
    </div>
  );
};

function TickDemo() {
  const checks = [];
  for (let i = 0; i <= 1000; i += 100) {
    checks.push({ status: 200, responseTime: 1000 - i, createdAt: "" });
  }
  return (
    <div class={styles.tickDemo}>
      <For each={checks}>{(check) => <Tick check={check} />}</For>
    </div>
  );
}

async function fetchEndpoints(): Promise<Endpoint[]> {
  const response = await fetch(`${API_BASE_URL}/endpoints`);
  const data = await response.json();
  return data.endpoints;
}

export default Index;
