'use strict';

var React = require("react");
var ReactDom = require("react-dom");

function Index$DummyComponent(Props) {
  return React.createElement("div", undefined, React.createElement("h2", undefined, "Hello, World!"));
}

var DummyComponent = {
  make: Index$DummyComponent
};

function component(text) {
  return React.createElement("div", {
              className: "container"
            }, React.createElement("div", {
                  className: "containerTitle"
                }, React.createElement("h1", undefined, text), React.createElement("div", {
                      className: "containerContent"
                    }, React.createElement(Index$DummyComponent, {}))));
}

var root = document.querySelector("body");

if (!(root == null)) {
  ReactDom.render(component("Hello, world!"), root);
}

exports.DummyComponent = DummyComponent;
exports.component = component;
/* root Not a pure module */
