import { gql } from 'apollo-boost';

export const CreateBookQuery = gql`
  mutation CreateBookQuery($isbn: String!, $pageCount: Int!, $title: String!) {
    createBook(isbn: $isbn, pageCount: $pageCount, title: $title)
  }
`;
