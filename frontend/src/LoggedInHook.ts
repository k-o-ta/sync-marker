import React from 'react';
import { ILoggedInContext } from './LoggedInContext';

export const useLoggedIn = (): ILoggedInContext => {
  const [loggedIn, setLoggedInState] = React.useState(false);

  const setLoggedIn = (loggedIn: boolean): void => {
    console.log('logged in?', loggedIn);
    setLoggedInState(loggedIn);
  };
  return {
    loggedIn,
    setLoggedIn
  };
};
