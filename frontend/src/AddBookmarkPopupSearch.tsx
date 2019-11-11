import React, {useContext, useState} from 'react';
import {
  AddBookmarkPopupContext,
  AddBookmarkPopupStatus
} from "./AddBookmarkPopupContext";

const AddBookmarkPopupSearch: React.FC = () => {
  const addBookmarkPopupContext = useContext(AddBookmarkPopupContext);
  const [formIsbn, setIsbn] = useState("");
  function handleChange(e:  React.ChangeEvent<HTMLInputElement>) {
    setIsbn(e.target.value);
  }
  function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    addBookmarkPopupContext.setStatus(AddBookmarkPopupStatus.add, formIsbn);
  }

  const popupStyle: { [key: string]: string } = {
    left: '50%',
    top: '50%',
    width: '300px',
    height: '200px',
    position: "fixed",
    marginLeft: '-150px',
    marginTop: '-100px',
    backgroundColor: 'white',
    borderRadius: '5px',
    textAlign: 'center',
  };
  return (
      <div style={popupStyle}>
        <form onSubmit={handleSubmit}>
          <label>
            ISBN13:
            <input type="text" value={formIsbn} onChange={handleChange} />
          </label>
          <button type="submit">Search</button>
        </form>
      </div>
  )
}
export default AddBookmarkPopupSearch
