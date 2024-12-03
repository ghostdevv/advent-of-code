import { join } from '@std/path';

/**
 * Read a given file. Takes in a base dir, and then the file path.
 */
export async function getInput(cwd: string, file: string) {
    const path = join(cwd, file);
    return await Deno.readTextFile(path);
}

/**
 * Reads a given file, and get the lines.
 */
export async function getInputLines(cwd: string, file: string) {
    return (await getInput(cwd, file)).split('\n');
}
