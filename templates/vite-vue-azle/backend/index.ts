import { $query, $update, nat } from 'azle';

// This is a global variable that is stored on the heap
let counter : nat = BigInt(0);

// Query calls complete quickly because they do not go through consensus
$query;
export function get(): nat {
    return counter;
}

// Update calls take a few seconds to complete
// This is because they persist state changes and go through consensus
$update;
export function add(n : nat): nat {
    counter += n; //
    return counter;
}


$update;
export function inc(): nat {
    counter += BigInt(1);
    return counter
}

