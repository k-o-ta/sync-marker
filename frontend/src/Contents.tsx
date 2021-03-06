import React, { useContext, useState } from 'react';
import { useMutation, useQuery } from '@apollo/react-hooks';
import { gql } from 'apollo-boost';
import { BookmarksQuery as TBookmarksQuery } from './queries/__generated__/BookmarksQuery';
import bookmarksQuery from './queries/bookmarksQuery';
import { LoggedInQuery as TLoggedInQuery } from './queries/__generated__/LoggedInQuery';
import Login from './Login';
import CreateUser from './CreateUser';
import loggedInQuery from './queries/loggedInQuery';
import Bookmarks from './Bookmarks';
import { LoggedInContext } from './LoggedInContext';
// import {LoggedInContext} from "./App";
//
//
interface Prop {
  loggedIn: boolean;
}
const Contents: React.FC<Prop> = prop => {
  const { loggedIn } = useContext(LoggedInContext);
  //   // const [loggedIn, setLoggedIn] = useState(false);
  //   const { loading, data } = useQuery<TLoggedInQuery>(
  //       loggedInQuery,
  //       {}
  //   );
  //
  //   if (loading) return <p>Loading...</p>
  //
  return (
    <div>
      {console.log('contents: ', loggedIn)}
      {(prop.loggedIn || loggedIn) && <Bookmarks />}
    </div>
  );
};

export default Contents;
