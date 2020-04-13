use std::fmt::Debug;

use serde::{Deserialize, Serialize};

impl Unmarshal for RedditSite {}

impl Unmarshal for Comments {}

pub trait Unmarshal {
    fn new<'de>(data: &'de str) -> Self
        where
            Self: Deserialize<'de> {

        let serialized_data: Self = serde_json::from_str(&data).unwrap();
        serialized_data
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ChildrenData {
    pub id: String,
    pub body: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Children {
    pub kind: String,
    pub data: ChildrenData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingData {
    pub children: Vec<Children>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Listing {
    pub data: ListingData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Comments {
    pub link: RedditSite,
    pub comments: Listing,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub title: String,
    pub name: String,
    pub author: String,
    pub subreddit: String,
    pub ups: i32,
    pub permalink: String,
    pub url: String,
    pub selftext: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Links {
    pub children: Vec<Post>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RedditSite {
    pub data: Links,
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marshal_subreddit() {
        let data = r#"{
            "kind": "Listing", "data": {"modhash": "", "dist": 1, "children": [{"kind": "t3", "data": {"approved_at_utc": null, "subreddit": "learnrust", "selftext": "I have a cargo project with 3 files\n\n     src/\n        helpers.rs\n        lib.rs\n        wallet.rs\n\nIn my lib.rs file I have\n\n    mod helpers;\n    mod wallet;\n    \n    pub fn hey() {\n        println(\"{:?}\", helpers::hello());\n        println(\"{:?}\", wallet::hello());\n    }\n\nhelpers.rs\n\n    pub fn hello() -&gt; String {\n        \"hello\".to_string()\n    }\n\nwallet.rs\n\n    pub fn amount() -&gt; u8 {\n        5\n    }\n\nThis works. Now if I want to use helpers.rs in wallet.rs I would logically do the following:\n\n    mod helpers;\n    \n    pub fn amount() -&gt; u8 {\n        println(\"{:?}\", helpers::hello());\n        5\n    }\n\nHowever this won't work. \n\nIt is like I can use mod only once per file. \n\nWhy can I use mod &lt;filename&gt; only once in the project ?", "author_fullname": "t2_lv2beia", "saved": false, "mod_reason_title": null, "gilded": 0, "clicked": false, "title": "Confused about how to use different files using mod &lt;filename&gt;", "link_flair_richtext": [], "subreddit_name_prefixed": "r/learnrust", "hidden": false, "pwls": null, "link_flair_css_class": null, "downs": 0, "hide_score": true, "name": "t3_fec7gm", "quarantine": false, "link_flair_text_color": "dark", "author_flair_background_color": null, "subreddit_type": "public", "ups": 2, "total_awards_received": 0, "media_embed": {}, "author_flair_template_id": null, "is_original_content": false, "user_reports": [], "secure_media": null, "is_reddit_media_domain": false, "is_meta": false, "category": null, "secure_media_embed": {}, "link_flair_text": null, "can_mod_post": false, "score": 2, "approved_by": null, "author_premium": false, "thumbnail": "", "edited": false, "author_flair_css_class": null, "author_flair_richtext": [], "gildings": {}, "content_categories": null, "is_self": true, "mod_note": null, "created": 1583523145.0, "link_flair_type": "text", "wls": null, "removed_by_category": null, "banned_by": null, "author_flair_type": "text", "domain": "self.learnrust", "allow_live_comments": false, "selftext_html": "&lt;!-- SC_OFF --&gt;&lt;div class=\"md\"&gt;&lt;p&gt;I have a cargo project with 3 files&lt;/p&gt;\n\n&lt;pre&gt;&lt;code&gt; src/\n    helpers.rs\n    lib.rs\n    wallet.rs\n&lt;/code&gt;&lt;/pre&gt;\n\n&lt;p&gt;In my lib.rs file I have&lt;/p&gt;\n\n&lt;pre&gt;&lt;code&gt;mod helpers;\nmod wallet;\n\npub fn hey() {\n    println(&amp;quot;{:?}&amp;quot;, helpers::hello());\n    println(&amp;quot;{:?}&amp;quot;, wallet::hello());\n}\n&lt;/code&gt;&lt;/pre&gt;\n\n&lt;p&gt;helpers.rs&lt;/p&gt;\n\n&lt;pre&gt;&lt;code&gt;pub fn hello() -&amp;gt; String {\n    &amp;quot;hello&amp;quot;.to_string()\n}\n&lt;/code&gt;&lt;/pre&gt;\n\n&lt;p&gt;wallet.rs&lt;/p&gt;\n\n&lt;pre&gt;&lt;code&gt;pub fn amount() -&amp;gt; u8 {\n    5\n}\n&lt;/code&gt;&lt;/pre&gt;\n\n&lt;p&gt;This works. Now if I want to use helpers.rs in wallet.rs I would logically do the following:&lt;/p&gt;\n\n&lt;pre&gt;&lt;code&gt;mod helpers;\n\npub fn amount() -&amp;gt; u8 {\n    println(&amp;quot;{:?}&amp;quot;, helpers::hello());\n    5\n}\n&lt;/code&gt;&lt;/pre&gt;\n\n&lt;p&gt;However this won&amp;#39;t work. &lt;/p&gt;\n\n&lt;p&gt;It is like I can use mod only once per file. &lt;/p&gt;\n\n&lt;p&gt;Why can I use mod &amp;lt;filename&amp;gt; only once in the project ?&lt;/p&gt;\n&lt;/div&gt;&lt;!-- SC_ON --&gt;", "likes": null, "suggested_sort": null, "banned_at_utc": null, "view_count": null, "archived": false, "no_follow": true, "is_crosspostable": false, "pinned": false, "over_18": false, "all_awardings": [], "awarders": [], "media_only": false, "can_gild": false, "spoiler": false, "locked": false, "author_flair_text": null, "visited": false, "removed_by": null, "num_reports": null, "distinguished": null, "subreddit_id": "t5_3amtr", "mod_reason_by": null, "removal_reason": null, "link_flair_background_color": "", "id": "fec7gm", "is_robot_indexable": true, "report_reasons": null, "author": "_TheBatzOne_", "discussion_type": null, "num_comments": 2, "send_replies": true, "whitelist_status": null, "contest_mode": false, "mod_reports": [], "author_patreon_flair": false, "author_flair_text_color": null, "permalink": "/r/learnrust/comments/fec7gm/confused_about_how_to_use_different_files_using/", "parent_whitelist_status": null, "stickied": false, "url": "https://www.reddit.com/r/learnrust/comments/fec7gm/confused_about_how_to_use_different_files_using/", "subreddit_subscribers": 3164, "created_utc": 1583494345.0, "num_crossposts": 0, "media": null, "is_video": false}}], "after": "t3_fec7gm", "before": null}
        }"#;
        let deserialized_data = RedditSite::new(&data).data.children;
        assert_eq!(deserialized_data[0].data.subreddit, "learnrust");
    }

    #[test]
    #[should_panic]
    fn test_marshaling_false_body_fails() {
        let data = r#"{kind": "Listing", "data": {"modhash": "", "dist": 1, "FOO": []}"#;
        RedditSite::new(&data);
    }

    #[test]
    fn test_comment() {
        let listing = RedditSite {
            data: Links {
                children: vec![Post {
                    data: Data {
                        title: String::from("Title"),
                        name: String::from("Name"),
                        author: String::from("Author"),
                        subreddit: String::from("Subreddit"),
                        ups: 2,
                        permalink: String::from("Permalink"),
                    },
                }],
            },
        };

        assert_eq!(listing.data.children[0].data.title, "Title");
    }


    #[test]
    fn test_comment_data() {
        let listing = RedditSite {
            data: Links {
                children: vec![Post {
                    data: Data {
                        title: String::from("Title"),
                        name: String::from("Name"),
                        author: String::from("Author"),
                        subreddit: String::from("Subreddit"),
                        ups: 2,
                        permalink: String::from("Permalink"),
                    },
                }],
            },
        };

        let comments = Comments {
            link: listing,
            comments: Listing {
                data: ListingData {
                    children: vec![Children {
                        kind: String::from("Comment-kind"),
                        data: ChildrenData {
                            id: String::from("Comment_id"),
                            body: None,
                        },
                    }],
                },
            },
        };

        assert_eq!(comments.comments.data.children[0].data.id, "Comment_id")
    }
}
