use serde_json::Value;

fn walk(d: &Value, f: fn(&Value) -> bool) -> i64 {
    match d {
        _ if f(d) => 0,
        Value::Number(n) => n.as_i64().unwrap(),
        Value::Object(o) => o.iter().map(|x| walk(x.1, f)).sum(),
        Value::Array(a) => a.iter().map(|x| walk(x, f)).sum(),
        _ => 0,
    }
}

pub fn part1(input: &str) -> i64 {
    walk(&serde_json::from_str(input).unwrap(), |_| false)
}

pub fn part2(input: &str) -> i64 {
    walk(&serde_json::from_str(input).unwrap(), |v| {
        v.as_object().map(|o| o.values().any(|x| x == "red")).unwrap_or(false)
    })
}
