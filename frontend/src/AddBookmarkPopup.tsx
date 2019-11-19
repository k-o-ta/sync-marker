import React, { useState } from 'react';
import {
  AddBookmarkPopupContext,
  AddBookmarkPopupStatus
} from './AddBookmarkPopupContext';
import AddBookmarkPopupSearch from './AddBookmarkPopupSearch';
import AddBookmarkPopupAdd from './AddBookmarkPopupAdd';
import AddBookmarkPopupHide from './AddBookmarkPopupHide';

const AddBookmarkPopup: React.FC = () => {
  const [addBookmarkState, setAddBookmarkState] = useState<{
    popup: AddBookmarkPopupStatus;
    isbn?: string;
  }>({ popup: AddBookmarkPopupStatus.hide, isbn: '' });
  let dom = <AddBookmarkPopupHide />;
  switch (addBookmarkState.popup) {
    case AddBookmarkPopupStatus.hide:
      dom = <AddBookmarkPopupHide />;
      console.log('hide');
      break;
    case AddBookmarkPopupStatus.search:
      dom = <AddBookmarkPopupSearch />;
      console.log('search');
      break;
    case AddBookmarkPopupStatus.add:
      dom = <AddBookmarkPopupAdd />;
      console.log('add');
      break;
  }
  return (
    <AddBookmarkPopupContext.Provider
      value={{
        status: addBookmarkState,
        setStatus: (popup: AddBookmarkPopupStatus, isbn?: string) => {
          setAddBookmarkState({ popup, isbn });
        }
      }}
    >
      {dom}
    </AddBookmarkPopupContext.Provider>
  );
};
export default AddBookmarkPopup;
