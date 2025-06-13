import { mdsvex } from 'mdsvex';
import adapter from '@sveltejs/adapter-node';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import remarkGithub from 'remark-github';
import remarkAbbr from 'remark-abbr';
import rehypeSlug from 'rehype-slug';
import rehypeAutolinkHeadings from 'rehype-autolink-headings';

// mdsvex config
const mdsvexConfig = {
    extensions: ['.svelte.md', '.md', '.svx'],
    // layout: {
    //     _: './src/mdsvexlayout.svelte' // default mdsvex layout
    // },
    remarkPlugins: [
        [
            remarkGithub,
            {
                // Use your own repository
                repository: 'https://github.com/mvasigh/sveltekit-mdsvex-blog.git'
            }
        ],
        remarkAbbr
    ],
    rehypePlugins: [
        rehypeSlug,
        [
            rehypeAutolinkHeadings,
            {
                behavior: 'wrap'
            }
        ]
    ]
};

/** @type {import('@sveltejs/kit').Config} */
const config = {
    preprocess: [vitePreprocess(), mdsvex(mdsvexConfig)],
    kit: { adapter: adapter() },
    extensions: ['.svelte', '.svx', ...mdsvexConfig.extensions]
};

export default config;
