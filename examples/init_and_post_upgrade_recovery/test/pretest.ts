import { execSync } from 'child_process';

async function pretest() {
    await new Promise((resolve) => setTimeout(resolve, 5000));

    execSync(
        `dfx canister uninstall-code init_and_post_upgrade_recovery || true`,
        {
            stdio: 'inherit'
        }
    );

    execSync(`dfx deploy init_and_post_upgrade_recovery --argument '(false)'`, {
        stdio: 'inherit'
    });

    execSync(`dfx generate init_and_post_upgrade_recovery`, {
        stdio: 'inherit'
    });
}

pretest();
