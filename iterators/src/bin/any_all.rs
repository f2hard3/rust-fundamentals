#[derive(Debug, PartialEq, Eq)]
enum ChannelType {
    Comedy,
    News,
    ProgrammingTutorials,
}

#[derive(Debug)]
struct TVChannel {
    name: String,
    channel_type: ChannelType,
}
fn main() {
    let channels = [
        TVChannel {
            name: String::from("CBS"),
            channel_type: ChannelType::Comedy,
        },
        TVChannel {
            name: String::from("RustLive"),
            channel_type: ChannelType::ProgrammingTutorials,
        },
        TVChannel {
            name: String::from("NBC"),
            channel_type: ChannelType::News,
        },
        TVChannel {
            name: String::from("RustTv"),
            channel_type: ChannelType::ProgrammingTutorials,
        },
    ];

    let all_are_rust = channels
        .iter()
        .all(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);
    println!("all_are_rust: {all_are_rust}");

    let any_is_rust = channels
        .iter()
        .any(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);
    println!("any_is_rust: {any_is_rust}");
}
