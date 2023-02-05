
import {
  createSignal,
  onCleanup,
} from "solid-js";
import h from "solid-js/h";

function SignIn(props){

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

  async function btnSignIn(){
    console.log("btnSignIn...")
    try{
      let resp = await fetch("/api/signin",{
        method:"POST",
        headers:{
          "Content-Type":"application/json"
        },
        body:JSON.stringify({
          alias:"testalias",
          passphrase:"testpass",
        })
      });
      let data = await resp.json();
      console.log(data)
    }catch(e){
      console.log(e)
    }
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

  function btnToDoList(){
    console.log("btnForgot...")
    if(typeof props.view == 'function'){
      props.view('todolist')
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
        h("button",{onClick:btnSignIn},"Sign In"),
      ),
    ),
    h("tr",{},
      h("td",{colspan:2},
        h("button",{onClick:btnSignUp},"Sign Up"),
      )
    ),
    h("tr",{},
      h("td",{colspan:2},
        h("button",{onClick:btnForgot},"Forgot"),
        h("button",{onClick:btnToDoList},"ToDoList"),
      )
    )
  ))
}

export default SignIn;