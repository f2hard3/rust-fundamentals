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
    let numbers = [10, 13, 23, 2, 8, 9, 6];
    let evens: Vec<i32> = numbers
        .into_iter()
        .filter(|number| number % 2 == 0)
        .collect();
    println!("{evens:?}");
    println!("{numbers:?}");

    let evens: Vec<i32> = numbers
        .iter()
        .filter(|number| *number % 2 == 0)
        .copied()
        .collect();
    println!("{evens:?}");
    println!("{numbers:?}");

    let first_even = numbers.into_iter().find(|number| number % 2 == 0);
    println!("{:?}", first_even);

    let first_odd = numbers.into_iter().find(|number| number % 2 != 0);
    println!("{:?}", first_odd);

    let nothing = numbers.into_iter().find(|number| *number > 100);
    println!("{:?}", nothing);

    let last_even = numbers.into_iter().rfind(|number| number % 2 == 0);
    println!("{:?}", last_even);

    let last_odd = numbers.into_iter().rfind(|number| number % 2 != 0);
    println!("{:?}", last_odd);

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

    let good_channels: Vec<&TVChannel> = channels
        .iter()
        .filter(|channel| channel.channel_type == ChannelType::ProgrammingTutorials)
        .collect();
    println!("{good_channels:?}");

    let good_channels: Vec<String> = channels
        .iter()
        .filter(|channel| channel.channel_type == ChannelType::ProgrammingTutorials)
        .map(|channel| channel.name.clone())
        .collect();
    println!("{good_channels:?}");

    let good_channels: Vec<String> = channels
        .iter()
        .filter_map(|channel| {
            if channel.channel_type == ChannelType::ProgrammingTutorials {
                Some(channel.name.clone())
            } else {
                None
            }
        })
        .collect();
    println!("{good_channels:?}");

    let good_channel = channels
        .iter()
        .find(|channel| channel.channel_type == ChannelType::ProgrammingTutorials);
    println!("{}", good_channel.is_some());
    println!("{}", good_channel.is_none());

    match good_channel {
        Some(channel) => println!("{channel:?}"),
        None => println!("There was no Rust programming channel"),
    }
}
