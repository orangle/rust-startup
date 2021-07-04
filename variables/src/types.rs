enum ConnectionState {
    Init,
    SyncReceived(HalfOpen),
    SyncAckSent(HalfOpen),
    AckRecieved(FullSession),
}

struct HalfOpen{}

struct FullSession{}

#[derive(Clone, Debug)]
enum Gender {
    Unknown = 0,
    Male = 1,
    Female = 2,
}

#[derive(Clone, Debug)]
struct User { 
    name: String, 
    age: u8,
    gender: Gender
}