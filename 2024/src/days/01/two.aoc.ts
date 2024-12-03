import { getInput } from '../../utils.ts';

function getColumn(input: string, regex: RegExp) {
    return input.match(regex)!.map((n) => Number.parseInt(n));
}

export async function run() {
    const input = await getInput(import.meta.dirname!, './input.txt');
    const left = getColumn(input, /^\d+/gm);
    const right = getColumn(input, /\d+$/gm);

    let similarity = 0;

    for (const number of left) {
        similarity += number * right.filter((n) => n == number).length;
    }

    console.log(`Total similarity: ${similarity}`);
}
