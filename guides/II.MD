# How to integrate with Internet Identity?
## Introduction
For developers who need to deploy **Internet Identity** on their local environments, this tutorial offers a step-by-step guide. However, if you're working on the mainnet, there's no need to deploy **Internet Identity** since its canister is already available.

## Prerequisites
⚠️ **Important**: Before proceeding, ensure you have the latest `dfx` version `0.14.3-beta.0` installed. Earlier versions are incompatible with this guide. For further context on potential issues with prior versions, refer to [this forum discussion](https://forum.dfinity.org/t/problem-insalling-internet-identity-in-local-setup/20417/20). To install it run the following:
```
DFX_VERSION=0.14.3-beta.0 sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
```
## Troubleshooting dfx setup
If you get any errors while installing dfx and can't solve them you can try this installation method and work with CLI inside a container with its own environment isolated from the main system. Using this method you can exerment with the installation inside the container without the risk of breaking something inside your OS.
### Install Docker
If the `docker -v` command outputs something like 
```
Docker version 24.0.2 ...
``` 
then skip the Docker installation step and go straight to the next step.
Go to the link and follow the instructions to install: 
```
https://docs.docker.com/engine/install/ubuntu/#install-using-the-repository
```

### Create an isolated environment
In VS code, open the extensions panel enter `Dev Containers` in the search and install the extension from Microsoft verified publisher.

Once the extension is installed, go to `View -> Command Palette` and type `(re)open folder in container`. 

At the prompt opens, type Ubuntu and hit enter then select `jammy` and hit enter and then click `OK`.

After downloading the images, docker will create a new container based on Ubuntu 22.04 lts and connect the project directory to it.
Now you are working in the container and your terminal should look something like this: 
```
vscode ➜ /workspaces/typescript-bootcamp (main) $ 
```

### Setup nodejs using nvm
To work with `azel` you also need to install `nodejs`. The easiest way to do this is to use node version manager(`nvm`). Here is a complete guide for different operating systems 
```
https://www.freecodecamp.org/news/node-version-manager-nvm-install-guide/
```

Enter this command in the terminal 
```
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash
``` 

This command will download and run the `nvm` installation script and add it to the `PATH` variable.

Type `source ~/.bashrc` in the terminal for the changes to take effect and install the latest recommended version of `nodejs`(18.17.1 at this time) using this command ```nvm install 18.17.1```.

You are ready to go!

Go back to the previous point and install dfx as suggested.

## Step 1: Add Internet Identity Into Your Project
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
dfx deploy internet_identity
```
## Step 3: Add An Essential Library to Your Project
DFINITY provides the auth-client library, simplifying the Internet Identity integration process. Install it with:

```
npm install @dfinity/auth-client @dfinity/identity
```
## Step 4: Create the authClient 
To facilitate user login, integrate the following function into your application. This function should be triggered when a user performs an action like clicking a button or any other interaction you identify as a login trigger.
```
const login = async () => {
    const authClient = await AuthClient.create();
    const isLocalNetwork = process.env.DFX_NETWORK == 'local';
    const identityProviderUrl = isLocalNetwork ? 
        `http://127.0.0.1:4943/?canisterId=${process.env.CANISTER_ID_INTERNET_IDENTITY}` : 
        'https://identity.ic0.app/';

    authClient.login({
      identityProvider: identityProviderUrl,
      maxTimeToLive: BigInt(7 * 24 * 60 * 60 * 1000 * 1000 * 1000),
      onSuccess: async () => {},
    });
};
```

How It Works:
Upon invoking this function, a window from `identity.ic0.app` will appear. After you authenticate, the system will store your delegation in the `localStorage` and establish your identity.

## Step 5: Getting the principal (Optional)
To obtain the principal in his textual formal, execute the following code:
```
const identity = await authClient.getIdentity();
const principal = identity.getPrincipal().toString();
```
## Step 6: Making Authenticated Calls
You can use that identity to perform authenticated calls with the `@dfinity/agent` **Actor**. 
The **Actor** is a `JavaScript` object representing the canister on the client side and enabling interactions from the browser.

```
const actor = Actor.createActor(idlFactory, {
  agent: new HttpAgent({
    identity,
  }),
  canisterId,
});
```

## Tips
For a smoother transition between mainnet and local development, you can leverage these environment variables in your client-side code:
- `process.env.DFX_NETWORK` will tell you whether you're on local or ic.
- `process.env.CANISTER_ID_INTERNET_IDENTITY` provides the canister ID of the Internet Identity canister.