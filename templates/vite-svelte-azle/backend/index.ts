import { $query, Principal, Record, ic, Vec, $update, Opt} from 'azle';
//types
type DAO_DB = Record<{ id: string; username: string }>;
type Proposal = Record<{ question: string; options: Vec<string> }>; // can i specify size here
type Votes = Record<{
  game: string;
  noOfVotes: number;
}>;


// variables
const daoMembers = new Map<Principal, string>();
const currentGame = 'puzzle';
const proposal: Proposal = {
  question: 'What game should we develop next?',
  options: ['Puzzle game', 'Open world', 'RPG'],
};

const votes: Votes[] = proposal.options.map((game) => ({ game, noOfVotes: 0 }));

// functions
$query;
export function whoami(): Principal {
  return ic.caller();
}

$update;
export function setDaoMember(member: DAO_DB): void {
    const id = ic.caller();
    let optMember = daoMembers.get(id);
    if(Opt.Some(optMember)){
        return;
    } else {
        daoMembers.set(id, member.username);
    }
}

$query;
export function getMembers(principal: string): string {
  console.log(Principal.fromText(principal));
  const response = daoMembers.has(Principal.fromText(principal))
    ? daoMembers.get(Principal.fromText(principal))
    : 'data unavailable';
  return response!;
}

$update;
export function updateDaoUserName(username: string): string {
  const caller = whoami();
  if (!daoMembers.has(caller)) {
    return 'unauthorized';
  }
  daoMembers.set(caller, username);
  return 'success';
}

$query;
export function currentPole(): Proposal {
  return proposal;
}

$update;
export function voteForGame(game: string): void {
  // check if call is dao member else return
  const index = votes.findIndex((option) => option.game === game);
  if (index === -1) return;
  votes[index].noOfVotes++;
}

$query;
export function votingStatus(): Vec<Votes> {
  return votes;
}

$query;
export function loadGame(): string {
  return currentGame;
}