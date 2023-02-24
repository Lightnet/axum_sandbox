// 

import { Link, useNavigate } from "@solidjs/router";
import { createSignal } from "solid-js";

export default function PageSignIn(){

  const [alias, setAlias] = createSignal('guest');
  const [passphrase, setPassphrase] = createSignal('password');

  const navigate = useNavigate();

  async function btnLogin(){
    try {
      let resp = await fetch('/api/signin',{
        method:'POST',
        headers:{
          'Content-Type': 'application/json'
        },
        body:JSON.stringify({
          alias:alias(),
          passphrase:passphrase()
        })
      })
  
      let data = await resp.json();
      console.log(data);  
    } catch (error) {
      console.log("Error:",error.message)
    }
  }

  function btnCancel(){
    navigate("/", { replace: true });
  }

  function btnSignup(){
    navigate("/signup", { replace: true });
  }

  function btnForgot(){
    navigate("/forgot", { replace: true });
  }

  return <>
    <div>
      <table>
        <Link href="/"> Home </Link>
        <tbody>
          <tr>
            <td colSpan="2">Sign In</td>
          </tr>

          <tr>
            <td><label>Alias:</label></td>
            <td><input value={alias()} onInput={(e)=>setAlias(e.target.value)} /></td>
          </tr>
          <tr>
            <td><label>Passphrase:</label></td>
            <td><input value={passphrase()} onInput={(e)=>setPassphrase(e.target.value)} /></td>
          </tr>

          <tr>
            <td colSpan="2">
              <button style="width:100%;" onClick={btnLogin}>Login</button>
            </td>
          </tr>

          <tr>
            <td colSpan="2">
            <button style="width:100%;" onClick={btnCancel}>Cancel</button>
            </td>
          </tr>

          <tr>
            <td colSpan="2">
              <button style="width:100%;" onClick={btnSignup}>Sign Up</button>
            </td>
          </tr>

          <tr>
            <td colSpan="2">
              <button style="width:100%;" onClick={btnForgot}>Forgot</button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </>
}