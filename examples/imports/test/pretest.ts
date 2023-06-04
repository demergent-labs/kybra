import { execSync } from 'child_process';

async function pretest() {
    await new Promise((resolve) => setTimeout(resolve, 5000));

    execSync(`pip install boltons==23.0.0`, {
        stdio: 'inherit'
    });

    execSync(`dfx canister uninstall-code imports || true`, {
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
