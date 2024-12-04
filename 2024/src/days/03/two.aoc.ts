import { getInput } from '../../utils.ts';

export async function run() {
    const input = await getInput(import.meta.dirname!, './input.txt');

    const result = input
        .matchAll(/mul\((\d+),(\d+)\)/g)
        .filter(({ index }) => {
            const current = `do()${input.slice(0, index)}`;
            return current.lastIndexOf("don't()") < current.lastIndexOf('do()');
        })
        .map(([, a, b]) => Number.parseInt(a) * Number.parseInt(b))
        .reduce((a, x) => a + x, 0);

    console.log(`Sum of all mul statements with do's and don'ts: ${result}`);
}
