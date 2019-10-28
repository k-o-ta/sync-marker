import React from 'react';
import logo from './logo.svg';
import './App.css';
import ApolloClient, {gql} from 'apollo-boost';
import {ApolloProvider, useQuery} from '@apollo/react-hooks';
import Login from "./Login";
import CreateUser from "./CreateUser";
import Bookmarks from "./Bookmarks";
import bookmarksQuery from './queries/bookmarksQuery';
import loggedInQuery from './queries/loggedInQuery';
import {LoggedInQuery as TLoggedInQuery} from './queries/__generated__/LoggedInQuery';

const client = new ApolloClient({
  uri: 'http://localhost:3000/graphql',
});

client
.query({
  query: gql`
      query BooFromIsbn {
        bookFromIsbn(isbn: "9784797321944"){
          name
          page
          isbn{
            code
          }
          dataSource
        }
      }
    `
})
.then(result => console.log(result));

export const LoggedInContext = React.createContext(false);
const App: React.FC = () => {
  // const { loading, data } = useQuery<TLoggedInQuery>(
  //     loggedInQuery,
  //     {}
  // );
  // if (loading) return <p>Loading...</p>
  return (
            <ApolloProvider client={client}>
              <LoggedInContext.Provider value={false}>
                <Login></Login>
                <CreateUser/>
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
}

export default App;
