import React, { useState } from 'react';
import logo from './logo.svg';
import './App.css';
import ApolloClient, { gql } from 'apollo-boost';
import { ApolloProvider, useQuery } from '@apollo/react-hooks';
import Login from './Login';
import CreateUser from './CreateUser';
import Bookmarks from './Bookmarks';
import bookmarksQuery from './queries/bookmarksQuery';
import loggedInQuery from './queries/loggedInQuery';
import { LoggedInQuery as TLoggedInQuery } from './queries/__generated__/LoggedInQuery';
import { LoggedInContext } from './LoggedInContext';
import { useLoggedIn } from './LoggedInHook';
import Signin from './Signin';
import Contents from './Contents';

const client = new ApolloClient({
  uri: 'http://localhost:3000/graphql'
});

client
  .query({
    query: gql`
      query BooFromIsbn {
        bookFromIsbn(isbn: "9784797321944") {
          name
          page
          isbn {
            code
          }
          dataSource
        }
      }
    `
  })
  .then(result => console.log(result));

// export const LoggedInContext = React.createContext(false);
const App: React.FC = () => {
  // const { loading, data } = useQuery<TLoggedInQuery>(
  //     loggedInQuery,
  //     {}
  // );
  // if (loading) return <p>Loading...</p>
  // const [loggedIn, setLoggedInState] = React.useState(false);
  // const loggedIn = useLoggedIn();
  // console.log("loggedIn?", loggedIn.loggedIn);
  const [loggedInState, setLoggedIn] = useState(false);
  return (
    <ApolloProvider client={client}>
      {/*<LoggedInContext.Provider value={{loggedIn: loggedIn, setLoggedIn: (loggedIn: boolean) => {*/}
      {/*    console.log("set!", loggedIn);*/}
      {/*    setLoggedInState(loggedIn)*/}
      {/*  }}}>*/}
      <LoggedInContext.Provider
        value={{ loggedIn: loggedInState, setLoggedIn: setLoggedIn }}
      >
        <Signin></Signin>
        <Contents />
        {/*<Login></Login>*/}
        {/*<CreateUser/>*/}
        <div className="App">
          <header className="App-header">
            <img src={logo} className="App-logo" alt="logo" />
            <p>
              Edit <code>src/App.tsx</code> and save to reload.
            </p>
            <a
              className="App-link"
              href="https://reactjs.org"
              target="_blank"
              rel="noopener noreferrer"
            >
              Learn React
            </a>
          </header>
        </div>
      </LoggedInContext.Provider>
    </ApolloProvider>
  );
  // if (data && data.loggedIn) {
  //   return (
  //     <ApolloProvider client={client}>
  //       <Contents/>
  //       <div className="App">
  //         <header className="App-header">
  //           <img src={logo} className="App-logo" alt="logo" />
  //           <p>
  //             Edit <code>src/App.tsx</code> and save to reload.
  //           </p>
  //           <a
  //               className="App-link"
  //               href="https://reactjs.org"
  //               target="_blank"
  //               rel="noopener noreferrer"
  //           >
  //             Learn React
  //           </a>
  //         </header>
  //       </div>
  //     </ApolloProvider>
  //   );
  // } else {
  //   return (
  //       <ApolloProvider client={client}>
  //         <Login></Login>
  //         <CreateUser/>
  //         <div className="App">
  //           <header className="App-header">
  //             <img src={logo} className="App-logo" alt="logo" />
  //             <p>
  //               Edit <code>src/App.tsx</code> and save to reload.
  //             </p>
  //             <a
  //                 className="App-link"
  //                 href="https://reactjs.org"
  //                 target="_blank"
  //                 rel="noopener noreferrer"
  //             >
  //               Learn React
  //             </a>
  //           </header>
  //         </div>
  //       </ApolloProvider>
  //   );
  // }
};

export default App;
