'use strict';

var $$Array = require("bs-platform/lib/js/array.js");
var React = require("react");
var Caml_array = require("bs-platform/lib/js/caml_array.js");
var Reprocessing = require("reprocessing/src/Reprocessing.js");
var Reprocessing_Env = require("reprocessing/src/Reprocessing_Env.js");
var Reprocessing_Draw = require("reprocessing/src/Reprocessing_Draw.js");
var Reprocessing_Constants = require("reprocessing/src/Reprocessing_Constants.js");

function renderReprocessing(param) {
  Reprocessing.setScreenId("canvas");
  var setup = function (env) {
    Reprocessing_Env.size(1000, 200, env);
    return 0;
  };
  var draw = function (x, env) {
    var incArray = function (size) {
      return $$Array.mapi((function (ix, param) {
                    return ix;
                  }), Caml_array.caml_make_vect(size, undefined));
    };
    Reprocessing_Draw.background(Reprocessing_Constants.black, env);
    Reprocessing_Draw.fill(Reprocessing_Constants.white, env);
    $$Array.iter((function (x) {
            return $$Array.iter((function (y) {
                          var param = [
                            x,
                            y
                          ];
                          return Reprocessing_Draw.rect(param, 2, 2, env);
                        }), $$Array.map((function (y) {
                              return (y << 1);
                            }), incArray(100)));
          }), $$Array.map((function (ix) {
                return (x | 0) + ix | 0;
              }), incArray(1000)));
    var t = Reprocessing_Env.deltaTime(env);
    return t * 1 + x;
  };
  Reprocessing.run(setup, undefined, draw, undefined, undefined, undefined, undefined, undefined, undefined, undefined, undefined);
  
}

function ReprocessingDemo(Props) {
  React.useEffect((function () {
          return renderReprocessing(undefined);
        }), []);
  return React.createElement("div", undefined, React.createElement("canvas", {
                  id: "canvas"
                }));
}

var make = ReprocessingDemo;

exports.renderReprocessing = renderReprocessing;
exports.make = make;
/* react Not a pure module */
