/**
 * Day utility fn for the day files.
 */
export function day(cb: DayRunner) {
    return cb;
}

/**
 * The day runner fn.
 */
export type DayRunner = () => void | Promise<void>;
