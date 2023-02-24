import { defineCollection } from 'astro:content';
import { z } from 'astro:content';

export const postsSchema = z
    .object({
        title: z.string(),
        description: z.string(),
        published: z.date(),
        updated: z.date().optional(),
        draft: z.boolean().optional(),
        featured: z.boolean().optional(),
        tags: z.array(z.string()).default(['others']),
        category: z.string().default('post'),
        // ogImage: z.string().optional(),
    })
    .strict();

const posts = defineCollection({
    schema: postsSchema,
});

export const collections = { posts };
