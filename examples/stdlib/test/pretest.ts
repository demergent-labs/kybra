import { execSync } from 'child_process';

async function pretest() {
    await new Promise((resolve) => setTimeout(resolve, 5000));

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
