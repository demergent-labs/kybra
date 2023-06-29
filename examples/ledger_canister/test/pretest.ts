import { execSync } from 'child_process';

async function pretest(icp_ledger_path: string) {
    await new Promise((resolve) => setTimeout(resolve, 5000));

    execSync(`dfx canister uninstall-code icp_ledger || true`, {
        stdio: 'inherit'
    });

    execSync(`dfx canister uninstall-code ledger_canister || true`, {
        stdio: 'inherit'
    });

    execSync(`mkdir -p ${icp_ledger_path}`, {
        stdio: 'inherit'
    });

    execSync(
        `cd ${icp_ledger_path} && curl -o ledger.wasm.gz https://download.dfinity.systems/ic/dfdba729414d3639b2a6c269600bbbd689b35385/canisters/ledger-canister_notify-method.wasm.gz`,
        {
            stdio: 'inherit'
        }
    );

    execSync(`cd ${icp_ledger_path} && gunzip -f ledger.wasm.gz`, {
        stdio: 'inherit'
    });

    execSync(
        `cd ${icp_ledger_path} && curl -o ledger.private.did https://raw.githubusercontent.com/dfinity/ic/dfdba729414d3639b2a6c269600bbbd689b35385/rs/rosetta-api/ledger.did`,
        {
            stdio: 'inherit'
        }
    );

    execSync(
        `cd ${icp_ledger_path} && curl -o ledger.public.did https://raw.githubusercontent.com/dfinity/ic/dfdba729414d3639b2a6c269600bbbd689b35385/rs/rosetta-api/ledger_canister/ledger.did`,
        {
            stdio: 'inherit'
        }
    );

    execSync(`dfx deploy ledger_canister`, {
        stdio: 'inherit'
    });

    execSync(
        `dfx deploy icp_ledger --argument='(record {minting_account = "'$(dfx ledger account-id)'"; initial_values = vec { record { "'$(dfx ledger account-id --of-canister ledger_canister)'"; record { e8s=100_000_000_000 } }; }; send_whitelist = vec {}})'`,
        {
            stdio: 'inherit'
        }
    );

    execSync(`dfx generate ledger_canister`, {
        stdio: 'inherit'
    });
}

pretest('src/icp_ledger');
