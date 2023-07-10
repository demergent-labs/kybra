import { Principal } from '@dfinity/principal';
import { getCanisterId, runTests, Test } from 'azle/test';
import { createActor } from './dfx_generated/keywords';
import {
    KeywordRecord,
    KeywordVariant,
    SimpleRecord
} from './dfx_generated/keywords/keywords.did';

const keywordsCanister = createActor(getCanisterId('keywords'), {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

const tests: Test[] = [
    {
        name: 'keyword_variant',
        test: async () => {
            const keyword_variant: KeywordVariant = { raise: null };
            const result = await keywordsCanister.keyword_variant(
                keyword_variant
            );

            return {
                Ok: 'raise' in result
            };
        }
    },
    {
        name: 'rust_keyword_variant',
        test: async () => {
            const result = await keywordsCanister.rust_keyword_variant();

            return {
                Ok: 'type' in result
            };
        }
    },
    {
        name: 'simple_keyword',
        test: async () => {
            const simple_record: SimpleRecord = {
                from: 'testing'
            };
            const result = await keywordsCanister.simple_keyword(simple_record);

            return {
                Ok: result.from === 'testing'
            };
        }
    },
    {
        name: 'rust_keyword',
        test: async () => {
            const result = await keywordsCanister.rust_keyword();

            return {
                Ok:
                    result.abstract === false &&
                    result.become === 'Become' &&
                    result.const === 3n &&
                    result.crate === 7n &&
                    result.fn === 'Function' &&
                    result.pub === null &&
                    result.type.every(
                        (entry, index) =>
                            entry === new Uint8Array([84, 121, 112, 101])[index]
                    )
            };
        }
    },
    {
        name: 'complex_keyword',
        test: async () => {
            const result = await keywordsCanister.complex_keyword();

            const expected_output: KeywordRecord = {
                False: false,
                True: 'False',
                and: 1n,
                assert: 2,
                class: 3,
                def: 4,
                del: 5n,
                elif: 6n,
                except: 7,
                finally: 8,
                from: 9,
                global: 10n,
                import: 11.12,
                is: 13.14,
                lambda: ['False'],
                nonlocal: 'False',
                not: { from: 'False' },
                or: {
                    with_: false,
                    with__: false,
                    with___: false,
                    with_______________________________________________________________________:
                        false,
                    with______1: false
                },
                pass: { raise: null },
                raise: new Uint8Array([102, 97, 108, 115, 101]),
                with: Principal.fromText('aaaaa-aa'),
                None: [false, true],
                as: null,
                await: [18n, 19n, 20n],
                break: [],
                continue: new Int16Array([21, 22, 23]),
                else: [false],
                for: false,
                if: false,
                in: false,
                return: false,
                try: false,
                while: false,
                yield: false
            };

            return {
                Ok:
                    result.False === expected_output.False &&
                    result.True === expected_output.True &&
                    result.and === expected_output.and &&
                    result.assert === expected_output.assert &&
                    result.class === expected_output.class &&
                    result.def === expected_output.def &&
                    result.del === expected_output.del &&
                    result.elif === expected_output.elif &&
                    result.except === expected_output.except &&
                    result.finally === expected_output.finally &&
                    result.from === expected_output.from &&
                    result.global === expected_output.global &&
                    Math.ceil(result.import) ===
                        Math.ceil(expected_output.import) &&
                    result.is === expected_output.is &&
                    result.lambda[0] === expected_output.lambda[0] &&
                    result.nonlocal === expected_output.nonlocal &&
                    result.not.from === expected_output.not.from &&
                    result.or.with_ === expected_output.or.with_ &&
                    result.or.with__ === expected_output.or.with__ &&
                    result.or.with___ === expected_output.or.with___ &&
                    result.or.with______1 === expected_output.or.with______1 &&
                    result.or
                        .with_______________________________________________________________________ ===
                        expected_output.or
                            .with_______________________________________________________________________ &&
                    'raise' in result.pass &&
                    result.raise.every(
                        (element, index) =>
                            element === expected_output.raise[index]
                    ) &&
                    result.None[0] === expected_output.None[0] &&
                    result.None[1] === expected_output.None[1] &&
                    result.as === expected_output.as &&
                    result.await.every(
                        (element, index) =>
                            element === expected_output.await[index]
                    ) &&
                    result.break.length === 0 &&
                    result.continue.every(
                        (element, index) =>
                            element === expected_output.continue[index]
                    ) &&
                    result.else[0] === expected_output.else[0] &&
                    result.for === expected_output.for &&
                    result.if === expected_output.if &&
                    result.in === expected_output.in &&
                    result.return === expected_output.return &&
                    result.try === expected_output.try &&
                    result.while === expected_output.while &&
                    result.yield === expected_output.yield &&
                    result.with.toString() === 'aaaaa-aa'
            };
        }
    }
];

runTests(tests);
