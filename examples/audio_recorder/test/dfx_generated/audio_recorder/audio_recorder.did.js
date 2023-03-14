export const idlFactory = ({ IDL }) => {
  const Recording = IDL.Record({
    'id' : IDL.Principal,
    'audio' : IDL.Vec(IDL.Nat8),
    'name' : IDL.Text,
    'created_at' : IDL.Nat64,
    'user_id' : IDL.Principal,
  });
  const KeyTooLarge = IDL.Record({ 'max' : IDL.Nat32, 'given' : IDL.Nat32 });
  const InsertError = IDL.Variant({
    'ValueTooLarge' : KeyTooLarge,
    'KeyTooLarge' : KeyTooLarge,
  });
  const CreateRecordingErr = IDL.Variant({
    'InsertError' : InsertError,
    'UserDoesNotExist' : IDL.Principal,
  });
  const CreateRecordingResult = IDL.Variant({
    'ok' : Recording,
    'err' : CreateRecordingErr,
  });
  const User = IDL.Record({
    'id' : IDL.Principal,
    'recording_ids' : IDL.Vec(IDL.Principal),
    'username' : IDL.Text,
    'created_at' : IDL.Nat64,
  });
  const CreateUserResult = IDL.Variant({ 'ok' : User, 'err' : InsertError });
  const DeleteRecordingError = IDL.Variant({
    'RecordingDoesNotExist' : IDL.Principal,
    'InsertError' : InsertError,
    'UserDoesNotExist' : IDL.Principal,
  });
  const DeleteRecordingResult = IDL.Variant({
    'ok' : Recording,
    'err' : DeleteRecordingError,
  });
  const DeleteUserErr = IDL.Variant({ 'UserDoesNotExist' : IDL.Principal });
  const DeleteUserResult = IDL.Variant({ 'ok' : User, 'err' : DeleteUserErr });
  return IDL.Service({
    'create_recording' : IDL.Func(
        [IDL.Vec(IDL.Nat8), IDL.Text, IDL.Principal],
        [CreateRecordingResult],
        [],
      ),
    'create_user' : IDL.Func([IDL.Text], [CreateUserResult], []),
    'delete_recording' : IDL.Func([IDL.Principal], [DeleteRecordingResult], []),
    'delete_user' : IDL.Func([IDL.Principal], [DeleteUserResult], []),
    'read_recording_by_id' : IDL.Func(
        [IDL.Principal],
        [IDL.Opt(Recording)],
        ['query'],
      ),
    'read_recordings' : IDL.Func([], [IDL.Vec(Recording)], ['query']),
    'read_user_by_id' : IDL.Func([IDL.Principal], [IDL.Opt(User)], ['query']),
    'read_users' : IDL.Func([], [IDL.Vec(User)], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
