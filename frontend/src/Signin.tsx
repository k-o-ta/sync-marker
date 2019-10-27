import React from 'react';
import { useMutation } from '@apollo/react-hooks';
import {gql} from "apollo-boost";
import Login from "./Login";
import CreateUser from "./CreateUser";


const Signin: React.FC = () => {

  return (
      <div>
        <Login/>
        <CreateUser/>
      </div>
  );
}

export default Signin;