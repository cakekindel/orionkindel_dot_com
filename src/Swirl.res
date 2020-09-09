module Wasm = {
  let tick = () => {
    ();
  };
}

module Renderer = {
  let render = %raw(`
    canvas => {
      const tap = fn => arg => { fn(arg); return arg };

      return import('pixi.js')
      .then(pixi => [
        pixi,
        {
          view: canvas,
          resizeTo: window,
          backgroundColor: 0x000000
        }
      ])
      .then(([pixi, options]) => [pixi, new pixi.Application(options)])
      .then(([pixi, app]) => {
        const circ = new pixi.Graphics();
        circ.beginFill(0xFFFFFF),
        circ.drawCircle(10, 10, 4);
        circ.endFill();

        return [pixi, app, circ];
      })
      .then(tap(([_, app, circ]) => app.stage.addChild(circ)));
      .catch(console.error)
    }
  `);
}

@react.component
let make = () => {
  let (hasRendered, hasRenderedSet) = React.useState(_ => false);
  let canvasRef = React.useRef(Js.Nullable.null);

  React.useEffect(_ => {
    Renderer.render(canvasRef.current);
    None;
  })

  <canvas id="background_canvas" ref={ReactDOM.Ref.domRef(canvasRef)}></canvas>
};
