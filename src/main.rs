use rand::Rng;
use task_generate::{get_config, get_path};

fn main() {
    let path = get_path("task.json");
    let path = path.as_str();

    let config = get_config(path);

    task_generate::gen(config);
}

#[test]
fn avg_test() {
    let mut rng = rand::thread_rng();

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    let mut list3: Vec<i32> = Vec::new();
    let mut list4: Vec<i32> = Vec::new();

    for i in 0..1000000 {
        let task_ratio: f64 = rng.gen();

        if task_ratio < 0.35 {
            list1.push(i);
        } else if task_ratio < 0.6 {
            list2.push(i);
        } else if task_ratio < 0.8 {
            list3.push(i);
        } else {
            list4.push(i);
        }
    }

    println!("list1: {}", list1.len());
    println!("list2: {}", list2.len());
    println!("list3: {}", list3.len());
    println!("list4: {}", list4.len());

    let ratio1 = list1.len() as f64 / 1000000f64;
    let ratio2 = list2.len() as f64 / 1000000f64;
    let ratio3 = list3.len() as f64 / 1000000f64;
    let ratio4 = list4.len() as f64 / 1000000f64;

    println!("list1 ratio: {}", ratio1);
    println!("list2 ratio: {}", ratio2);
    println!("list3 ratio: {}", ratio3);
    println!("list4 ratio: {}", ratio4);
}
