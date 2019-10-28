/* tslint:disable */
/* eslint-disable */
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL query operation: BooFromIsbn
// ====================================================

export interface BooFromIsbn_bookFromIsbn_isbn {
  __typename: "Isbn";
  code: string;
}

export interface BooFromIsbn_bookFromIsbn {
  __typename: "Book";
  name: string;
  page: number;
  isbn: BooFromIsbn_bookFromIsbn_isbn;
  dataSource: string;
}

export interface BooFromIsbn {
  bookFromIsbn: BooFromIsbn_bookFromIsbn;
}
