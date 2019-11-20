import React, { useState } from 'react';
import {
  ProgressPopupContext,
  ProgressPopupStatus
} from './progressPopupContext';
import AddBookmarkPopupHide from '../AddBookmarkPopupHide';
import { AddBookmarkPopupStatus } from '../AddBookmarkPopupContext';
import AddBookmarkPopup from '../AddBookmarkPopup';
// import AddBookmarkPopupSearch from "./AddBookmarkPopupSearch";
import ProgressPopupAdd from './ProgressPopupAdd';
// import AddBookmarkPopupHide from "./AddBookmarkPopupHide";

const ProgressPopup: React.FC = () => {
  const [progressState, setProgressState] = useState<{
    popup: ProgressPopupStatus;
    isbn?: string;
    pageCount?: number;
  }>({
    popup: ProgressPopupStatus.hide,
    isbn: undefined,
    pageCount: undefined
  });
  let dom = <AddBookmarkPopupHide />;
  switch (progressState.popup) {
    case ProgressPopupStatus.hide:
      // dom = <AddBookmarkPopupHide/>;
      console.log('hide');
      break;
    case ProgressPopupStatus.add:
      dom = <ProgressPopupAdd />;
      console.log('add');
      break;
  }
  return (
    <ProgressPopupContext.Provider
      value={{
        status: progressState,
        setStatus: (
          popup: ProgressPopupStatus,
          isbn?: string,
          pageCount?: number
        ) => {
          setProgressState({ popup, isbn, pageCount });
        }
      }}
    >
      {dom}
    </ProgressPopupContext.Provider>
  );
};
export default AddBookmarkPopup;
