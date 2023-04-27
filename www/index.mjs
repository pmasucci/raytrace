document.addEventListener('DOMContentLoaded', () => {
  const IMAGE_WIDTH = 3840;
  const ASPECT_RATIO = 16/9;
  const IMAGE_HEIGHT = Math.floor(IMAGE_WIDTH / ASPECT_RATIO);
  const renderButton = document.querySelector(".render-button");
  const menuButton = document.querySelector(".settings-menu");
  const menu = document.querySelector(".form-container");
  const ctx = document.querySelector('canvas').getContext('2d');
  const picture = ctx.createImageData(IMAGE_WIDTH, IMAGE_HEIGHT);
  // Inputs
  let width = document.querySelector("#width");
  let samples = document.querySelector("#samples");
  
  let menuIsOpen = false;
  menuButton.addEventListener("click", (e) => {
    e.preventDefault();
    let add = menuIsOpen ? "closed" : "open";
    let remove = !menuIsOpen ? "closed": "open";
    menu.classList.add(add);
    menu.classList.remove(remove)
    menuIsOpen = !menuIsOpen;
  });
  
  ctx.canvas.width = IMAGE_WIDTH;
  ctx.canvas.height = IMAGE_HEIGHT;
  let url = new URL("/ws", window.location.href);
  url.protocol = url.protocol.replace("http", "ws");
  let ws = new WebSocket(url.href);

  renderButton.addEventListener("click", (e) => {

    e.preventDefault();
    ws.send({
      event: "RENDER",
      settings: {
        width: width.value,
        samples: samples.value
      }
    });
  });
  let counter = 0;
  let done = false;
  const closeConnection = () => {
    done = false;
  }
  const processQueue = [];
  ws.onmessage = (ev) => {
    if (ev.data === "end") {
      closeConnection();
    } else {
      let line = JSON.parse(ev.data);
      processQueue.push(line);
    }
  }
  
  ws.onerror = closeConnection;
  ws.onclose = closeConnection;

  
  const render = (lines) => {
    while (lines.length > 0) {
      let line = lines.pop();
      for (let x = 0; x < line.pixels.length; x += 3) {
        let imageIndex = (line.row * IMAGE_WIDTH * 4) + (x / 3) * 4;
        const red = line.pixels[x];
        const green = line.pixels[x + 1];
        const blue = line.pixels[x + 2];
        const alpha = 255;
        picture.data[imageIndex] = red;
        picture.data[imageIndex + 1] = green;
        picture.data[imageIndex + 2] = blue;
        picture.data[imageIndex + 3] = alpha;
      }
    }


    ctx.putImageData(picture, 0, 0);
  }

  const step = () => {
    if (processQueue.length > 0) {
      render(processQueue.splice(0, 10));
    }

    if (!done || processQueue.length > 0) {
      window.requestAnimationFrame(step);
    }

  }

  window.requestAnimationFrame(step);

});
