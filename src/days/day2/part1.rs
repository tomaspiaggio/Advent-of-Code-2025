#[warn(dead_code)]
pub(crate) fn mock_data() -> Vec<(i64, i64)> {
    let data = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    data.split(",").map(|x| {
        let mut split = x.split("-");
        let left: i64 = split.next().unwrap().parse().expect("not a number");
        let right: i64 = split.next().unwrap().parse().expect("not a number");
        (left, right)
    }).collect()
}

pub(crate) fn data() -> Vec<(i64, i64)> {
    let data = "5542145-5582046,243-401,884211-917063,1174-1665,767028-791710,308275-370459,285243789-285316649,3303028-3361832,793080-871112,82187-123398,7788-14096,21-34,33187450-33443224,2750031-2956556,19974-42168,37655953-37738891,1759-2640,55544-75026,9938140738-9938223673,965895186-966026269,502675-625082,11041548-11204207,1-20,3679-7591,8642243-8776142,40-88,2872703083-2872760877,532-998,211488-230593,3088932-3236371,442734-459620,8484829519-8484873271,5859767462-5859911897,9987328-10008767,656641-673714,262248430-262271846";
    data.split(",").map(|x| {
        let mut split = x.split("-");
        let left: i64 = split.next().unwrap().parse().expect("not a number");
        let right: i64 = split.next().unwrap().parse().expect("not a number");
        (left, right)
    }).collect()
}

fn is_invalid(num: i64) -> bool {
    let stringified = format!("{}", num);
    if stringified.len() % 2 == 1 {
        return false
    }

    let splitted: Vec<&str> = stringified.split("")
        .filter(|x| *x != "")
        .collect::<Vec<&str>>();
    let length = splitted.len();
    let half_length = length / 2;

    for i in 0..half_length {
        if (splitted[i] != splitted[i + half_length]) {
            return false
        }
    }

    true
}

pub fn run(feeder: Option<fn() -> Vec<(i64, i64)>>) -> i64 {
    let data = feeder.map(|x| x())
        .unwrap_or_else(|| data());

    let mut sum = 0;

    for (left, right) in data {
        println!("About to test {}-{}", left, right);
        for i in left..=right {
            let result = is_invalid(i);
            if result {
                println!("Invalid ID: {}", i);
                sum += i;
            }
        }
    }

    println!("Result: {}", sum);

    sum
}