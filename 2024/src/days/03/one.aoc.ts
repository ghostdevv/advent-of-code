import { getInput } from '../../utils.ts';

export async function run() {
    const input = await getInput(import.meta.dirname!, './input.txt');

    const result = input
        .matchAll(/mul\((\d+),(\d+)\)/g)
        .map(([, a, b]) => Number.parseInt(a) * Number.parseInt(b))
        .reduce((a, x) => a + x, 0);

    console.log(`Sum of all mul statements: ${result}`);
}
