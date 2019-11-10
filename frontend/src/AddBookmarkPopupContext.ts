import React from 'react';

export interface IAddBookmarkPopupContext {
  status: AddBookmarkPopupStatus;
  setStatus: (status: AddBookmarkPopupStatus) => void;
}

export enum AddBookmarkPopupStatus {
  hide,
  search,
  add
}

export const AddBookmarkPopupContextDefaultValue = {
  status: AddBookmarkPopupStatus.hide,
  setStatus: (status: AddBookmarkPopupStatus) => {},
}

export const AddBookmarkPopupContext = React.createContext<IAddBookmarkPopupContext>(AddBookmarkPopupContextDefaultValue);