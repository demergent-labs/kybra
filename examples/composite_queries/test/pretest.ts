import { getCanisterId } from 'azle/test';
import { execSync } from 'child_process';

async function pretest() {
    execSync(`dfx canister uninstall-code canister1 || true`, {
        stdio: 'inherit'
    });

    execSync(`dfx canister uninstall-code canister2 || true`, {
        stdio: 'inherit'
    });

    execSync(`dfx canister uninstall-code canister3 || true`, {
        stdio: 'inherit'
    });

    execSync(`dfx deploy canister3`, {
        stdio: 'inherit'
    });

    execSync(
        `dfx deploy canister2 --argument '(principal "${getCanisterId(
            'canister3'
        )}")'`,
        {
            stdio: 'inherit'
        }
    );

    execSync(
        `dfx deploy canister1 --argument '(principal "${getCanisterId(
            'canister2'
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
