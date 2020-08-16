[%%raw "import('./flow_field/pkg')"]

let component = text =>
  <div className="container">
    <div className="containerTitle">
      <h1>{React.string(text)}</h1>
      <div className="containerContent">
        <ReprocessingDemo />
      </div>
    </div>
  </div>;

switch (ReactDOM.querySelector("#app-root")) {
| Some(root) => ReactDOM.render(component("Hello, world!"), root)
| None => ()
}
