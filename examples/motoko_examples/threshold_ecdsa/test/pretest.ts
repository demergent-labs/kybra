import { execSync } from 'child_process';

async function pretest() {
    await new Promise((resolve) => setTimeout(resolve, 5000));

    execSync(`dfx canister uninstall-code threshold_ecdsa || true`, {
        stdio: 'inherit'
    });

    execSync(`dfx deploy`, {
        stdio: 'inherit'
    });

    await new Promise((resolve) => setTimeout(resolve, 10_000));

    execSync(`dfx generate`, {
        stdio: 'inherit'
    });
}

pretest();
