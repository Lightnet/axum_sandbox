//

import { Link, useNavigate } from "@solidjs/router";
import { createSignal } from "solid-js";

export default function PageAccount(){

  const navigate = useNavigate();

  return (<>
    <div>
      <Link href="/"> Home </Link>

    </div>
  </>)
}