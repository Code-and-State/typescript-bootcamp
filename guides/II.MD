# How to integrate with Internet Identity?
## Introduction
For developers who need to deploy **Internet Identity** on their local environments, this tutorial offers a step-by-step guide. However, if you're working on the mainnet, there's no need to deploy **Internet Identity** since its canister is already available.

### Step 1: Add Internet Identity Into Your Project
In your dfx.json file, locate the canister section and include the following code block:
```
"internet_identity": {
  "type": "custom",
  "candid": "https://github.com/dfinity/internet-identity/releases/latest/download/internet_identity.did",
  "wasm": "https://github.com/dfinity/internet-identity/releases/latest/download/internet_identity_dev.wasm.gz",
  "remote": {
    "id": {
      "ic": "rdmx6-jaaaa-aaaaa-aaadq-cai"
    }
  },
  "frontend": {}
}
```
This snippet will fetch the most recent **Internet Identity** canister release from **DFINITY's** GitHub repository and deploy it on your local replica. Note that if you're on the mainnet, the Internet Identity canister's ID is `rdmx6-jaaaa-aaaaa-aaadq-cai`.

## Step 2: Deploying Internet Identity on Your Local Environment
To deploy, run the command:

```
dfx deploy internet-identity
```
## Step 3: Add An Essential Library to Your Project
DFINITY provides the auth-client library, simplifying the Internet Identity integration process. Install it with:

```
npm install @dfinity/auth-client @dfinity/identity --save-dev
````

## Step 4: Dive In and Experiment!
It's now your turn! 
You can design a button and integrate it with Internet Identity. For a practical example, check out:

- [Internet Identity Integration Guide](https://internetcomputer.org/docs/current/developer-docs/integrations/internet-identity/integrate-identity/#using-the-auth-client-library-to-log-in-with-internet-identity)

## Tips
For a smoother transition between mainnet and local development, you can leverage these environment variables in your client-side code:

- `process.env.DFX_NETWORK` will tell you whether you're on local or ic.
- `process.env.CANISTER_ID_INTERNET_IDENTITY` provides the canister ID of the Internet Identity canister.