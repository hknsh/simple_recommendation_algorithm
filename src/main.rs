use csv::Reader;
use serde::Deserialize;
use std::collections::HashSet;
use std::error::Error;

#[derive(Debug, Clone)]
struct Post {
    id: u32,
    title: String,
    tags: HashSet<String>,
}

#[derive(Debug, Clone)]
struct User {
    id: u32,
    username: String,
    preferences: HashSet<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct UserRecord {
    id: u32,
    username: String,
    preferences: String,
}

#[derive(Debug, Clone, Deserialize)]
struct PostRecord {
    id: u32,
    title: String,
    tags: String,
}

fn normalize_tag(tag: &str) -> String {
    tag.trim().to_lowercase().replace("_", "")
}

fn recommended_posts(user: &User, posts: &[Post], limit: usize) -> Vec<Post> {
    let mut recommended_posts: Vec<(Post, usize)> = posts
        .iter()
        .map(|post| {
            let score = post.tags.intersection(&user.preferences).count();
            (post.clone(), score)
        })
        .filter(|(_, score)| *score > 0)
        .collect();

    recommended_posts.sort_by(|a, b| b.1.cmp(&a.1));

    recommended_posts
        .into_iter()
        .take(limit)
        .map(|(post, _)| post)
        .collect()
}

fn recommended_users(current_user: &User, users: &[User], limit: usize) -> Vec<User> {
    let mut recommended_users: Vec<(User, usize)> = users
        .iter()
        .filter(|user| user.id != current_user.id)
        .map(|user| {
            let score = user
                .preferences
                .intersection(&current_user.preferences)
                .count();
            (user.clone(), score)
        })
        .filter(|(_, score)| *score > 0)
        .collect();

    recommended_users.sort_by(|a, b| b.1.cmp(&a.1));

    recommended_users
        .into_iter()
        .take(limit)
        .map(|(user, _)| user)
        .collect()
}

fn load_users_from_csv(path: &str) -> Result<Vec<User>, Box<dyn Error>> {
    let mut users = Vec::new();
    let mut reader = Reader::from_path(path)?;

    for result in reader.deserialize() {
        let record: UserRecord = result?;
        let preferences: HashSet<String> = record
            .preferences
            .split(',')
            .map(|tag| normalize_tag(tag))
            .collect();

        users.push(User {
            id: record.id,
            username: record.username,
            preferences,
        });
    }
    Ok(users)
}

fn load_posts_from_csv(path: &str) -> Result<Vec<Post>, Box<dyn Error>> {
    let mut posts = Vec::new();
    let mut reader = Reader::from_path(path)?;

    for result in reader.deserialize() {
        let record: PostRecord = result?;
        let tags: HashSet<String> = record
            .tags
            .split(",")
            .map(|tag| normalize_tag(tag))
            .collect();

        posts.push(Post {
            id: record.id,
            title: record.title,
            tags,
        });
    }
    Ok(posts)
}

fn main() -> Result<(), Box<dyn Error>> {
    let users = load_users_from_csv("./src/csv/users.csv")?;
    let posts = load_posts_from_csv("./src/csv/posts.csv")?;

    println!("\n========== Recommended users and posts ==========");
    for i in 0..15 {
        let user = &users[i];

        let recommended_users = recommended_users(user, &users, 5);
        let recommended_posts = recommended_posts(user, &posts, 5);

        println!("\nUser {} (ID: {})", user.username, user.id);
        println!("Preferences: {:?}", user.preferences);

        println!("Recommended users:");
        for (index, recommended_user) in recommended_users.iter().enumerate() {
            println!(
                "  {}. {} (ID: {}) - Preferences: {:?}",
                index + 1,
                recommended_user.username,
                recommended_user.id,
                recommended_user.preferences
            );
        }

        println!("Recommended posts:");
        for (index, post) in recommended_posts.iter().enumerate() {
            println!(
                "  {}. {} (ID: {}) - Tags: {:?}",
                index + 1,
                post.title,
                post.id,
                post.tags
            );
        }
        println!("\n------------------------------------------------------------");
    }

    Ok(())
}
