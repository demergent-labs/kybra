import { execSync } from 'child_process';

async function pretest() {
    execSync(`dfx canister uninstall-code key_value_store || true`, {
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
