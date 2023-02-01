console.log("init client...");

//import { render } from "https://cdn.skypack.dev/solid-js/web";
//import html from "https://cdn.skypack.dev/solid-js/html";
//import h from "https://cdn.skypack.dev/solid-js/h";

import { render } from "solid-js/web";

import App from "./components/app.js";
render(App, document.body);

//import('/components/app.js').then(({default}) => {
//import('/components/app.js').then((module) => {
  //console.log("load app...")
  //console.log(module)
  //const App = module.default;
  //render(App, document.body);
//});

//import('/importtest.js').then(({ test }) => {
  // your code
  //console.log("import test...")
  //console.log(test)
  //test();
//});