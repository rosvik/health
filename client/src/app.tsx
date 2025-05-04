/* @refresh reload */
import { Route, Router } from "@solidjs/router";
import { render } from "solid-js/web";
import "./app.css";
import EndpointPage from "./pages/Endpoint";
import Index from "./pages/Index";

const root = document.getElementById("root");

if (import.meta.env.DEV && !(root instanceof HTMLElement)) {
  throw new Error("Root element not found");
}

render(
  () => (
    <Router>
      <Route path="/" component={Index} />
      <Route path="/:endpoint" component={EndpointPage} />
    </Router>
  ),
  root!
);
