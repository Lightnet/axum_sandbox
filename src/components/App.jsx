// 

// 

//import PageTask from "./task/task";

import { Routes, Route } from "@solidjs/router"
import AuthProvider from "./auth/AuthProvider";

import PageHome from "./pages/Home";
import PageSignIn from "./auth/signin";
import PageSignUp from "./auth/signup";
import PageTask from "./task/task";
import PageAccount from "./pages/Account";

function App() {

  return (<AuthProvider>
    <Routes>
      <Route path="/" component={PageHome} />
      <Route path="/account" component={PageAccount} />
      <Route path="/signin" component={PageSignIn} />
      <Route path="/signup" component={PageSignUp} />
      <Route path="/task" component={PageTask} />
    </Routes>
  </AuthProvider>);

  /*
  return (
    <div >    
      <p>
        Edit <code>src/App.jsx</code> and save to reload.
      </p>
      <p> Test Rust, solid-js, npm and others. </p>
      <PageTask/>
    </div>
  );
  */
}

export default App;