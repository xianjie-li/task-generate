use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
struct Task {
    label: String,
    probability: Option<f64>,
    rand_category: Option<Vec<Vec<TaskCateItem>>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TaskCateItem {
    label: String,
    ratio: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all(deserialize = "camelCase"))]
struct Config {
    rest_day_probability: f64,
    tasks: Vec<Task>,
}

pub fn gen() {
    let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    config_path.push("task.json");

    let path = config_path.to_str().unwrap();

    let Config {
        tasks,
        rest_day_probability,
    } = get_config(path);
    if !tasks.len() == 0 {
        panic!("tasks can't be empty");
    }

    let mut rng = rand::thread_rng();

    let rest_day_ratio: f64 = rng.gen();

    if rest_day_ratio < rest_day_probability {
        println!("{}", "今天是休息日, 好好休息下吧~");
        return;
    }

    let mut list: Vec<String> = Vec::new();

    // 过滤掉未匹配任务
    for task in tasks.iter() {
        match task.probability {
            Some(probability) => {
                let task_ratio: f64 = rng.gen();

                if task_ratio < probability {
                    list.push(handle_random_category(task, &mut rng));
                }
            }
            None => list.push(handle_random_category(task, &mut rng)),
        }
    }
    for text in list {
        println!("{}", text);
    }
}

/// 处理randCategory
fn handle_random_category(task: &Task, rng: &mut ThreadRng) -> String {
    match &task.rand_category {
        Some(category) => {
            if category.len() == 0 {
                return task.label.clone();
            }

            // let mut label = String::new();
            let mut ls: Vec<&String> = Vec::new();

            for cate in category.iter() {
                if cate.len() != 0 {
                    let mut base_ratio = 0f64;

                    for t in cate.iter() {
                        base_ratio += t.ratio;
                        let task_ratio: f64 = rng.gen();

                        if task_ratio < base_ratio {
                            ls.push(&t.label);
                            break;
                        }
                    }
                }
            }

            let mut s = task.label.clone();

            for (i, text) in ls.iter().enumerate() {
                let s1: String = format!("$category{}", i + 1);
                let s2: &str = s1.as_str();
                s = s.replace(s2, text);
            }

            s
        }
        None => task.label.clone(),
    }
}

/// 根据传入配置地址获取Config
fn get_config(path: &str) -> Config {
    let mut file = File::open(path).unwrap();
    let mut json_str = String::from("");

    file.read_to_string(&mut json_str).unwrap();

    let config: Config = serde_json::from_str(&json_str).unwrap();

    config
}
