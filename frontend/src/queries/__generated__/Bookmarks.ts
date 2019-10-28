/* tslint:disable */
/* eslint-disable */
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: Contents
// ====================================================

export interface Bookmarks_bookmarks_isbn {
  __typename: "Isbn";
  code: string;
}

export interface Bookmarks_bookmarks {
  __typename: "Bookmark";
  title: string;
  pageCount: number;
  isbn: Bookmarks_bookmarks_isbn;
  pageInProgress: number;
}

export interface Bookmarks {
  bookmarks: Bookmarks_bookmarks[];
}
