import React from 'react';
import { useMutation } from '@apollo/react-hooks';
import {gql} from "apollo-boost";

const LOGIN = gql`
  mutation {
    login(email: "foo@example.com", password: "123abcdef")
  }
`;


const Login: React.FC = () => {

  const [ login, { data } ] = useMutation(LOGIN);
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
            <input type="text" />
          </label>
          <label>
            pass:
            <input type="text" />
          </label>
          <button type="submit">Login</button>
        </form>
      </div>
  )
}

export default Login;