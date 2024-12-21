# Generics Example in Rust

This code demonstrates the use of generics in Rust.  It defines a `ChatMessage` struct that can hold any type of content, alongside a timestamp.

Key features:

* **`DigitalContent` enum:** Represents different types of media (AudioFile, VideoFile).
* **`ChatMessage<T>` struct:** A generic struct to store chat messages with content of type `T` and a timestamp.
* **`consume_entertainment()` method:**  Specific to `ChatMessage` instances holding `DigitalContent`, simulates consuming the media.
* **`retrieve_time()` method:**  Generic method available for all `ChatMessage` types, returns the message timestamp.

The `main` function creates instances of `ChatMessage` with different content types (String and `DigitalContent`) and demonstrates the use of the defined methods.
