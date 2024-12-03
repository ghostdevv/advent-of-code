import { intro, isCancel, outro, select, type SelectOptions } from '@clack/prompts';
import { ensureDir, expandGlob } from '@std/fs';
import { gray, red, yellow } from '@std/fmt/colors';
import { parseArgs } from '@std/cli';
import { join } from '@std/path';
import dedent from 'dedent';

class Day {
    constructor(
        /**
         * The day number (1 - 24).
         */
        public readonly day: number,
        /**
         * The absolute path to the day's folder on disk.
         */
        public readonly path: string,
    ) {}

    /**
     * A function to run the day's code.
     */
    async run() {
        const files = await Array.fromAsync(expandGlob('*.aoc.ts', { root: this.path }));

        if (!files.length) {
            exit(`No files found for day ${this}`);
        }

        for await (const file of expandGlob('*.aoc.ts', { root: this.path })) {
            const mod: { run: DayRunner } = await import(file.path);

            console.log(yellow(`AOC ${this} `), gray(`Running ${file.name}`));
            await mod.run();
            console.log('');
        }
    }

    /**
     * Render the day as a string
     */
    toString() {
        return display_day(this.day);
    }
}

/**
 * Handle cancel results from Clack
 */
function handle_cancel<T>(value: T): asserts value is Exclude<T, symbol> {
    if (isCancel(value)) {
        outro('Cancelled!');
        Deno.exit(0);
    }
}

function exit(error: string): never {
    console.error(red('AOC    '), error);
    Deno.exit(1);
}

/**
 * Turn a day number into a string. It makes sure all days are two digits.
 */
function display_day(day: number) {
    return `${day < 10 ? '0' : ''}${day}`;
}

/**
 * The type of the `run` function which a day file exports.
 */
type DayRunner = () => void | Promise<void>;

const DAYS_FOLDER = join(import.meta.dirname!, './days');
const CURRENT_DAY = new Date().getDate();

const args = parseArgs<{ day?: boolean | number }>(Deno.args);

if (typeof args.day == 'boolean') {
    exit('day arg must be a number');
}

const days_map = new Map<number, Day>();

for await (const f of Deno.readDir(DAYS_FOLDER)) {
    if (f.isDirectory && /^\d{2}$/.test(f.name)) {
        const day_num = Number.parseInt(f.name);
        days_map.set(day_num, new Day(day_num, join(DAYS_FOLDER, f.name)));
    }
}

let day = days_map.get(args.day!) || null;

if (!day) {
    intro('Advent of Code 2024');

    const options: SelectOptions<Day | number>['options'] = [];

    for (let i = 1; i <= 24; i++) {
        if (days_map.has(i)) {
            const day = days_map.get(i)!;
            options.push({ label: `${day}`, value: day });
        } else if (i <= CURRENT_DAY) {
            options.push({
                label: `Create ${display_day(i)}`,
                hint: `Day missing, do you want to create it?`,
                value: i,
            });
        }
    }

    const choice = await select<Day | number>({
        message: 'Please pick a day to run',
        options,
    });

    handle_cancel(choice);

    if (typeof choice == 'number') {
        const path = join(DAYS_FOLDER, `./${display_day(choice)}`);

        await ensureDir(path);
        await Deno.writeTextFile(
            join(path, './one.aoc.ts'),
            dedent`
                export function run() {
                    console.log('Day ${display_day(choice)} Part One');
                }
            `,
        );

        day = new Day(choice, path);
    } else {
        day = choice;
    }

    outro(`Running day ${day}...`);
}

await day.run();
