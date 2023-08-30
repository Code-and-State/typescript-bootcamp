# Day 6
## Introduction 👋
Welcome to Day 6!

This will be the last module of the week, Day 7 will be used for catching up on any remaining items and showing you possible next steps after this Bootcamp! So enjoy ☀️

## Goal(s) of the day 🎯
We created a token and a voting system for our DAO to align everyone's interests.

Essentially:

- The token can be exchanged among members of the community to exchange services and valuables assets.
- If our community does good things, more people might want our tokens, so their value goes up.

As it stands right now, our DAO has a major flaw. What prevents someone from buying a lot of tokens, voting on a decision, and then selling all their tokens on the market? This is called a **“buy-vote-sell”** attack.

A competitor could use this method to mess with the DAO's goals. Fortunately, there are several ways to prevent such an attack:

- **Lock-up period**: After or before voting, the tokens are locked for a set period.
- **Token aging**: The longer you have your tokens, the stronger your vote. This encourages keeping tokens and reduces the influence of quick buy-sell actions.
- **Quadratic voting**: This is a mechanism where the cost of additional votes grows quadratically.The more votes you want, the more tokens it costs. Like, one vote costs one token, two votes cost four tokens, and it keeps multiplying. It’s designed so one person or group can't easily take over. This is the mechanism used [by Gitcoin to determine which projects to fund.](https://www.gitcoin.co/blog/gitcoin-grants-quadratic-funding-for-the-world)
- **Reputation or identity system**: Besides tokens, we could factor in a member’s reputation in the DAO for voting. Long-time, active members could get a stronger voice, even if someone else just buys tons of tokens.

And that’s only the tip of the iceberg. DAOs are still very experimental. We'll see a lot of different models in the coming years, and hopefully, we'll learn what works.

Your task of the day is to i**mprove your decision system.**

To do so, we will take inspiration from the NNS (Network Nervous System), the DAO of the Internet Computer. In the NNS, voting power comes from locking their ICP token in something called neurons. The voting power depends on:

- The time they commit to locking up tokens (**Dissolve delay**)
- How long they've had them locked (**Age**)
- The number of tokens they’ve staked.

This model aligns the incentives of the DAO with long-term holders. Making for a more sustainable model. At the end of the day, here are your goals:

- [ ]  You have implemented a staking system, which enables members of your DAO to lock tokens for a specific period.
- [ ]  You have modified your voting system to take into account the new staking system. Members who stake their assets have an increase in voting power, that you’ll set.
- [ ]  You have verified that your code is safe and secure.
- [ ]  (**Optional**) You have implemented a quadratic voting system.
## Useful resources 📚
| Title | Type |  URL | Description
|-----------------|-----------------|-----------------|-----------------|
| What is Quadratic Funding? | Article | [Click Here](https://cointelegraph.com/explained/quadratic-funding-the-future-of-crowdfunding-explained#:~:text=The%20idea%20behind%20quadratic%20funding,and%20the%20total%20amount%20donated.) | Quadratic Funding is a new method of funding public goods.
| The Network Nervous System (NNS) explained | Article | [Click Here](https://github.com/motoko-bootcamp/motoko-starter/blob/main/manuals/appendix/appendix-4/APPENDIX-4.MD) | The most advanced DAO (so far) on the Internet Computer is the Network Nervous System, which governs the network's future, coordinates parties, and structures its layout.
| Inside The Internet Computer - The NNS | Video | [Click Here](https://www.youtube.com/watch?v=hWnsluxmRqc&list=PLuhDt1vhGcrfHG_rnRKsqZO1jL_Pd970h) | The Network Nervous System (NNS) is an open algorithmic governance system that controls the Internet Computer network. 
| Security best practices | Article | [Click Here](https://internetcomputer.org/docs/current/developer-docs/security/) | This document provides security best practices for developing canisters and web apps served by canisters on the Internet Computer.
