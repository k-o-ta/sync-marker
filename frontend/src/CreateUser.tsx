import React from 'react';
import { useMutation } from '@apollo/react-hooks';
import { gql } from 'apollo-boost';

const CREATE = gql`
  mutation CreateUser($email: String!, $password: String!) {
    createUser(email: $email, password: $password)
  }
`;

const CreateUser: React.FC = () => {
  let email: React.RefObject<HTMLInputElement> = React.createRef();
  let password: React.RefObject<HTMLInputElement> = React.createRef();

  const [create, { data }] = useMutation(CREATE);
  function handleSubmit(e: any) {
    e.preventDefault();
    create({
      variables: {
        email: email.current != null ? email.current.value : '',
        password: password.current != null ? password.current.value : ''
      }
    });
    console.log('login submit');
  }
  return (
    <div>
      this is create user
      <form onSubmit={handleSubmit}>
        <label>
          id:
          <input ref={email} type="text" />
        </label>
        <label>
          pass:
          <input ref={password} type="text" />
        </label>
        <button type="submit">Create User</button>
      </form>
    </div>
  );
};

export default CreateUser;
