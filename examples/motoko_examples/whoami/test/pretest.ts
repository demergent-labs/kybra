import { execSync } from 'child_process';
import { someonePrincipal } from 'azle/examples/motoko_examples/whoami/test/tests';

async function pretest() {
    await new Promise((resolve) => setTimeout(resolve, 5000));

    execSync(`dfx canister uninstall-code whoami || true`, {
        stdio: 'inherit'
    });

    execSync(
        `dfx deploy --argument '(principal "${someonePrincipal}")' whoami`,
        {
            stdio: 'inherit'
        }
    );

    execSync(`dfx generate`, {
        stdio: 'inherit'
    });
}

pretest();
