import { execSync } from 'child_process';

async function pretest() {
    execSync(`dfx canister uninstall-code init || true`, {
        stdio: 'inherit'
    });

    execSync(
        `dfx deploy --argument '(record { id = "0" }, variant { Fire }, principal "rrkah-fqaaa-aaaaa-aaaaq-cai")' init`,
        {
            stdio: 'inherit'
        }
    );

    execSync(`dfx generate`, {
        stdio: 'inherit'
    });
}

pretest();
