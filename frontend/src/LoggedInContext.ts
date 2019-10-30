import React from 'react';

export interface ILoggedInContext {
  loggedIn: boolean;
  setLoggedIn: (loggedIn: boolean) => void;
}

export const LOGGED_IN_DEFAULT_VALUE = {
  loggedIn: false,
  setLoggedIn: (loggedIn: boolean) => {},
}

export const LoggedInContext = React.createContext<ILoggedInContext>(LOGGED_IN_DEFAULT_VALUE);