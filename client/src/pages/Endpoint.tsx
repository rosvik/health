import { useParams } from "@solidjs/router";
import { createResource, Show, type Component } from "solid-js";
import Graph from "../components/Graph";
import Ticks from "../components/Ticks";
import { API_BASE_URL, Check } from "../utils";
import styles from "./Endpoint.module.css";

const EndpointPage: Component = () => {
  const { endpoint } = useParams();
  const [checks] = createResource(endpoint, fetchChecks, {
    initialValue: [],
  });

  return (
    <div class={styles.container}>
      <h1>{endpoint}</h1>
      <Ticks endpointName={endpoint} />
      <Show when={checks().length > 0}>
        <Graph checks={checks()} />
      </Show>
    </div>
  );
};

async function fetchChecks(endpoint: string): Promise<Check[]> {
  let response = await fetch(`${API_BASE_URL}/checks/${endpoint}?limit=500`);
  let data = await response.json();
  return data.checks;
}

export default EndpointPage;
