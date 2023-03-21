import { runTests, Test } from 'azle/test';
import { getTests } from 'azle/examples/motoko_examples/superheroes/test/tests';
import { createActor } from './dfx_generated/superheroes';
import { Superhero } from './dfx_generated/superheroes/superheroes.did';

const superheroesCanister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    ...getTests(superheroesCanister as any).filter(
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
            const result = await superheroesCanister.update_(0, spiderman);

            return {
                Ok: result === true
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
            const result = await superheroesCanister.update_(1, superman);

            return {
                Ok: result === true
            };
        }
    },
    {
        name: 'delete with a valid id',
        test: async () => {
            const result = await superheroesCanister.delete_hero(0);

            return {
                Ok: result === true
            };
        }
    }
];

runTests(tests);
