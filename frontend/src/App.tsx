import React from 'react';
import logo from './logo.svg';
import './App.css';
import ApolloClient, {gql} from 'apollo-boost';
import { ApolloProvider } from '@apollo/react-hooks';
import Login from "./Login";
import CreateUser from "./CreateUser";

const client = new ApolloClient({
  uri: 'http://localhost:3000/graphql',
});

client
.query({
  query: gql`
      {
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

const App: React.FC = () => {
  return (
    <ApolloProvider client={client}>
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
    </ApolloProvider>
  );
}

export default App;
