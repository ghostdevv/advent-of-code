import { getInput } from '../../utils.ts';

function getColumn(input: string, regex: RegExp) {
    return input.match(regex)!.map((n) => Number.parseInt(n)).sort((a, b) => b - a);
}

export async function run() {
    const input = await getInput(import.meta.dirname!, './input.txt');
    const left = getColumn(input, /^\d+/gm);
    const right = getColumn(input, /\d+$/gm);

    let total = 0;

    for (let i = 0; i < left.length; i++) {
        total += Math.abs(left[i] - right[i]);
    }

    console.log(`Total distances: ${total}`);
}
