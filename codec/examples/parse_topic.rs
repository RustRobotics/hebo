use hebo_codec::topic::Topic;

fn main() {
    let t_sys = Topic::parse("$SYS/dev/cpu/+").unwrap();
    println!("t_sys: {t_sys:?}");
    assert!(t_sys.is_match("$SYS/dev/cpu/01"));
}
