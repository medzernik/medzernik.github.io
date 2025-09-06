// src/posts.rs

use chrono::NaiveDate;
use gray_matter::{Matter, engine::YAML};
use include_dir::{Dir, include_dir};
use leptos::prelude::*;
use leptos::{IntoView, component};
use once_cell::sync::Lazy;
use pulldown_cmark::{Options, Parser, html};
use serde::{Deserialize, Serialize};
use stylance::import_crate_style;
import_crate_style!(style, "src/main.module.css");

// This is the directory where your markdown posts are stored.
static POST_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/public/content/posts");

#[component]
pub fn AllPostsPage() -> impl IntoView {
    leptos::logging::warn!("LOCATION: {POST_DIR:#?}");
    view! {
        <div class=style::bodyContainer>
            <h1 class=style::headerText>"# All Posts"</h1>
            <p class=style::bodyText>"Here you can find all my thoughts and articles."</p>

            // This list will be populated with your posts
            <ul class=style::bodyContainer>
                // The <For> component is used to iterate over a list
                <For
                    // `each`: Provides the list of items. We clone POSTS here.
                    each=|| POSTS.clone()
                    // `key`: A unique identifier for each item in the list.
                    // The slug is perfect for this.
                    key=|post| post.slug.clone()
                    // `children`: The template to render for each item.
                    // The `post` variable holds the data for one post.
                    children=|post: Post| {
                        view! {
                            <li class=style::articleButtonContainer>
                                // Create a link to the specific post's page
                                <a href=format!("/posts/{}", post.slug)>
                                    <p class=style::articleTitleButton>{post.metadata.title}</p>
                                </a>
                                <p class=style::articleDateButton>
                                    {post.metadata.date.to_string()}
                                </p>
                                <p class=style::articleDescriptionButton>
                                    {post.metadata.description}
                                </p>
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}
// Define the structure for a post's metadata (frontmatter).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PostMetadata {
    pub title: String,
    pub date: NaiveDate, // Using NaiveDate for simplicity
    pub description: String,
}

// Define the main structure for a Post.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub slug: String,
    pub metadata: PostMetadata,
    pub html: String,
}

// Use `once_cell` to load and parse posts only once at build time.
// This is the static list of all posts you'll use in your components.
pub static POSTS: Lazy<Vec<Post>> = Lazy::new(|| {
    let mut posts: Vec<Post> = POST_DIR
        .files()
        .filter_map(|file| {
            // Ensure we are only parsing markdown files
            if file.path().extension().and_then(|s| s.to_str()) != Some("md") {
                return None;
            }

            let slug = file
                .path()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string();

            let content = file.contents_utf8().unwrap();

            // Parse frontmatter and markdown content
            let matter = Matter::<YAML>::new();
            let result = matter.parse::<PostMetadata>(content).unwrap();
            leptos::logging::warn!("ITEMS: {result:#?}");

            let metadata: PostMetadata = result.data.unwrap_or_default();

            // Convert markdown body to HTML
            let mut options = Options::empty();
            options.insert(Options::ENABLE_STRIKETHROUGH);
            let parser = Parser::new_ext(&result.content, options);
            let mut html = String::new();
            html::push_html(&mut html, parser);

            Some(Post {
                slug,
                metadata,
                html,
            })
        })
        .collect();

    // Sort posts by date, newest first
    posts.sort_by(|a, b| b.metadata.date.cmp(&a.metadata.date));

    posts
});
