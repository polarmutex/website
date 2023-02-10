import { defineCollection } from 'astro:content';
import { z } from 'astro:content';

export const blogSchema = z
    .object({
        author: z.string().optional(),
        pubDatetime: z.date(),
        title: z.string(),
        postSlug: z.string().optional(),
        featured: z.boolean().optional(),
        draft: z.boolean().optional(),
        tags: z.array(z.string()).default(['others']),
        ogImage: z.string().optional(),
        description: z.string(),
    })
    .strict();

const blog = defineCollection({
    schema: blogSchema,
});

export const collections = { blog };