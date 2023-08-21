export const idlFactory = ({ IDL }) => {
  return IDL.Service({ 'randomBytes' : IDL.Func([], [IDL.Text], []) });
};
export const init = ({ IDL }) => { return []; };
