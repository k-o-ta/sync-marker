/* tslint:disable */
/* eslint-disable */
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: BookFromIsbnQuery
// ====================================================

export interface BookFromIsbnQuery_bookFromIsbn_isbn {
  __typename: "Isbn";
  code: string;
}

export interface BookFromIsbnQuery_bookFromIsbn {
  __typename: "Book";
  name: string;
  page: number;
  isbn: BookFromIsbnQuery_bookFromIsbn_isbn;
  dataSource: string;
}

export interface BookFromIsbnQuery {
  bookFromIsbn: BookFromIsbnQuery_bookFromIsbn;
}

export interface BookFromIsbnQueryVariables {
  isbn: string;
}
