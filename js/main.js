import init, { lagrange } from "../pkg/pages.js";

async function run() {
  await init();

  const form = document.getElementById("lagrange-form");
  const output = document.getElementById("output");

  form.addEventListener("submit", (event) => {
    event.preventDefault();

    const x = document.getElementById("x-value").value;
    const pointsInput = document.getElementById("points").value;

    const result = lagrange(x, pointsInput);
    output.textContent = result;
  });
}

run();
