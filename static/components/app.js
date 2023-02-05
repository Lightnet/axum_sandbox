
//"https://cdn.skypack.dev/solid-js";
//import { render } from "https://cdn.skypack.dev/solid-js/web";
//import html from "https://cdn.skypack.dev/solid-js/html";
//import h from "https://cdn.skypack.dev/solid-js/h";

import {
  createSignal,
  onCleanup,
  createMemo,
} from "solid-js";

import h from "solid-js/h";

import SignIn from "./auth/signin.js";
import SignUp from "./auth/signup.js";
import Forgot from "./auth/forgot.js";
import ToDoList from "./todolist/todolist.js";

const App = () => {
  console.log("init app2...")
  const [count, setCount] = createSignal(0);
  const [view, setView] = createSignal("signin");

  const timer = setInterval(() => setCount(count() + 1), 1000);
  onCleanup(() => clearInterval(timer));

  function viewRoute(value){
    console.log(value);
    setView(value)
  }

  const renderView = createMemo(()=>{
    if(view() == "signin"){
      return SignIn({view:viewRoute});
    }
    if(view() == "signup"){
      return SignUp({view:viewRoute});
    }
    if(view() == "forgot"){
      return Forgot({view:viewRoute});
    }
    if(view() == "todolist"){
      return ToDoList({view:viewRoute});
    }
  })

  return h("div", {}, renderView);

};

export default App;

//const App = () => {
  //console.log("init app2...")
  //const [count, setCount] = createSignal(0),
    //timer = setInterval(() => setCount(count() + 1), 1000);
  //onCleanup(() => clearInterval(timer));
  //return html`<div>${count}</div>`;
  // or
  //return h("div", {}, count);
//};