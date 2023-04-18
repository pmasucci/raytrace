document.addEventListener('DOMContentLoaded', () => {
  const IMAGE_WIDTH = 30;
  const ASPECT_RATIO = 1;
  const IMAGE_HEIGHT = Math.floor(IMAGE_WIDTH / ASPECT_RATIO);
  const button = document.querySelector("button");
  const ctx = document.querySelector('canvas').getContext('2d');
  const picture = ctx.createImageData(IMAGE_WIDTH, IMAGE_HEIGHT);
  ctx.canvas.width = IMAGE_WIDTH;
  ctx.canvas.height = IMAGE_HEIGHT;
  let url = new URL("/ws", window.location.href);
  url.protocol = url.protocol.replace("http", "ws");
  let ws = new WebSocket(url.href);

  button.addEventListener("click", (e) => {
    e.preventDefault();
    ws.send("beep");
  });
  let counter = 0;
  const processQueue = [];
  ws.onmessage = (ev) => {
    if (ev.data === "end") {
      render();
    } else {
      let line = JSON.parse(ev.data);
      processQueue.push(line);
    }
  }

  const render = () => {
    let lineNos = {};
    while (processQueue.length > 0) {
      let line = processQueue.pop();
      for (let x = 0; x < line.pixels.length; x += 3) {
        let imageIndex = (line.row * IMAGE_WIDTH * 4) + (x / 3) * 4;
        const red = line.pixels[x];
        const green = line.pixels[x + 1];
        const blue = line.pixels[x + 2];
        const alpha = 255;
        if(imageIndex % 4 !== 0){
          debugger;
        }
        picture.data[imageIndex] = red;
        picture.data[imageIndex + 1] = green;
        picture.data[imageIndex + 2] = blue;
        picture.data[imageIndex + 3] = alpha;
        if (x === 0) {
          console.log(line);
          console.log(`image index: ${imageIndex}`);
          console.log(`line number ${line.row}`);
        }
      }

    }

    for (let n = 0; n < picture.data.length; n += 4) {
      console.log(`ln ${4 + (n / 4)}`, picture.data[n], picture.data[n + 1], picture.data[n + 2], picture.data[n + 3],`px ${n / 4}`);
    }
    ctx.putImageData(picture, 0, 0);
    console.log(lineNos);
  }
});


// problem child at pixel 23
// row = 0. x = 23
// 0, 3, 6, 9, 12, 15, 18, 21, 24

