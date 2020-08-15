open React;
open Reprocessing;

let renderReprocessing = () => {
  setScreenId("canvas");
  let setup = env => {
    Env.size(~width=1000, ~height=200, env);
    0.
  };

  let draw = (x, env) => {
    let rect = Draw.rect(~width=2, ~height=2, env);
    let incArray = size => Array.make(size, ())
      |> Array.mapi((ix, _) => ix);

    Draw.background(Constants.black, env);
    Draw.fill(Constants.white, env);

    incArray(1000)
      |> Array.map(ix => int_of_float(x) + ix)
      |> Array.iter(x => {
        incArray(100)
          |> Array.map(y => y * 2)
          |> Array.iter(y => rect(~pos=(x, y)))
      })


    Env.deltaTime(env)
      |> t => (t *. 1.) +. x
  };

  run(~setup, ~draw, ());

  None;
};

[@react.component]
let make = () => {
  useEffect0(renderReprocessing);
  <div><canvas id="canvas" /></div>;
};
