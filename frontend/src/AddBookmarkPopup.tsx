import React, {useContext, useState} from 'react';
import {AddBookmarkPopupContext, AddBookmarkPopupStatus} from "./AddBookmarkPopupContext";
import AddBookmarkPopupSearch from "./AddBookmarkPopupSearch";
import AddBookmarkPopupAdd from "./AddBookmarkPopupAdd";
import AddBookmarkPopupHide from "./AddBookmarkPopupHide";

const AddBookmarkPopup: React.FC = () => {
  const [addBookmarkState, setAddBookmarkState] = useState(AddBookmarkPopupStatus.hide);
  // const addBookmarkPopupContext = useContext(AddBookmarkPopupContext);
  let dom = (<AddBookmarkPopupHide/>);
  switch (addBookmarkState) {
    case AddBookmarkPopupStatus.hide:
      dom = <AddBookmarkPopupHide/>;
      console.log("hide");
      break;
    case AddBookmarkPopupStatus.search:
      dom = <AddBookmarkPopupSearch/>;
      console.log("search");
      break;
    case AddBookmarkPopupStatus.add:
      dom = <AddBookmarkPopupAdd/>;
      console.log("add");
      break;
  }
  return (
      <AddBookmarkPopupContext.Provider value={{status: addBookmarkState, setStatus: setAddBookmarkState}}>
        {dom}
      </AddBookmarkPopupContext.Provider>
  )
}
export default AddBookmarkPopup;
