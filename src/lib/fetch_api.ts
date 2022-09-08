/*  */
import { writable, type Writable } from "svelte/store";

export default function (url: string): (Writable<{}> | (() => Promise<void>))[] {
    const loading: Writable<boolean> = writable(false);
    const error: Writable<boolean> = writable(false);
    const data: Writable<{}> = writable({});

    async function get(): Promise<void> {
        loading.set(true);
        error.set(false);

        try {
            const response: Response = await fetch(url);
            data.set(await response.json());
        } catch(e) {
            error.set(e);
            if (error instanceof Error) {
                reportError("Error trying to fetch resource.")
            }
        }
        loading.set(false);
    }
    get();

    return [data, loading, error, get];
}
