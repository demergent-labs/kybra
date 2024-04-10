import { execSync } from 'child_process';

async function pretest() {
    execSync(`dfx canister uninstall-code stdlib || true`, {
        stdio: 'inherit'
    });

    execSync(`dfx deploy stdlib`, {
        stdio: 'inherit'
    });

    execSync(`dfx generate stdlib`, {
        stdio: 'inherit'
    });
}

pretest();
