export function main() {
  trt.setup_panic_hook();

  const scene = trt.Scene.new();

  const canvas = document.getElementById('canvas') as HTMLCanvasElement;
  const ctx = canvas.getContext('2d') as CanvasRenderingContext2D;

  const MAX_X = 300;
  const MAX_Y = 300;

  let y = MAX_Y - 1;

  const drawPx = (x: number, y: number) => {
    const color = scene.pixel_color(x, y);

    const r = (color & 0x00FF0000) >> 16;
    const g = (color & 0x0000FF00) >> 8;
    const b = (color & 0x000000FF) >> 0;

    // console.log(color, r, g, b)

    ctx.fillStyle = `rgb(${r}, ${g}, ${b})`
    ctx.fillRect(x, MAX_Y - y, 1, 1)
  }

  // let x = 0;

  let start = new Date;

  function draw() {
    for (let x = 0; x < MAX_X; x++) {
      drawPx(x, y);
    }
    y -= 1
    if (y >= 0) {
      setTimeout(draw, 0);
    } else {
      console.log(+new Date() - +start)
    }
  }

  draw()
}
