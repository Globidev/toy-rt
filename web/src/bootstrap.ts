declare global {
  var trt: typeof import('trt');
  var wasm_memory: any;
}

import { main } from "./index";

export async function loadWasm() {
  const trt = await import('trt');

  window.trt = trt;
}

loadWasm()
  .then(main);
