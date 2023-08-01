import { execSync } from 'child_process';
import * as fs from 'fs';
import * as path from 'path';
import rimraf from 'rimraf';

const sourceDir = path.join(__dirname, 'src', 'declarations', 'backend');
const destDir = path.join(__dirname, 'backend', 'declarations');

// If destination directory exists, delete it
if (fs.existsSync(destDir)) {
    rimraf.sync(destDir);
}

// Rename and move the directory
fs.rename(sourceDir, destDir, (err) => {
    if (err) {
        console.error('An error occurred while renaming/moving the directory:', err);
    } else {
        console.log('Directory renamed and moved successfully!');
    }
});

async function pretest() {
    await new Promise((resolve) => setTimeout(resolve, 5000));

    execSync(`dfx canister uninstall-code backend || true`, {
        stdio: 'inherit'
    });

    execSync(`dfx deploy  backend`, {
        stdio: 'inherit'
    });

    execSync(`dfx generate backend`, {
        stdio: 'inherit'
    });
}

pretest();
