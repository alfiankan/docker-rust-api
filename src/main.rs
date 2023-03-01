use std::env;
use crate::manager::manager::run_pipe;

mod manager;
mod api;
mod sourcers;
mod trigger;
mod puller;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args[1] == String::from("pull") {

        match env::var("MINIO_ENDPOINT") {
            Ok(_) => {
                println!("{}", "Pulling down from s3");
                puller::minio::pull_down_data_minio();
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }

    }

    if args[1] == String::from("push") {
        println!("{}", "push to s3");
        puller::minio::push_data_minio();

    }

    if args[1] == String::from("run-pipe") {
        println!("{}", "Running PIPE");
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                run_pipe().await;
            })
    }
}
