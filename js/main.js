import init, {
  initialize_new,
  lagrange,
  readme_markdown,
} from "../pkg/pages.js";

const readme_md_output = document.getElementById("readme-markdown-output");

const x = document.getElementById("x-value");
const points = document.getElementById("points");
const output = document.getElementById("output");

async function recalculate() {
  output.textContent = lagrange(x.value, points.value);
}

async function run() {
  await init();
  initialize_new();

  readme_md_output.innerHTML = marked.parse(readme_markdown());
  await recalculate();

  x.addEventListener("input", recalculate);
  points.addEventListener("input", recalculate);
}

run();
