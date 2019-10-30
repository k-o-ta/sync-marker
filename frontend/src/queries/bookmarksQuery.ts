import {gql} from 'apollo-boost';

export default gql`
  query BookmarksQuery{
    bookmarks{
      id
      title
      pageCount
      isbn{
        code
      }
      pageInProgress
    }
  }
`