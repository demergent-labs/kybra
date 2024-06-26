import { execSync } from 'child_process';

async function pretest() {
    execSync(`dfx canister uninstall-code null_example || true`, {
        stdio: 'inherit'
    });

    execSync(`dfx deploy null_example`, {
        stdio: 'inherit'
    });

    execSync(`dfx generate`, {
        stdio: 'inherit'
    });
}

pretest();
