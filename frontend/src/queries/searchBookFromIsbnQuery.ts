import { gql } from 'apollo-boost';

export default gql`
  query BookFromIsbnQuery($isbn: String!) {
    bookFromIsbn(isbn: $isbn) {
      name
      page
      isbn {
        code
      }
      dataSource
    }
  }
`;
