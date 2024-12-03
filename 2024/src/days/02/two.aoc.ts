import { getInputLines } from '../../utils.ts';

function check(type: number, report: number[], top = false): boolean {
    for (let i = 1; i < report.length; i++) {
        const sum = report[i - 1] - report[i];
        const safe = Math.sign(sum) === type && Math.abs(sum) <= 3;

        if (!safe) {
            return top
                ? check(type, report.toSpliced(i, 1)) || check(type, report.toSpliced(i - 1, 1))
                : false;
        }
    }

    return true;
}

export async function run() {
    const lines = await getInputLines(import.meta.dirname!, './input.txt');

    let safe = 0;

    for (const line of lines) {
        const report = line.split(' ').map((n) => Number.parseInt(n));
        const type = Math.sign(report.at(0)! - report.at(-1)!);

        if (check(type, report, true)) {
            safe++;
        }
    }

    console.log(`Safe reports: ${safe}`);
}
