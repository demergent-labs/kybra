import { execSync } from 'child_process';

async function pretest() {
    execSync(`dfx canister uninstall-code management_canister || true`, {
        stdio: 'inherit'
    });

    execSync(`dfx deploy management_canister`, {
        stdio: 'inherit'
    });

    execSync(
        `dfx ledger fabricate-cycles --canister management_canister --cycles 100000000000000`,
        {
            stdio: 'inherit'
        }
    );

    execSync(`dfx generate management_canister`, {
        stdio: 'inherit'
    });
}

pretest();
