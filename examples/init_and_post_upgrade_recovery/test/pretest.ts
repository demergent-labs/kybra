import { execSync } from 'child_process';

async function pretest() {
    execSync(
        `dfx canister uninstall-code init_and_post_upgrade_recovery || true`,
        {
            stdio: 'inherit'
        }
    );

    execSync(`dfx build`, {
        stdio: 'inherit'
    });

    execSync(`dfx generate init_and_post_upgrade_recovery`, {
        stdio: 'inherit'
    });
}

pretest();
