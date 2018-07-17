trait Summary {
    fn summarize(&self) -> String;
}

trait Length {
    fn length(&self) -> usize;
}

struct Article {
    text: String,
    summary: String
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("Summary: {}", self.summary)
    }
}

impl Length for Article {
    fn length(&self) -> usize {
        return self.text.len()
    }
}

fn notify<T: Summary + Length> (item: T) {
    println!("News: {}, read full text(has length {})", item.summarize(), item.length())
}

fn main() {
    let a = Article {
        text: String::from("a first text"),
        summary: String::from("summary of text")
    };
    notify(a);
}

