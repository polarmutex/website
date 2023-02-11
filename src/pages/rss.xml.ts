import rss from '@astrojs/rss';
import { CollectionEntry, getCollection } from 'astro:content';

import { SITE_DESCRIPTION, SITE_TITLE } from '@config';

function sortPosts(a: CollectionEntry<'posts'>, b: CollectionEntry<'posts'>) {
    return Number(b.data.publishDate) - Number(a.data.publishDate);
}

export const get = async (context: any) => {
    const unsortedPosts = await getCollection('posts');
    const posts = unsortedPosts.sort((a, b) => sortPosts(a, b));

    return rss({
        title: `${SITE_TITLE}’s Blog`,
        description: SITE_DESCRIPTION,
        site: context.site!.href,
        items: posts.map((item) => ({
            title: item.data.title,
            description: item.data.description,
            link: `/posts/${item.slug}/`,
            pubDate: item.data.published,
        })),
    });
};
