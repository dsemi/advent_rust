use json;
use json::JsonValue;

fn walk(d: &JsonValue, f: fn(&JsonValue) -> bool) -> i32 {
    match d {
        _ if f(d) => 0,
        JsonValue::Number(_) => d.as_i32().unwrap(),
        JsonValue::Object(_) => d.entries().map(|x| walk(x.1, f)).sum(),
        JsonValue::Array(_) => d.members().map(|x| walk(x, f)).sum(),
        _ => 0,
    }
}

pub fn part1(input: &str) -> i32 {
    walk(&json::parse(input).unwrap(), |_| false)
}

pub fn part2(input: &str) -> i32 {
    walk(&json::parse(input).unwrap(), |v| {
        v.is_object() && v.entries().any(|x| x.1 == "red")
    })
}
