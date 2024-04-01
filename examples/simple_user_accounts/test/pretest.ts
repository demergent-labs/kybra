import { execSync } from 'child_process';

async function pretest() {
    execSync(`dfx canister uninstall-code simple_user_accounts || true`, {
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
