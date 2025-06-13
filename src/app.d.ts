// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
/// <reference types="umami-browser" />
declare global {
    namespace App {
        // interface Error {}
        // interface Locals {}
        // interface PageData {}
        // interface PageState {}
        // interface Platform {}
    }
    interface Window {
        umami?: umami.umami;
    }
}

export { };
