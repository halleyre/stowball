document.getElementById("aye").innerHTML = "start the rescue helicopter";
// wee bit of debug bracketing

/* ------------ *
 * Web Assembly *
 * ------------ */ 
import init, { greet } from "./wasm-pack/stowball.js";
init().then(()  => {
  greet();
});

// probs success
document.getElementById("bee").innerHTML = "build the helicopter";
