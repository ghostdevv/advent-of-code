import { intro, isCancel, outro, select, type SelectOptions } from '@clack/prompts';
import { ensureDir, existsSync } from '@std/fs';
import type { DayRunner } from './utils.ts';
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
        const path = join(this.path, './main.ts');

        if (!existsSync(path)) {
            throw new Error(`No main.ts found for day ${this} (${path})`);
        }

        const mod = await import(path);
        const runner = mod.default as DayRunner | undefined;

        if (!runner) {
            throw new Error(`Unable to find runner for day ${this} (${path})`);
        }

        await runner();
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

/**
 * Turn a day number into a string. It makes sure all days are two digits.
 */
function display_day(day: number) {
    return `${day < 10 ? '0' : ''}${day}`;
}

const DAYS_FOLDER = join(import.meta.dirname!, './days');
const CURRENT_DAY = new Date().getDate();

intro('Advent of Code 2024');

const days_map = new Map<number, Day>();

for await (const f of Deno.readDir(DAYS_FOLDER)) {
    if (f.isDirectory && /^\d{2}$/.test(f.name)) {
        const day_num = Number.parseInt(f.name);
        days_map.set(day_num, new Day(day_num, join(DAYS_FOLDER, f.name)));
    }
}

const args = parseArgs(Deno.args);
let day = days_map.get(args._.at(0) as number) || null;

if (!day) {
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
            join(path, './main.ts'),
            dedent`
                import { day } from '../../utils.ts';
    
                export default day(async () => {
                    console.log('Day ${display_day(choice)}');
                });
            `,
        );

        day = new Day(choice, path);
    } else {
        day = choice;
    }
}

outro(`Running day ${day}...`);

await day.run();
