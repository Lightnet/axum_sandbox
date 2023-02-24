// 

import { Link, useNavigate } from "@solidjs/router";
import { createSignal } from "solid-js";


export default function PageSignUp(){

  const [alias, setAlias] = createSignal('guest');
  const [passphrase, setPassphrase] = createSignal('password');
  const [email, setEmail] = createSignal('email');

  const navigate = useNavigate();

  async function btnSignup(){
    try {
      let resp = await fetch('/api/signup',{
        method:'POST',
        headers:{
          'Content-Type': 'application/json'
        },
        body:JSON.stringify({
          alias:alias(),
          passphrase:passphrase(),
          email:email()
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

  return (<>
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
            <td><label>Email:</label></td>
            <td><input value={email()} onInput={(e)=>setEmail(e.target.value)} /></td>
          </tr>

          <tr>
            <td colSpan="2">
              <button style="width:100%;" onClick={btnSignup}>Register</button>
            </td>
          </tr>

          <tr>
            <td colSpan="2">
            <button style="width:100%;" onClick={btnCancel}>Cancel</button>
            </td>
          </tr>

        </tbody>
      </table>
    </div>
  </>)
}