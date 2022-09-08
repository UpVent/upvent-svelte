// For Blog.svelte
export type Posts = {
    _embedded: any,
    slug: string,
    title: {
        rendered: string
    },
    excerpt: {
        rendered: string
    }
};


// For Post.svelte
export type Post = {
    _embedded: {
        "wp:featuredmedia": [{
            alt_text: string,
            source_url: string
        }],
    },
    title: {
        rendered: string
    },
    excerpt: {
        rendered: string
    },
    content: {
        rendered: string
    }
};