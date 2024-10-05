export default function({ start = 0, step = 1}, dom) {
  const button = document.createElement("button");
  button.innerText = "Click me";
  dom.appendChild(button);

  const text = document.createElement("span");
  dom.appendChild(text);

  let state = start;

  text.innerText = state;
  button.addEventListener("click", () => {
    state += 1;

    text.innerText = state;
  })
}
