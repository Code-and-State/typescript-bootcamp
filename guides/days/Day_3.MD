# Day 3

## Introduction 👋
Congratulation for making it to the 3rd day of this Bootcamp! We hope you are making enough progress and excited about all the new skills you’re learning. Today we’ll talk about money. 💸
## Goal(s) of the day 🎯
Most **DAOs** that have been around for a while will have their own token. These tokens are used to fund operations and show who's contributing more, either in terms of money or effort. If you're more active, you get more tokens. Today, we want to set up a token using the **ICRC_2** standard, which is commonly used in the ecosystem.

To create a token, you need a **canister**. This keeps track of everyone's tokens and their transactions. The **canister** ensures that tokens can be sent around safely. Token transfers are done through **update calls** to make sure everything's secure.

It's best to add this token functionality directly into the existing canister to avoid complications

By the end of the day, here are your goals:

- [ ]  Users automatically receive 100 units of your token whenever they create their profile.
- [ ]  Users are able to send tokens to other members of the DAO through the app.
- [ ]  Users are able to see their balance through the app.
## Useful resources 📚
| Title | Type |  URL | Description
|-----------------|-----------------|-----------------|-----------------|
| ICRC-1 Standard | Article | [Click Here](https://internetcomputer.org/docs/current/developer-docs/integrations/icrc-1/) | `ICRC-1` is a standard for creating Tokens on the Internet Computer.
| ICRC-2 Standard | Repository | [Click Here](https://github.com/dfinity/ICRC-1/blob/main/standards/ICRC-2/README.md) | `ICRC-2` is an extension of the base `ICRC-1` standard. `ICRC-2` specifies a way for an account owner to delegate token transfers to a third party on the owner's behalf.
| Tokenomics | Video | [Click Here](https://www.youtube.com/watch?v=-LnDHDsdmis) | An overview of tokenomics and how to create a sustainable business model in Web3. 
| Stable Structure | Video | [Click Here](https://demergent-labs.github.io/azle/stable_structures.html) | Stable structures are data structures with familiar APIs that allow access to stable memory. Stable memory is a separate memory location from the heap that currently allows up to 64 GiB of storage. Stable memory persists automatically across upgrades.
