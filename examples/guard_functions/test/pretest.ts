import { execSync } from 'child_process';

async function pretest() {
    execSync(`dfx canister uninstall-code guard_functions || true`, {
        stdio: 'inherit'
    });

    execSync(`dfx deploy guard_functions`, {
        stdio: 'inherit'
    });

    execSync(`dfx generate`, {
        stdio: 'inherit'
    });
}

pretest();
