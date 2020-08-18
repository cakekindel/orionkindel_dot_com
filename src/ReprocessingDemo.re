open React;
open Reprocessing;
open Infix;

let canvasId = "canvas";

let to_float_rgb = n => n->float_of_int->(n => n /. 255.);

let bg_color: colorT = {
  r: 67->to_float_rgb,
  g: 35->to_float_rgb,
  b: 113->to_float_rgb,
  a: 1.,
};

let fg_color: colorT = {
  r: 250->to_float_rgb,
  g: 174->to_float_rgb,
  b: 123->to_float_rgb,
  a: 1.,
};

let render = () => {
  let _ =
    FlowField.init(.)
    |> Js.Promise.then_((flowField: FlowField.jsModule) => {
         setScreenId(canvasId);

         let setup = env => {
           let width = 1500;
           let height = 1000;

           let _ =
             env
             -<- Env.size(~width, ~height)
             // -<- Draw.background(Constants.black)
             -<- Draw.stroke(fg_color)
             -<- Draw.strokeCap(Round)
             -<- Draw.strokeWeight(1);

           Draw.background(bg_color, env);
           flowField.setup(. width, height);
         };

         let draw = (_, env) => {
           flowField.tick(. (pos, prev) => {
             let p1 = (pos.x, pos.y);
             let p2 = (prev.x, prev.y);

             Draw.linef(~p1, ~p2, env);
             ();
           });
         };

         run(~setup, ~draw, ());
         Js.Promise.resolve();
       });
  ();
};

[@react.component]
let make = () => {
  let (rendered, setRendered) = useState(_ => false);

  if (!rendered) {
    render();
    setRendered(_ => true);
  };

  <div> <canvas id="canvas" /> </div>;
};
