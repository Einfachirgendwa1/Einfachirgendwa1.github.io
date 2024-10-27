import init, { lagrange } from "../pkg/pages.js";

const x = document.getElementById("x-value");
const points = document.getElementById("points");
const output = document.getElementById("output");

async function recalculate() {
  output.textContent = lagrange(x.value, points.value);
}

async function run() {
  await init();
  await recalculate();

  x.addEventListener("input", recalculate);
  points.addEventListener("input", recalculate);
}

run();
