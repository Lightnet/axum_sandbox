
import {
  createSignal,
  onCleanup,
} from "solid-js";
import h from "solid-js/h";

function Login(props){

  const [alias, setAlias] = createSignal("test");
  const [passphrase, setPassphrase] = createSignal("test");

  function InputAlias(e){
    console.log("typing alias...")
    setAlias(e.target.value);
  }

  function InputPassphrase(e){
    console.log("typing alias...")
    setPassphrase(e.target.value);
  }

  function btnSignIn(){
    console.log("btnSignIn...")
  }
  function btnSignUp(){
    console.log("btnSignUp...")
    if(typeof props.view == 'function'){
      props.view('signup')
    }
  }
  function btnForgot(){
    console.log("btnForgot...")
    if(typeof props.view == 'function'){
      props.view('forgot')
    }
  }

  return h("table",{}, h("tbody",{},
    h("tr",{},
      h("td",{colspan:2},
        h("label",{},"Sign In"),
      ),
    ),
    h("tr",{},
      h("td",{},h("label",{},"Alias:")),
      h("td",{},h("input",{value:alias(),onInput:InputAlias})),
    ),
    h("tr",{},
      h("td",{},h("label",{},"Passphrase:")),
      h("td",{},h("input",{value:passphrase(),onInput:InputPassphrase})),
    ),
    h("tr",{},
      h("td",{colspan:2},
        h("button",{onClick:btnSignUp},"Sign Up"),
        h("button",{onClick:btnSignIn},"Sign In"),
        h("button",{onClick:btnForgot},"Forgot"),
      ),
    ),
  ))
}

export default Login;