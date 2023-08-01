export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'add' : IDL.Func([IDL.Nat], [IDL.Nat], []),
    'get' : IDL.Func([], [IDL.Nat], ['query']),
    'inc' : IDL.Func([], [IDL.Nat], []),
  });
};
export const init = ({ IDL }) => { return []; };
