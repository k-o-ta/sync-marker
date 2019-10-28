/* tslint:disable */
/* eslint-disable */
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: BookmarksQuery
// ====================================================

export interface BookmarksQuery_bookmarks_isbn {
  __typename: "Isbn";
  code: string;
}

export interface BookmarksQuery_bookmarks {
  __typename: "Bookmark";
  title: string;
  pageCount: number;
  isbn: BookmarksQuery_bookmarks_isbn;
  pageInProgress: number;
}

export interface BookmarksQuery {
  bookmarks: BookmarksQuery_bookmarks[];
}
