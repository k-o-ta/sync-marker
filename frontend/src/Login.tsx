import React, {useContext} from 'react';
import {useMutation} from '@apollo/react-hooks';
import {gql} from "apollo-boost";
import {useLoggedIn} from "./LoggedInHook";
import {LoggedInContext} from "./LoggedInContext";

const LOGIN = gql`
  mutation Login{
    login(email: "foo@example.com", password: "123abcdef")
  }
`;


const Login: React.FC = () => {
  const loggedIn = useContext(LoggedInContext);

  const [login, {data}] = useMutation(LOGIN, {
    onCompleted: (data: boolean) => {
      console.log("login done!", data);
      loggedIn.setLoggedIn(data);
    }
  });

  function handleSubmit(e: any) {
    e.preventDefault();
    login();
    console.log("login submit");
  }

  return (
      <div>
        this is login form
        <form onSubmit={handleSubmit}>
          <label>
            id:
            <input type="text"/>
          </label>
          <label>
            pass:
            <input type="text"/>
          </label>
          <button type="submit">Login</button>
        </form>
      </div>
  )
}

export default Login;