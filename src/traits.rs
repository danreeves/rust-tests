#[test]
fn traits() {
    trait Summary {
        fn summarise(&self) -> String;
    }

    fn get_summary<T: Summary>(item: T) -> String {
        format!("Summary: {}", item.summarise())
    }

    struct Tweet {
        text: String,
        author: String,
    }

    struct Article {
        headline: String,
        body: String,
        author: String,
    }

    impl Summary for Tweet {
        fn summarise(&self) -> String {
            format!("@{} says: {}", self.author, self.text)
        }
    }

    impl Summary for Article {
        fn summarise(&self) -> String {
            format!(
                "{} by {}: {}...",
                self.headline,
                self.author,
                self.body[..10].to_string()
            )
        }
    }

    let my_tweet = Tweet {
        text: "lol fart".to_string(),
        author: "dnrvs".to_string(),
    };

    assert_eq!(get_summary(my_tweet), "Summary: @dnrvs says: lol fart");

    let my_article = Article {
        headline: "Best tweet ever".to_string(),
        body: "Someone just made the best tweet ever but we can't show it to you".to_string(),
        author: "Dan Reeves".to_string(),
    };

    assert_eq!(
        get_summary(my_article),
        "Summary: Best tweet ever by Dan Reeves: Someone ju..."
    );
}
