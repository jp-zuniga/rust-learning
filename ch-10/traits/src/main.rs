// Traits! What are they‽

use traits::aggregator::Summary;
use traits::aggregator::news::NewsArticle;
use traits::aggregator::social::SocialPost;


fn main() {
    let post = SocialPost {
        username: String::from("horse_ebooks"),
        content: String::from("yes, of course, you'll only find people here!"),
        reply: false,
        repost: false,
    };

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("1 new social media post!\n{}", post.summarize());
    println!();
    println!("New article available!\n{}", article.summarize());
}
