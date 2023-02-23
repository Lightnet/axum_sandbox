import { Link } from "@solidjs/router";

export default function PageHome(){

  return (<div>
    <label>Home</label>
    <div>
      <Link href="/"> Home </Link> <span> | </span>
      <Link href="/account"> Account </Link> <span> | </span>
      <Link href="/signin"> Sign In</Link> <span> | </span>
      <Link href="/signup"> Sign Up</Link> <span> | </span>
      <Link href="/task"> Task </Link> <span> | </span>
    </div>
    </div>);
}