/* tslint:disable */
/* eslint-disable */
// This file was automatically generated and should not be edited.

// ====================================================
// GraphQL mutation operation: ProgressQuery
// ====================================================

export interface ProgressQuery_progress {
  __typename: "Progress";
  id: string;
  pageInProgress: number;
}

export interface ProgressQuery {
  progress: ProgressQuery_progress;
}

export interface ProgressQueryVariables {
  isbn: string;
  pageCount: number;
}
