/**
 * custom "tap" infix operator.
 * execute an impure function with a value,
 * disregarding its return value
 */
let (-<-) = (a, fn) => {fn(a); a};
