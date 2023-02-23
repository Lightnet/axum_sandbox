import { createSignal, createContext, useContext } from "solid-js";


const AuthContext = createContext();

export default function AuthProvider(props) {

  const [count, setCount] = createSignal(props.count || 0);
  const [token, setToken] = createSignal('');
  const [isLogin, setIsLogin] = createSignal(false);

  const value = {
    token, setToken,
    isLogin, setIsLogin
  }

  return (
    <AuthContext.Provider value={value}>
      {props.children}
    </AuthContext.Provider>
  );

}
