import React from 'react';

export interface IAddBookmarkPopupContext {
  status: { popup: AddBookmarkPopupStatus; isbn?: string };
  setStatus: (popup: AddBookmarkPopupStatus, isbn?: string) => void;
}

export enum AddBookmarkPopupStatus {
  hide,
  search,
  add
}

export const AddBookmarkPopupContextDefaultValue = {
  status: { popup: AddBookmarkPopupStatus.hide, isbn: undefined },
  setStatus: (popup: AddBookmarkPopupStatus, isbn?: string) => {}
};

export const AddBookmarkPopupContext = React.createContext<
  IAddBookmarkPopupContext
>(AddBookmarkPopupContextDefaultValue);
