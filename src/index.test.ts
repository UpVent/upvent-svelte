import { describe, it, expect } from 'vitest';
import {
    fapi_url,
    api_user,
    api_user_pass,
    facebook_link,
    whatsapp_link,
    twitter_link,
    instagram_link,
    linkedin_link,
    github_link,
    email_link
} from '$lib/common/settings';

/** Test all configuration variables are properly configured / fetched from .env file */
describe("Make sure all options under contig.ts have a valid value", () => {
    it("Checks if the API URL is defined", () => {
        expect(fapi_url).toBeDefined();
        expect(fapi_url).not.toBeFalsy();
    });

    it("Checks if the API URL is defined", () => {
        expect(api_user).toBeDefined();
        expect(api_user).not.toBeFalsy();
    });

    it("Checks if the API URL is defined", () => {
        expect(api_user_pass).toBeDefined();
        expect(api_user_pass).not.toBeFalsy();
    });

    it("Checks if the API URL is defined", () => {
        expect(facebook_link).toBeDefined();
        expect(facebook_link).not.toBeFalsy();
    });

    it("Checks if the API URL is defined", () => {
        expect(whatsapp_link).toBeDefined();
        expect(whatsapp_link).not.toBeFalsy();
    });

    it("Checks if the API URL is defined", () => {
        expect(twitter_link).toBeDefined();
        expect(twitter_link).not.toBeFalsy();
    });

    it("Checks if the API URL is defined", () => {
        expect(instagram_link).toBeDefined();
        expect(instagram_link).not.toBeFalsy();
    });

    it("Checks if the API URL is defined", () => {
        expect(linkedin_link).toBeDefined();
        expect(linkedin_link).not.toBeFalsy();
    });

    it("Checks if the API URL is defined", () => {
        expect(github_link).toBeDefined();
        expect(github_link).not.toBeFalsy();
    });

    it("Checks if the API URL is defined", () => {
        expect(email_link).toBeDefined();
        expect(email_link).not.toBeFalsy();
    });
});
