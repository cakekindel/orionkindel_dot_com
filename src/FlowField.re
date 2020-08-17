/**
 * Type describing the JS glue code module,
 * wrapping our wasm code
 */
type jsModule = {
    /**
     * Accepts the canvas' width, height,
     * and returns a State object
     */
    setup: (. int, int) => unit,

    /**
     * Updates the flow field's internal state,
     * and invokes a closure that draws each particle
     *
     * drawParticle closure expects 4 arguments:
     * - particle's current position x-value
     * - particle's current position y-value
     * - particle's previous position x-value
     * - particle's previous position y-value
     */
    tick: (. (float, float, float, float) => unit) => unit
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
