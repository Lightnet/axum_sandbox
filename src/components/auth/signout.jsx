// 

import { Link, useNavigate } from "@solidjs/router";
import { createSignal } from "solid-js";


export default function PageSignOut(){

  const [alias, setAlias] = createSignal('guest');
  const [passphrase, setPassphrase] = createSignal('password');

  const navigate = useNavigate();

  function btnSignOut(){

  }

  function btnCancel(){
    navigate("/", { replace: true });
  }

  return (<>
    <div>
      <p>Are you sure to Sign Out?</p>
      <button onClick={btnSignOut}>Logout</button>
      <button onClick={btnCancel}>Cancel</button>
    </div>
  </>)
}