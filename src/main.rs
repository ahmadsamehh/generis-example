// Define an enum representing different types of digital content.
#[derive(Debug)]
enum DigitalContent {
    AudioFile,
    VideoFile,
}

// Define a struct representing a chat message with generic content and a timestamp.
#[derive(Debug)]
struct ChatMessage<T> {
    content: T,   // The content of the message, which can be of any type T.
    time: String, // The time the message was sent.
}

// Implement methods for ChatMessage where the content is DigitalContent.
impl ChatMessage<DigitalContent> {
    // Define a method to "consume" digital content (e.g., play audio or video).
    fn consume_entertainment(&self) {
        println!("Watching {:?}", self.content);
    }
}

// Implement methods for ChatMessage with any type of content.
impl<T> ChatMessage<T> {
    // Define a method to retrieve the time the message was sent.
    fn retrieve_time(&self) -> String {
        self.time.clone()
    }
}

fn main() {
    // Create a ChatMessage with a String content.
    let message = ChatMessage {
        content: String::from("Hello Khallody"),
        time: String::from("12:00"),
    };

    // Create a ChatMessage with DigitalContent.
    let message2 = ChatMessage {
        content: DigitalContent::AudioFile,
        time: String::from("12:00"),
    };

    // Print both messages using the Debug format.
    println!("{:?}", message);
    println!("{:?}", message2);

    // Retrieve and print the time of the first message.
    println!("{}", message.retrieve_time());

    // Consume the entertainment content of the second message.
    message2.consume_entertainment();
}
