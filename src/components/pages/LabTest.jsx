//

import { Link, useNavigate } from "@solidjs/router";
import { createSignal } from "solid-js";

export default function PageLabTest(){

  const navigate = useNavigate();

  async function btnGetCookie(){
    let resp = await fetch('/api/getcookie');
  }

  return (<>
    <div>
      <Link href="/"> Home </Link>
      <button onClick={btnGetCookie}> get cookies </button>
    </div>
  </>)
}