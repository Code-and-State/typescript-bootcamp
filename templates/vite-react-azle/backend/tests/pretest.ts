import { execSync } from 'child_process';

async function pretest() {
    await new Promise((resolve) => setTimeout(resolve, 5000));

    execSync(`dfx canister uninstall-code backend || true`, {
        stdio: 'inherit'
    });

    execSync(`dfx deploy backend`, {
        stdio: 'inherit'
    });

    execSync(`dfx generate backend`, {
        stdio: 'inherit'
    });

    execSync(`bash backend/tests/scripts/copy-dec.sh`, {
        stdio: 'inherit'
    });

}

pretest();