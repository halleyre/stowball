import stowball from "./wasm-pack/stowball.js";

const canvas = document.querySelector("canvas");
const resize = () => {
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;
}
resize()
window.addEventListener('resize', resize)

stowball()
