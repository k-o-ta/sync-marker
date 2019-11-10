import React, {useContext} from 'react';
import {AddBookmarkPopupContext, AddBookmarkPopupStatus} from "./AddBookmarkPopupContext";
const AddBookmarkPopupAdd: React.FC = () => {
  function handleSubmit(e: any) {
    e.preventDefault();
    addBookmarkPopupContext.setStatus(AddBookmarkPopupStatus.hide);
  }
  const addBookmarkPopupContext = useContext(AddBookmarkPopupContext);
  const popupStyle: { [key: string]: string } = {
    left: '50%',
    top: '50%',
    width: '300px',
    height: '200px',
    position: "fixed",
    'margin-left': '-150px',
    'margin-top': '-100px',
    'background-color': 'white',
    'border-radius': '5px',
    'text-align': 'center',
  };
  return (
      <div style={popupStyle}>
        <form onSubmit={handleSubmit}>
          <label>
            title:
            <input type="text"/>
          </label>
          <label>
            page:
            <input type="text"/>
          </label>
          <button type="submit">Add</button>
        </form>
      </div>
  )
}
export default AddBookmarkPopupAdd
