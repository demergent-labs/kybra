import { execSync } from 'child_process';

async function pretest() {
    execSync(`dfx canister uninstall-code quicksort || true`, {
        stdio: 'inherit'
    });

    execSync(`dfx deploy`, {
        stdio: 'inherit'
    });

    execSync(`dfx generate`, {
        stdio: 'inherit'
    });
}

pretest();
