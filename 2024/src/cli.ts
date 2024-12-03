import { intro, isCancel, outro, select, type SelectOptions } from '@clack/prompts';
import { ensureDir, existsSync } from '@std/fs';
import type { DayRunner } from './utils.ts';
import { parseArgs } from '@std/cli';
import { join } from '@std/path';
import dedent from 'dedent';

const DAYS_FOLDER = join(import.meta.dirname!, './days');

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
        j;
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

/**
 * Get the day to run. First checking the args, then prompting the user.
 */
async function get_day(days: Day[]): Promise<Day> {
    const { _: args } = parseArgs(Deno.args);

    const args_day = days.find(({ day }) => day === args.at(0)) || null;
    if (args_day) return args_day;

    const current_day = new Date().getDay();

    const options: SelectOptions<Day | 'NEW'>['options'] = days.map((day) => ({
        label: `${day}`,
        value: day,
    }));

    if (!days.find((day) => day.day == current_day)) {
        options.unshift({
            label: `Create ${display_day(current_day)}`,
            value: 'NEW',
            hint: `Today is missing, create it?`,
        });
    }

    const day = await select<Day | 'NEW'>({
        message: 'Please pick a day to run',
        options,
    });

    handle_cancel(day);

    if (day == 'NEW') {
        const path = join(DAYS_FOLDER, `./${display_day(current_day)}`);

        await ensureDir(path);
        await Deno.writeTextFile(
            join(path, './main.ts'),
            dedent`
                import { day } from '../../utils.ts';

                export default day(async () => {
                    console.log('Day ${display_day(current_day)}');
                });
            `,
        );

        return new Day(current_day, path);
    }

    return day;
}

intro('Advent of Code 2024');

const days = (await Array.fromAsync(Deno.readDir(DAYS_FOLDER)))
    .filter((f) => f.isDirectory && /^\d{2}$/.test(f.name))
    .map((f) => new Day(Number.parseInt(f.name), join(DAYS_FOLDER, f.name)))
    .sort((a, b) => b.day - a.day);

if (days.length == 0) {
    throw new Error('No days found!');
}

const day = await get_day(days);

outro(`Running day ${day}...`);

await day.run();
