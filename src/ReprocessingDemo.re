open React;
open Reprocessing;

let drawParticle = (env, par: FlowField.particle) => {
  Draw.linef(
    ~p1=(par.pos.x, par.pos.y),
    ~p2=(par.pos_prev.x, par.pos_prev.y),
    env
  );
  ();
};

let rendered = ref(false);

let renderReprocessing = () => {
  let _ = FlowField.init()
    |> Js.Promise.then_((flowField: FlowField.flowFieldModule) => {

      setScreenId("canvas");
      let setup = env => {
        let width = 2000;
        let height = 1000;
        Env.size(~width, ~height, env);
        Draw.background(Constants.white, env);
        flowField.setup(width, height);
      };

      let draw = (state: FlowField.state, env) => {
        /* Draw each particle in the field */
        Draw.strokeCap(Square, env);
        Draw.stroke(Constants.white, env);
        Draw.strokeWeight(1, env);
        Array.iter(drawParticle(env), state.particles);

        flowField.tick(state);
      };

      run(~setup, ~draw, ());
      rendered := true;
      Js.Promise.resolve(());
    });

  None;
};

[@react.component]
let make = () => {
  if (!rendered^) {
    let _ = renderReprocessing();
  };
  <div><canvas id="canvas" /></div>;
};
