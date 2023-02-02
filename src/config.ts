// Place any global data in this file.
// You can import this data from anywhere in your site by using the `import` keyword.

export const SITE_TITLE = 'Brian Ryall';
export const SITE_DESCRIPTION =
	'Welcome to my blog! I write about various topics including development, finances, cubing, and more';
export const TWITTER_HANDLE = '@polarmutex';
export const MY_NAME = 'Brian Ryall';

// setup in astro.config.mjs
const BASE_URL = new URL(import.meta.env.SITE);
export const SITE_URL = BASE_URL.origin;
