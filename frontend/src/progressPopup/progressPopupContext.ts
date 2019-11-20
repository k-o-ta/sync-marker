import React from 'react';

export interface IProgressPopupContext {
  status: { popup: ProgressPopupStatus; isbn?: string; pageCount?: number };
  setStatus: (
    popup: ProgressPopupStatus,
    isbn?: string,
    pageCount?: number
  ) => void;
}

export enum ProgressPopupStatus {
  hide,
  add
}

export const ProgressPopupContextDefaultValue = {
  status: {
    popup: ProgressPopupStatus.hide,
    isbn: undefined,
    pageCount: undefined
  },
  setStatus: (
    popup: ProgressPopupStatus,
    isbn?: string,
    pageCount?: number
  ) => {}
};

export const ProgressPopupContext = React.createContext<IProgressPopupContext>(
  ProgressPopupContextDefaultValue
);
