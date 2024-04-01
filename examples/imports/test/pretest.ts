import { execSync } from 'child_process';

async function pretest() {
    execSync(`pip install boltons==23.0.0`, {
        stdio: 'inherit'
    });

    execSync(`dfx canister uninstall-code imports || true`, {
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
