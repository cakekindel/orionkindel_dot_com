/** A thin 2D Vector implementation */
type vector = { x: float, y: float };

/**
 * Type describing the JS glue code module,
 * wrapping our wasm code
 */
type jsModule = {
    /**
     * Initialize the state within the WASM module
     */
    setup: (. int, int) => unit,

    /**
     * Updates the flow field's internal state,
     * and invokes a closure that draws each particle
     */
    tick: (. (vector, vector) => unit) => unit
};

/**
 * Asynchronously import the module with the wasm
 */
let init = (.) => {
    let jsModule: Js.Promise.t(jsModule) = [%raw {|
        import('./flow_field/pkg')
    |}];

    jsModule
};
