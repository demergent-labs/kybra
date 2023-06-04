import { execSync } from 'child_process';

async function pretest() {
    await new Promise((resolve) => setTimeout(resolve, 5000));

    execSync(`dfx canister uninstall-code init || true`, {
        stdio: 'inherit'
    });

    execSync(
        `dfx deploy --argument '(record { id = "0" }, variant { Fire }, principal "rrkah-fqaaa-aaaaa-aaaaq-cai")' init`,
        {
            stdio: 'inherit'
        }
    );

    await new Promise((resolve) => setTimeout(resolve, 10_000));

    execSync(`dfx generate`, {
        stdio: 'inherit'
    });
}

pretest();
