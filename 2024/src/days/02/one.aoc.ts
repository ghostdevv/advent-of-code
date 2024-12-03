import { getInputLines } from '../../utils.ts';

export async function run() {
    const lines = await getInputLines(import.meta.dirname!, './input.txt');

    let safe = 0;

    for (const line of lines) {
        const report = line.split(' ').map((n) => Number.parseInt(n));
        const type = Math.sign(report.at(0)! - report.at(-1)!);

        const is_safe = report.every((n, i, a) =>
            i == 0 || Math.sign(a[i - 1] - n) === type && Math.abs(a[i - 1] - n) <= 3
        );

        if (is_safe) {
            safe++;
        }
    }

    console.log(`Safe reports: ${safe}`);
}
