let component = text =>
  <div className="container">
    <div className="containerContent">
      <Swirl />
    </div>
  </div>;

switch (ReactDOM.querySelector("#app-root")) {
| Some(root) => ReactDOM.render(component("Hello, world!"), root)
| None => ()
}
