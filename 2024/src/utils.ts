/**
 * Day utility fn for the day files.
 */
export function day(cb: () => Promise<void>) {
    return cb;
}

/**
 * The day runner fn.
 */
export type DayRunner = () => void | Promise<void>;
