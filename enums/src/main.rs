// Defining an Enum
// Enums are a way of defining custom data types in a different way than you do with structs.
// An enum value can only be on of its variants.
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// Enum Values
// Note that the variants of the enum are namespaced under its identifier, and we use a double colon to separate the two.
// This is useful because now both values IpAddrKind::V4 and IpAddrKind::V6 are of the same type: IpAddrKind. We can then,
// for instance, define a function that takes any IpAddrKind;
//      fn route(ip_kind: IpAddrKind) {}

// Using enums has even more advantages. Thinking more about our IP adress type, at the moment we don't have a way to store
// the actual IP address data; we only know what kind it is. We can solve this by using struct

// However, representing the same concept using just an enum is more concise: rather than an enum inside struct, we can put
// data directly into each enum variant. This new definition of the IpAddr enum says that both V4 and V6 variants will have
// associated String values:
enum IpAddrV2 {
    V4(String),
    V6(String),
}
// We attach data to each variant of the enum directly, so there is no need for an extra struct.Here it's also easier to see
// another detail of how enums work: the name of each enum variant that we define also becomes a function that constructs an
// instance of the enum. That is, `IpAddrV2::V4()` is a function call that takes a `String` argument and returns an instance
// of the `IpAddrV2` type.

// There''s another advantage to using an enum rather than a struct: each variant can have different types and amounts of
// associated data. Version four type IP addresses will always have four metric components that will have values between 0 and
// 255. If we wanted to store V4 adddresses as four u8 values but still express V6 addresses as one String value, we wouldn't
// be able to with a struct. Enums handle this case with ease:
enum IpAddrV3 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Standart library already has a definition for IP addresses we can use; let's take a look
// struct Ipv4Addr {
//      --snip--
// }
// struct Ipv6Addr {
//     --snip--
// }
// enum IpAddr {
//     V4(Ipv4Addr),
//     V6(Ipv6Addr),
// }
// This code illustrates that you can put any kind of data inside an enum variant: strings, numeric types, or structs..
// You can even include another enum!

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// this enum has four variants with different types:
// - Quit, has no data associated with it at all.
// - Move, has named fields like a struct does.
// - Write includes a single String.
// - ChangeColor includes three i32 values

// Defining an enum with variants such as the ones above is similar to defining different kinds of struct definitions, except
// the enum doesn't use the struct keyword and all the variants are grouped together under the Message type. The following
// structs could hold the same data that preceding enum variants hold:
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct

// But if we used the different structs, which each have their own type, we couldn't as easily define a function to take any
// of these kinds of messages as we could with the `Message` enum which is a single type

// There is one more similarity between enums and structs: just as we're able to define methods on structs using impl, we're
// also able define methods on enums. Here's a method named call that we could define on our Message enum:
impl Message {
    fn call(&self) {
        println!("called under {:?}", self);
    }
}

// The Option Enum and its Advantages Over Null Values
// The `Option` type encodes the very common scenario in which a value could be something or it could be nothing. For example,
// if you request the first of a list containing items, you would get a value. If you request the first item of an empty list,
// you would get nothing. Expressing this concept in terms of the type system means the compiler can check whether you've
// handled all the cases you should be handling; this functionality can prevent bugs that are extremely common in other
// programming languages.

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddrV2::V4(String::from("127.0.0.1"));
    let loopback = IpAddrV2::V6(String::from("::1"));

    let home = IpAddrV3::V4(127, 0, 0, 1);
    let loopback = IpAddrV3::V6(String::from("::1"));

    let message = Message::Write(String::from("hello"));
    message.call();
}
