import { gql } from 'apollo-boost';

export default gql`
  query BookmarksQuery {
    bookmarks {
      id
      title
      pageCount
      isbn {
        code
      }
      pageInProgress
    }
  }
`;

export const progressQuery = gql`
  mutation ProgressQuery($isbn: String!, $pageCount: Int!) {
    progress(isbn: $isbn, pageCount: $pageCount) {
      id
      pageInProgress
    }
  }
`;
