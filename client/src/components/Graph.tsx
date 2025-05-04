import { createSignal, onMount } from "solid-js";
import { Check } from "../utils";

export default function Graph({ checks }: { checks: Check[] }) {
  const [canvas, setCanvas] = createSignal<HTMLCanvasElement>();
  onMount(() => {
    if (canvas()) drawGraph(canvas()!, checks);
  });

  const scale = 2;
  const width = 1000;
  const height = 500;

  return (
    <canvas
      ref={setCanvas}
      width={width * scale}
      height={height * scale}
      style={{ width: `${width}px`, height: `${height}px` }}
    />
  );
}

function drawGraph(canvas: HTMLCanvasElement, checks: Check[]) {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;

  ctx.clearRect(0, 0, canvas.width, canvas.height);
  ctx.fillStyle = "white";

  const maxValue = Math.max(...checks.map((check) => check.responseTime)) + 50;
  const areaStartX = 80;
  const areaStartY = 40;
  const areaHeight = canvas.height - 80;
  const barWidth = 4;
  const barPadding = 2;

  // Draw area border
  ctx.strokeStyle = "#ffffff22";
  ctx.strokeRect(areaStartX, areaStartY, canvas.width - areaStartX, areaHeight);

  // Draw horizontal lines every 100ms
  for (let i = 0; i <= maxValue; i += 100) {
    const y = areaStartY + areaHeight - (i / maxValue) * areaHeight;
    ctx.beginPath();
    ctx.moveTo(areaStartX, y);
    ctx.lineTo(canvas.width, y);
    ctx.stroke();
  }

  // Draw labels every 100ms
  ctx.textAlign = "right";
  ctx.font = "24px Arial";
  for (let i = 0; i <= maxValue; i += 100) {
    const y = areaStartY + areaHeight - (i / maxValue) * areaHeight + 8;
    ctx.fillText(i.toString(), areaStartX - 10, y);
  }

  // Draw the graph
  ctx.fillStyle = "white";
  let i = 0;
  for (const check of checks) {
    const barHeight = (check.responseTime / maxValue) * areaHeight;
    const x = areaStartX + i * (barWidth + barPadding);
    const y = areaStartY + areaHeight - barHeight;
    ctx.fillRect(x, y, barWidth, barHeight);
    i++;
  }
}
