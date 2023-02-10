<script>
    import { TWITTER_HANDLE, SITE_URL } from '../config';
</script>

<footer class="mx-auto mb-8 flex w-full max-w-2xl flex-col items-start justify-center">
    <hr class="border-1 mb-8 w-full border-gray-200 dark:border-gray-800" />
    <div class="grid w-full max-w-2xl grid-cols-1 gap-4 px-4 pb-16 sm:grid-cols-2 sm:px-8">
        <div class="flex flex-col space-y-4">
            <a class="text-gray-500 transition hover:text-gray-300" href="/">Home</a>
            <a class="text-gray-500 transition hover:text-gray-300" href="/about">About</a>
            <a class="text-gray-500 transition hover:text-gray-300" href="/rss.xml" rel="external">
                RSS
            </a>
        </div>
        <div class="flex flex-col space-y-4">
            <a
                class="text-gray-500 transition hover:text-gray-300"
                target="_blank"
                rel="noopener noreferrer"
                href={'https://twitter.com/intent/follow?screen_name=' + TWITTER_HANDLE}
            >
                Twitter
            </a>
            <a
                class="text-gray-500 transition hover:text-gray-300"
                target="_blank"
                rel="noopener noreferrer"
                href={SITE_URL}
            >
                GitHub
            </a>
            <!--<a
                class="text-gray-500 transition hover:text-gray-300"
                target="_blank"
                rel="noopener noreferrer"
                href={MY_YOUTUBE}
            >
                YouTube
            </a>-->
        </div>
    </div>
    <p class="prose px-4 dark:prose-invert sm:px-8">
        This blog is based on the
        <a href="https://swyxkit.netlify.app/">swyxkit</a>
        template.
    </p>
</footer>
