import React, { useContext, useState } from "react";
import { useQuery } from "@apollo/react-hooks";
import { BookmarksQuery as TBookmarksQuery } from "./queries/__generated__/BookmarksQuery";
import bookmarksQuery from "./queries/bookmarksQuery";
import AddBookmarkPopup from "./AddBookmarkPopup";
import {
  ProgressPopupContext,
  ProgressPopupStatus
} from "./progressPopup/progressPopupContext";
import ProgressPopupAdd from "./progressPopup/ProgressPopupAdd";

const Bookmarks: React.FC = () => {
  const [progressState, setProgressState] = useState<{
    popup: ProgressPopupStatus;
    isbn?: string;
    pageCount?: number;
  }>({
    popup: ProgressPopupStatus.hide,
    isbn: undefined,
    pageCount: undefined
  });
  const progressPopupContext = useContext(ProgressPopupContext);
  const { loading, data, refetch } = useQuery<TBookmarksQuery>(
    bookmarksQuery,
    {}
  );
  if (data === undefined) {
    return <div> no bookmarks</div>;
  }
  function handleClick(e: React.MouseEvent<HTMLLIElement, MouseEvent>) {
    e.preventDefault();
    console.log(e.currentTarget.dataset.bookmarkIsbn);
    progressPopupContext.setStatus(
      ProgressPopupStatus.add,
      e.currentTarget.dataset.bookmarkIsbn,
      parseInt(e.currentTarget.dataset.pageCount || "0")
    );
    setProgressState({
      popup: ProgressPopupStatus.add,
      isbn: e.currentTarget.dataset.bookmarkIsbn,
      pageCount: parseInt(e.currentTarget.dataset.bookmarkPageInProgress || "0")
    });
  }

  refetch();
  return (
    <div>
      <ProgressPopupContext.Provider
        value={{
          status: progressState,
          setStatus: (
            popup: ProgressPopupStatus,
            isbn?: string,
            pageCount?: number
          ) => {
            console.log("set");
            setProgressState({ popup, isbn, pageCount });
          }
        }}
      >
        <ul>
          {data.bookmarks.map(bookmark => (
            <li
              key={bookmark.id}
              onClick={handleClick}
              data-bookmark-isbn={bookmark.isbn.code}
              data-bookmark-page-in-progress={bookmark.pageInProgress}
            >
              {bookmark.title}: {bookmark.pageInProgress}/{bookmark.pageCount}
              ページ
            </li>
          ))}
        </ul>
        {console.log("hoge")}
        {console.log(progressState.popup)}
        {progressState.popup === ProgressPopupStatus.add && (
          <ProgressPopupAdd />
        )}
      </ProgressPopupContext.Provider>
      <AddBookmarkPopup />
    </div>
  );
};

export default Bookmarks;
