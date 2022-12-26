use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, near_bindgen, AccountId};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Clone)]
pub struct Post {
    id: u128,
    title: String,
    description: String,
    tags: Vec<String>,
    media: String,
    users_who_liked: Vec<AccountId>,
    owner_id: AccountId,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct SocialNetworking {
    posts: UnorderedMap<u128, Post>,
    number_of_posts: u128,
    likes_by_user: UnorderedMap<AccountId, Vec<Post>>,
    posts_by_tag: UnorderedMap<String, Vec<Post>>,
}

// Define the default, which automatically initializes the contract
impl Default for SocialNetworking {
    fn default() -> Self {
        Self {
            posts: UnorderedMap::new(b'm'),
            number_of_posts: 0,
            likes_by_user: UnorderedMap::new(b'n'),
            posts_by_tag: UnorderedMap::new(b'o'),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl SocialNetworking {
    //Helper function
    pub fn add_posts_by_tag(&mut self, post: Post, tags: Vec<String>) {
        let mut posts_for_tag: Vec<Post>;
        for tag in tags {
            if let None = self.posts_by_tag.get(&tag) {
                posts_for_tag = Vec::<Post>::new();
            } else {
                posts_for_tag = self.posts_by_tag.get(&tag).unwrap();
            }

            posts_for_tag.push(post.clone());
            self.posts_by_tag.insert(&tag, &posts_for_tag);
        }
    }

    pub fn add_post(
        &mut self,
        title: String,
        description: String,
        tags: String,
        media: String,
    ) -> Post {
        let tags_iterator = tags.split(",");
        let mut tags = Vec::<String>::new();
        for tag in tags_iterator {
            tags.push(tag.to_string());
        }
        let post = Post {
            id: self.number_of_posts,
            title: title,
            description: description,
            tags: tags.clone(),
            media: media,
            users_who_liked: Vec::<AccountId>::new(),
            owner_id: env::signer_account_id(),
        };

        self.posts.insert(&post.id, &post);
        self.number_of_posts += 1;
        self.add_posts_by_tag(post.clone(), tags);
        return post;
    }

    pub fn get_all_posts(&self) -> Vec<(u128, Post)> {
        return self.posts.to_vec();
    }
}
