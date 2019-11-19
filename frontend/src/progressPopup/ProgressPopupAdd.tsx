import React, { useContext, useState } from 'react';
// import searchBookFromIsbnQuery from "./queries/searchBookFromIsbnQuery";
import { useMutation, useQuery } from '@apollo/react-hooks';
import { ProgressQuery } from '../queries/__generated__/ProgressQuery';
import { progressQuery } from '../queries/bookmarksQuery';
// import {CreateBookQuery as TCreateBookQuery} from "./queries/__generated__/CreateBookQuery";
// import {CreateBookQuery} from "./queries/bookQuery";
import {
  ProgressPopupContext,
  ProgressPopupStatus
} from './progressPopupContext';
// import {CreateBookQuery} from "./queries/bookQuery";
const ProgressPopupAdd: React.FC = () => {
  async function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    await progress({
      variables: {
        isbn: progressPopupContext.status.isbn || '',
        pageCount: pageCount
      }
    });
    progressPopupContext.setStatus(
      ProgressPopupStatus.hide,
      progressPopupContext.status.isbn,
      pageCount
    );
  }
  const progressPopupContext = useContext(ProgressPopupContext);
  const [pageCount, setPageCount] = useState<number | undefined>(
    progressPopupContext.status.pageCount
  );
  function handleChange(e: React.ChangeEvent<HTMLInputElement>) {
    e.preventDefault();
    console.log();
    const pageCount = parseInt(e.target.value);
    if (!isNaN(pageCount)) {
      setPageCount(pageCount);
    } else if (e.target.value === '') {
      setPageCount(undefined);
    }
  }
  const popupStyle: { [key: string]: string } = {
    left: '50%',
    top: '50%',
    width: '300px',
    height: '200px',
    position: 'fixed',
    marginLeft: '-150px',
    marginTop: '-100px',
    backgroundColor: 'white',
    borderRadius: '5px',
    textAlign: 'center'
  };
  // const { loading, error, data } = useQuery<TBookFromIsbnQuery>(
  //     searchBookFromIsbnQuery,
  //     {
  //       variables: {isbn: progressPopupContext.status.isbn!}
  //     }
  // );
  // const [book, _] = useState({isbn: data && data.bookFromIsbn.isbn.code, pageCount: data && data.bookFromIsbn.page, title: data && data.bookFromIsbn.name, datasource: data && data.bookFromIsbn.dataSource});
  // const [ createBook, {error: createBookError, data: createBookData} ] = useMutation<TCreateBookQuery>(CreateBookQuery);
  const [progress, { error: progressError, data: progressData }] = useMutation<
    ProgressQuery
  >(progressQuery, {
    onCompleted: (data: ProgressQuery) => {
      console.log(data.progress);
    }
  });
  // if (data === undefined) {
  // return (<div></div>)
  // }

  // if (loading) return (<div>'Loading...'</div>);
  // if (error) return (<div>`Error! ${error.message}`</div>);

  console.log('pageCount');
  console.log(pageCount);
  return (
    <div style={popupStyle}>
      <form onSubmit={handleSubmit}>
        <label>
          progress:
          <input type="text" value={pageCount || ''} onChange={handleChange} />
        </label>
        <button type="submit">Add</button>
      </form>
    </div>
  );
};
export default ProgressPopupAdd;
