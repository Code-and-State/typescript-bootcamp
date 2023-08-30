# Day 2

## Introduction 👋
Welcome to the 2nd day of this Bootcamp! 

Yesterday, was mainly about introducing the week and the concept of DAOs, but today we will dive more into the code and start implementing more features.

## Goal(s) of the day 🎯
A DAO is first and foremost an organization, and like any organization is it composed of different members.  By leveraging the capacities of canisters, we can store data about members of the organisation.  The goal of today  is to write code that will enable any user to register to the DAO, create their profile and save all of this data into our backend. 

Traditionally, to login into dApp most Web3 users use a wallet such as **Metamask** or **Coinbase Wallet**. However, wallets might not be the best way to login into applications. Indeed:

- Most users are not familiar with wallets, but prefer using traditional login systems such as “Login with Google”. Wallets can be quite complex to setup and using wallets introduces a learning curve for those not familiar with crypto.
- Wallets introduce risk of censorship and privacy issues. For instance, last year it was revealed that [Metamask was sharing user information, such as IP address, with Infura, a centralized piece of blockchain infrastructure.](https://www.coindesk.com/tech/2022/11/30/metamask-ip-sharing-debacle-highlights-the-scourge-of-crypto-centralization/)
- Fees and frequent transaction verifications detract from the optimal user experience.

On the Internet Computer, a new authentication system has been developed called **Internet Identity** attempts to solve all the issues caused by traditional wallets (privacy, user experience, usability). Today, we will also explore this identity system, explain how it works and you can integrate is as a dApp developer. 

By the end of the day, here are your goals:

- [ ]  Users are able to login using **Internet Identity**, or another of the common authentication solution on the Internet Computer.
- [ ]  Users are able to register their profile. The data is then saved into our backend canister.
- [ ]  User profile will automatically load whenever a user log back into the application.

## Useful resources 📚
| Title | Type |  URL | Description
|-----------------|-----------------|-----------------|-----------------|
| CRUD application | Repository | [Click Here](https://github.com/demergent-labs/azle/blob/main/examples/simple_user_accounts/src/simple_user_accounts.ts) | An example of a simple CRUD (Create, Read,Update, Delete) canister written with TypeScript.
| Candid | Article | [Click Here](https://demergent-labs.github.io/azle/candid.html) | Candid is a IDL (Interface Description Language) created by DFINITY, for the Internet Computer.
| Inside the Internet Computer - Identity and Authentication on the Internet Computer | Video | [Click Here](https://www.youtube.com/watch?v=9eUTcCP_ELM) | Today, the main means of identity and authentication used online are usernames and passwords. The Internet Computer has replaced this model with a more advanced and secure method of cryptographic authentication, as described in this talk.
| A new chapter in digital identity: Internet Identity | Article | [Click Here](https://github.com/motoko-bootcamp/motoko-starter/blob/main/manuals/appendix/appendix-3/APPENDIX-3.MD) | A high-level read about Internet Identity to understand why it matters and how it works. 
| Internet Identity Integration Guide | Article | [Click Here](https://github.com/Code-and-State/typescript-bootcamp/blob/main/guides/II.MD) | This guide shows how to integrate and test a project with Internet Identity.