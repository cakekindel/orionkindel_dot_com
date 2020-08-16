type vector = {
    x: float,
    y: float
};

type particle = {
    pos_prev: vector,
    pos: vector,
    vel: vector,
    accel: vector
};

type state = {
    particles: Js.Array.t(particle)
};

type flowFieldModule = {
    setup: (int, int) => state,
    tick: (state) => state
};

let init = () => {
    let import: Js.Promise.t(flowFieldModule) = [%raw "import('./flow_field/pkg')"];
    import
};
