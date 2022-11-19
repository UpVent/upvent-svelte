import { api_result } from "src/stores/store";
import { describe, expect, it} from "vitest";

/** Test the API result variable */
describe("Create empty array for storing data", () => {
    it("Returns an empty writable storage", () => {
        expect(api_result).toBeDefined();
    });
});

/** Test if the list is correctly emptied */
describe("Make the api_result array empty like components do", () => {
    api_result.length = 0;
    it("Gives the Svelte store a length of zero", () => {
        expect(api_result.length).toBe(0);
    });
});