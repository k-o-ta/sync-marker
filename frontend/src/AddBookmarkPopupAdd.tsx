import React, {useContext} from 'react';
import {BookFromIsbnQuery as TBookFromIsbnQuery} from './queries/__generated__/BookFromIsbnQuery';
import searchBookFromIsbnQuery from "./queries/searchBookFromIsbnQuery";
import {
  AddBookmarkPopupContext,
  AddBookmarkPopupStatus
} from "./AddBookmarkPopupContext";
import {useQuery} from "@apollo/react-hooks";
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
    marginLeft: '-150px',
    marginTop: '-100px',
    backgroundColor: 'white',
    borderRadius: '5px',
    textAlign: 'center',
  };
  const { loading, error, data } = useQuery<TBookFromIsbnQuery>(
      searchBookFromIsbnQuery,
      {
        variables: {isbn: addBookmarkPopupContext.status.isbn!}
      }
  );

  if (loading) return (<div>'Loading...'</div>);
  if (error) return (<div>`Error! ${error.message}`</div>);

  return (
      <div style={popupStyle}>
        {data && (
            <form onSubmit={handleSubmit}>
              <label>
                title:
                <input type="text" value={data.bookFromIsbn.name}/>
              </label>
              <label>
                page:
                <input type="text" value={data.bookFromIsbn.page}/>
              </label>
              <button type="submit">Add</button>
            </form>
        )}
      </div>
  )
}
export default AddBookmarkPopupAdd
