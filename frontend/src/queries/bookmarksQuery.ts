import {gql} from 'apollo-boost';

export default gql`
  query BookmarksQuery{
    bookmarks{
      title
      pageCount
      isbn{
        code
      }
      pageInProgress
    }
  }
`