import React, {useContext, useState} from 'react';
import {BookFromIsbnQuery as TBookFromIsbnQuery} from './queries/__generated__/BookFromIsbnQuery';
import searchBookFromIsbnQuery from "./queries/searchBookFromIsbnQuery";
import {
  AddBookmarkPopupContext,
  AddBookmarkPopupStatus
} from "./AddBookmarkPopupContext";
import {useMutation, useQuery} from "@apollo/react-hooks";
import {ProgressQuery} from "./queries/__generated__/ProgressQuery";
import {progressQuery} from "./queries/bookmarksQuery";
import {CreateBookQuery as TCreateBookQuery} from "./queries/__generated__/CreateBookQuery";
import {CreateBookQuery} from "./queries/bookQuery";
// import {CreateBookQuery} from "./queries/bookQuery";
const AddBookmarkPopupAdd: React.FC = () => {
  async function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    if (book && book.datasource == "network") {
      await createBook( {variables: {isbn: book.isbn || "", pageCount: book.pageCount || 0, title: book.title || ""}});
    }
    await progress({variables: { isbn: book.isbn || "", pageCount: 0}});
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
  const [book, _] = useState({isbn: data && data.bookFromIsbn.isbn.code, pageCount: data && data.bookFromIsbn.page, title: data && data.bookFromIsbn.name, datasource: data && data.bookFromIsbn.dataSource});
  const [ createBook, {error: createBookError, data: createBookData} ] = useMutation<TCreateBookQuery>(CreateBookQuery);
  const [ progress, {error: progressError, data: progressData} ] = useMutation<ProgressQuery>(progressQuery,
      {
      onCompleted: (data: ProgressQuery) => {
        console.log(data.progress);
    }
  }
  );
  if (data === undefined) {
    return (<div></div>)
  }

  if (loading) return (<div>'Loading...'</div>);
  if (error) return (<div>`Error! ${error.message}`</div>);

  return (
      <div style={popupStyle}>
            <form onSubmit={handleSubmit}>
              <label>
                title:
                <input type="text" value={data.bookFromIsbn.name} readOnly/>
              </label>
              <label>
                page:
                <input type="text" value={data.bookFromIsbn.page} readOnly/>
              </label>
              <button type="submit">Add</button>
            </form>
      </div>
  )
}
export default AddBookmarkPopupAdd
