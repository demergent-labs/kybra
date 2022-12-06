import { run_tests, Test } from 'azle/test';
import { get_tests } from 'azle/examples/motoko_examples/superheroes/test/tests';
import { execSync } from 'child_process';
import { createActor } from './dfx_generated/superheroes';
import { Superhero } from './dfx_generated/superheroes/superheroes.did';

const superheroes_canister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'deploy',
        prep: async () => {
            await new Promise((resolve) => setTimeout(resolve, 5000));

            execSync(`dfx canister uninstall-code superheroes || true`, {
                stdio: 'inherit'
            });

            execSync(`dfx deploy superheroes`, {
                stdio: 'inherit'
            });
        }
    },
    ...get_tests(superheroes_canister as any).filter(
        (value) =>
            value.name != 'update when adding superpowers' &&
            value.name != 'update removing superpowers' &&
            value.name != 'delete with a valid id'
    ),
    {
        name: 'update when adding superpowers',
        test: async () => {
            const spiderman: Superhero = {
                name: 'Spiderman',
                superpowers: [
                    [
                        'superhuman speed',
                        [
                            [
                                'spider-sense',
                                [['wall crawling', [['web shooting', []]]]]
                            ]
                        ]
                    ]
                ]
            };
            const result = await superheroes_canister.update_(0, spiderman);

            return {
                ok: result === true
            };
        }
    },
    {
        name: 'update removing superpowers',
        test: async () => {
            const superman: Superhero = {
                name: 'Superman',
                superpowers: []
            };
            const result = await superheroes_canister.update_(1, superman);

            return {
                ok: result === true
            };
        }
    },
    {
        name: 'delete with a valid id',
        test: async () => {
            const result = await superheroes_canister.delete_hero(0);

            return {
                ok: result === true
            };
        }
    }
];

run_tests(tests);
