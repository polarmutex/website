use crate::components::feature_card::*;
use crate::components::latest_posts::*;
use leptos::*;

#[component]
pub fn Homepage(cx: Scope) -> impl IntoView {
    view! { cx,
        // sets the document title
        //<Title text="Welcome to Leptos"/>
        <h1>"Welcome to Leptos!"</h1>
        <div class="flex flex-col items-start justify-center max-w-2xl px-4 pb-16 mx-auto border-gray-200 dark:border-gray-700 sm:px-8">
            <div class="flex flex-col-reverse items-start sm:flex-row">
                <div class="flex flex-col pr-8">
                    <h1 class="mb-3 text-3xl font-bold tracking-tight text-black dark:text-white md:text-5xl">
                        "This is "
                        <span class="relative inline-block ml-2 before:absolute before:-inset-1 before:block before:-skew-y-3 before:bg-red-500">
                            <span class="relative text-yellow-400 skew-y-3">"Brian Ryall"</span>
                        </span>
                        "!"
                    </h1>
                    <h2 class="mb-4 text-gray-700 dark:text-gray-200">
                        "An opinionated blog starter for "
                        <span class="font-semibold">
                            "SvelteKit + Tailwind + Netlify/Vercel."
                        </span>
                        " Updated for "
                        <a href="https://github.com/sveltejs/kit/">"SvelteKit 1.0"</a>
                        "!"
                    </h2>
                </div>
            </div>
            <section class="w-full mb-16">
                <h3 class="mb-6 text-2xl font-bold tracking-tight text-black dark:text-white md:text-4xl">
                    "Featured Posts"
                </h3>
                <div class="flex flex-col gap-6 md:flex-row">
                    <FeatureCard title="Welcome to swyxkit 2022!" href="/welcome" string_data="Jan 2022" />
                    <FeatureCard
                        title="Moving to a GitHub CMS"
                        href="/moving-to-a-github-cms"
                        string_data="Jan 2022"
                    />
                    <FeatureCard title="HTML Ipsum demo" href="/moo" string_data="Jan 2022" />
                </div>
            </section>

            <LatestPosts {items} />
        </div>
    }
}
