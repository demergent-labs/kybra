import { getCanisterId } from 'azle/test';
import { execSync } from 'child_process';

async function pretest() {
    execSync(`dfx canister uninstall-code func_types || true`, {
        stdio: 'inherit'
    });

    execSync(`dfx canister uninstall-code notifiers || true`, {
        stdio: 'inherit'
    });

    execSync(`dfx deploy notifiers`, {
        stdio: 'inherit'
    });

    execSync(
        `dfx deploy func_types --argument '(principal "${getCanisterId(
            'notifiers'
        )}")'`,
        {
            stdio: 'inherit'
        }
    );

    execSync(`dfx generate`, {
        stdio: 'inherit'
    });
}

pretest();
