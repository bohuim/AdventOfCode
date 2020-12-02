mod InputReader;

enum EventType
{
    BeginShift,
    FallAsleep,
    WakeUp
}

struct Event
{
}

fn main()
{
    let lines: Vec<String> = InputReader::read_lines("day04.input");
}
