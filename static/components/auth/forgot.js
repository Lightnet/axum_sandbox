
import {
  createSignal,
  onCleanup,
} from "solid-js";
import h from "solid-js/h";

function Forgot(props){

  const [alias, setAlias] = createSignal("test");
  const [email, setEmail] = createSignal("test");

  function InputAlias(e){
    setAlias(e.target.value);
  }

  function InputEmail(e){
    setEmail(e.target.value);
  }

  async function btnRecovery(){
    console.log("btnRecovery...")
    try{
      let resp = await fetch("/api/forgot",{
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

  function btnBack(){
    console.log("btnBack...")
    if(typeof props.view == 'function'){
      props.view('signin')
    }
  }

  return h("table",{}, h("tbody",{},
    h("tr",{},
      h("td",{colspan:2},
        h("label",{},"Forgot"),
      ),
    ),
    h("tr",{},
      h("td",{},h("label",{},"Alias:")),
      h("td",{},h("input",{value:alias(),onInput:InputAlias})),
    ),
    h("tr",{},
      h("td",{},h("label",{},"E-Mail:")),
      h("td",{},h("input",{value:email(),onInput:InputEmail})),
    ),
    h("tr",{},
      h("td",{colspan:2},
        h("button",{onClick:btnBack},"Back"),
        h("button",{onClick:btnRecovery},"Recovery"),
      ),
    ),
  ))
}

export default Forgot;