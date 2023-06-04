import { execSync } from 'child_process';

async function pretest() {
    await new Promise((resolve) => setTimeout(resolve, 5000));

    execSync(`dfx canister uninstall-code stable_memory || true`, {
        stdio: 'inherit'
    });

    execSync(`dfx deploy`, {
        stdio: 'inherit'
    });

    await new Promise((resolve) => setTimeout(resolve, 10_000));

    execSync(`dfx generate`, {
        stdio: 'inherit'
    });

    execSync(
        `dfx ledger fabricate-cycles --canister stable_memory --cycles 100000000000000`,
        {
            stdio: 'inherit'
        }
    );
}

pretest();
