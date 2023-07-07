import { getCanisterId } from 'azle/test';
import { execSync } from 'child_process';

async function pretest() {
    await new Promise((resolve) => setTimeout(resolve, 5000));

    execSync(`dfx canister uninstall-code rejections || true`, {
        stdio: 'inherit'
    });

    execSync(`dfx canister uninstall-code some_service || true`, {
        stdio: 'inherit'
    });

    execSync(`dfx deploy some_service`, {
        stdio: 'inherit'
    });

    execSync(
        `dfx deploy rejections --argument '(principal "${getCanisterId(
            'some_service'
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
