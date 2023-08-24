import { blob, Result, $update, match, ic, nat } from 'azle';
import { managementCanister } from 'azle/canisters/management';
import  decodeUtf8 from 'decode-utf8';

$update;
export async function randomBytes(): Promise<string> {
    const randomBytesCall =await managementCanister.raw_rand().call();
    
    return match(randomBytesCall, {
        Ok: (rand) => (decodeUtf8(Uint8Array.from(rand))),
        Err: (err) => ic.trap(err ?? 'Error occurred'),
    });
}