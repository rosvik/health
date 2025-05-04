import { A } from "@solidjs/router";
import { createResource, For, type Component } from "solid-js";
import Ticks from "../components/Ticks";
import { API_BASE_URL, Endpoint } from "../utils";
import styles from "./Index.module.css";

const Index: Component = () => {
  const [endpoints] = createResource(fetchEndpoints, {
    initialValue: [],
  });

  return (
    <div class={styles.container}>
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

async function fetchEndpoints(): Promise<Endpoint[]> {
  const response = await fetch(`${API_BASE_URL}/endpoints`);
  const data = await response.json();
  return data.endpoints;
}

export default Index;
