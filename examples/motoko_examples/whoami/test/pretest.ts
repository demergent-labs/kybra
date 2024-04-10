import { execSync } from 'child_process';
import { someonePrincipal } from './tests';

async function pretest() {
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
