import React, { useContext } from 'react';
import { useMutation } from '@apollo/react-hooks';
import { gql } from 'apollo-boost';
import Login from './Login';
import CreateUser from './CreateUser';
import { useLoggedIn } from './LoggedInHook';
import { LoggedInContext } from './LoggedInContext';

interface Prop {
  loggedIn: boolean;
}
const Signin: React.FC<Prop> = prop => {
  // const {loggedIn} = useLoggedIn();
  const { loggedIn } = useContext(LoggedInContext);
  return (
    <div>
      {console.log('in sign in', loggedIn)}
      {/*{loggedIn ? (<div>true</div>) : <Login></Login>}*/}
      {!prop.loggedIn && !loggedIn && <Login></Login>}
      {!prop.loggedIn && !loggedIn && <CreateUser />}
    </div>
  );
};

export default Signin;
