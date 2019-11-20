import React, { useContext } from 'react';
import {
  AddBookmarkPopupContext,
  AddBookmarkPopupStatus
} from './AddBookmarkPopupContext';
const AddBookmarkPopupHide: React.FC = () => {
  const addBookmarkPopupContext = useContext(AddBookmarkPopupContext);
  function handleClick(e: any) {
    e.preventDefault();
    console.log('search pop up');
    addBookmarkPopupContext.setStatus(AddBookmarkPopupStatus.search);
  }
  return <button onClick={handleClick}>AddBookmark</button>;
};
export default AddBookmarkPopupHide;
