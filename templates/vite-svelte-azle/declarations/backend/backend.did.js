export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'add' : IDL.Func([IDL.Nat], [IDL.Nat], []),
    'get' : IDL.Func([], [IDL.Nat], ['query']),
    'inc' : IDL.Func([], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
