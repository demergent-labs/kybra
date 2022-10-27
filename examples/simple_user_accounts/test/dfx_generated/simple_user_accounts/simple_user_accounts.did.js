export const idlFactory = ({ IDL }) => {
  const User = IDL.Record({ 'id' : IDL.Text, 'username' : IDL.Text });
  return IDL.Service({
    'create_user' : IDL.Func([IDL.Text], [User], []),
    'get_user_by_id' : IDL.Func([IDL.Text], [User], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };