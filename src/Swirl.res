module Wasm = {
  type coord = (float, float)

  type fluidPoint = {
    x: int,
    y: int,
    density: float,
  }

  type wasmModule = {
    tick: unit => Js.Array.t<(int, int, float)>,
    mouse_move: (int, int, int, int) => unit,
  }

  let use: unit => Js.Promise.t<wasmModule> = %raw(`() => import('./wasm_swirl/pkg')`)

  let tick: unit => Js.Promise.t<Js.Array.t<(int, int, float)>> = () => {
    use() |> Js.Promise.then_(mod => {
      mod.tick()
        ->Js.Promise.resolve
    })
  }

  let mouseMove = (viewportWidth: int, viewportHeight: int, mouseX: int, mouseY: int) => {
    use() |> Js.Promise.then_(mod => {
      mod.mouse_move(viewportWidth, viewportHeight, mouseX, mouseY)->Js.Promise.resolve
    })
  }
}

module Renderer = {
  let render: Js.Nullable.t<Dom.element> => Js.Promise.t<unit> = %raw(
    `
    canvas => {
      const tap = fn => arg => { fn(arg); return arg; };

      return import('pixi.js')
        .then(pixi => [
          pixi,
          new pixi.Application({
            view: canvas,
            resizeTo: window,
            backgroundColor: 0x000000
          })
        ])
        .then(tap(([pixi, app]) => {
          app.ticker.add(() => {
            //console.time("point_compute");
            const circ = new pixi.Graphics();

            const renderFluid = ([x, y, density]) => {
              console.time("point_draw");

              density = density * 255;
              const densityHex = density.toString(16);
              const color = parseInt(
                densityHex // red
                + densityHex // blue
                + densityHex, // green
                16 // hexadecimal
              );

              x = (x / 64) * window.innerWidth;
              y = (y / 64) * window.innerHeight;
              const radius = 2;

              circ.beginFill(color)
                .drawCircle(x, y, radius)
                .endFill();
              //console.timeEnd("point_draw"); // avg. 0-1ms
            };

            app.stage.addChild(circ)

            console.time("compute");
            Wasm.tick()
            .then(points => {
              console.timeEnd("compute"); // avg 320ms
              console.time("paint");
              return points;
            })
            .then(points => points.forEach(renderFluid))
            .then(() => console.timeEnd("paint")) // avg 6000-8000ms
            .catch(console.error);
          });
        }));
    }
  `
  )
}

type sizedEl = {"innerWidth": float, "innerHeight": float}

@react.component
let make = () => {
  Js.log("render");
  let (hasRendered, hasRenderedSet) = React.useState(_ => false)
  let canvasRef = React.useRef(Js.Nullable.null)

  React.useEffect(_ => {
    let _ = Renderer.render(canvasRef.current)
    None
  })

  let onMouseMove = event => {
    let _ = switch canvasRef.current->Js.Nullable.toOption {
    | Some(el) =>
      el
      ->ReactDOM.domElementToObj
      ->(
        el =>
          Wasm.mouseMove(
            el["width"],
            el["height"],
            event->ReactEvent.Mouse.clientX,
            event->ReactEvent.Mouse.clientY,
          )
      )
    | None => Js.Promise.resolve()
    }
  }

  <canvas id="background_canvas" onMouseMove ref={ReactDOM.Ref.domRef(canvasRef)} />
}
