[@bs.val] external document: Js.t({..}) = "document";

module DummyComponent = {
  [@react.component]
  let make = () =>
    <div>
      <h2>{React.string("Hello, World!")}</h2>
    </div>;
};

let component = text =>
  <div className="container">
    <div className="containerTitle">
      <h1>{React.string(text)}</h1>
      <div className="containerContent">
        <DummyComponent />
      </div>
    </div>
  </div>;

switch (ReactDOM.querySelector("body")) {
| Some(root) => ReactDOM.render(component("Hello, world!"), root)
| None => ()
}
