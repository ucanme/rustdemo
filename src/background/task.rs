use std::num::ParseIntError;
use std::string::ParseError;
use tokio::sync::Mutex;
use std::sync::Arc;
use futures::future::join;
use text_colorizer::*;
use crate::error::error::CustomError;

pub struct Task {
    pub name: String,
}

impl Task {
    pub async fn run(&self) {
        // 使用 tokio::sync::Mutex 替代 std::sync::Mutex
        let vec: Arc<Mutex<Vec<u64>>> = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]));

        println!("Starting tasks for {}", self.name.as_str().red());

        // 使用 tokio::join! 同时等待多个任务完成
        let task1 = tokio::spawn(task1(vec.clone()));
        let task2 = tokio::spawn(task2(vec.clone()));

        let _ = tokio::join!(task1, task2);
        let mut x = "hello world".to_string();
         x = x.replace("world", "world2");
        println!("{}", x);

        let x = "1.0";
        let y : Result<i8,ParseIntError> = x.parse();
        match y {
            Ok(val)=>{
                println!("{}", val);
            }
            Err(e)=>{
               let X : CustomError =  e.into();
                println!("{}", X);
            }
        }

        eprintln!("All tasks completed for {}", self.name.as_str().red());
    }
}

async fn task1(vec: Arc<Mutex<Vec<u64>>>) {
    for i in 1..100 {
        // 减少锁的持有时间
        {
            let mut data = vec.lock().await;
            data[0] = i;
        }

        println!("Task 1: Updated value to {}", i);

        // 在锁外执行异步操作
      //  tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
}

async fn task2(vec: Arc<Mutex<Vec<u64>>>) {
    for i in 1..100 {
        // 减少锁的持有时间
        {
            let mut data = vec.lock().await;
            data[0] = i;
        }

        println!("Task 2: Updated value to {}", i);

        // 在锁外执行异步操作
        //tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
}