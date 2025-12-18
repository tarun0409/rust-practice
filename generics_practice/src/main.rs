#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}

#[derive(Debug)]
struct ChatMessage<T> {
    content: T,
    time: String,
}

impl ChatMessage<DigitalContent> {
    fn consume_entertainment(&self) {
        println!("Hey check this out: {:?}", self.content);
    }
}
impl<T> ChatMessage<T> {
    fn retrieve_time(&self) -> String {
        return self.time.clone();
    }
}

fn main() {
    let x = ChatMessage {
        content: "Hello",
        time: String::from("9 AM"),
    };
    let y = ChatMessage {
        content: String::from("Hello but cooler"),
        time: String::from("10 AM"),
    };
    let z = ChatMessage {
        content: DigitalContent::VideoFile,
        time: String::from("11 AM"),
    };
    z.consume_entertainment();
    println!("{} === {} === {}", x.retrieve_time(), y.retrieve_time(), z.retrieve_time());

}
