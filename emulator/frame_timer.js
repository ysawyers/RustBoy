let delay_between_frames = 1000 / 59.7;

let frame_start = performance.now();
let busy_loop = false;

onmessage = (e) => {
  if (e.data === "REQUEST") {
    if (!busy_loop) {
      setTimeout(() => {
        if (performance.now() - frame_start >= delay_between_frames) {
          //   console.log(`Time passed ${performance.now() - frame_start}ms`);

          frame_start = performance.now();
          busy_loop = false;
          postMessage("RENDER");
        } else {
          busy_loop = true;
          //   console.log(
          //     `time remaining... ${delay_between_frames - (performance.now() - frame_start)}ms`
          //   );
          postMessage("WAIT");
        }
      }, 1000 / 130);
    } else {
      if (performance.now() - frame_start >= delay_between_frames) {
        busy_loop = false;
        frame_start = performance.now();
        postMessage("RENDER");
      } else {
        postMessage("WAIT");
      }
    }
  }
};
