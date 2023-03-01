use std::env;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use s3::error::S3Error;
use s3::{Bucket, Region};
use s3::creds::Credentials;

pub fn pull_down_data_minio() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            start_pull_down_data_minio().await.expect("Failed pulling down");
        })
}

async fn start_pull_down_data_minio() -> Result<(), S3Error> {

    let bucket = Bucket::new(
        &*env::var("MINIO_BUCKET_NAME").unwrap(),
        Region::Custom {
            region: env::var("MINIO_REGION").unwrap(),
            endpoint: env::var("MINIO_ENDPOINT").unwrap(),
        },
        Credentials::new(
            Option::from(&*env::var("MINIO_ACCESS_KEY").unwrap()),
            Option::from(&*env::var("MINIO_SECRET_KEY").unwrap()),
            Option::from(&*env::var("MINIO_SECURITY_TOKEN").unwrap()),
            Option::from(&*env::var("MINIO_SESSION_TOKEN").unwrap()),
            Option::from(&*env::var("MINIO_PROFILE").unwrap()),
        )?,
    )?
        .with_path_style();

    let s3_path = &*env::var("MINIO_FILE_NAME").unwrap();

    let response_data = bucket.get_object(s3_path).await?;
    assert_eq!(response_data.status_code(), 200);
    //assert_eq!(test, response_data.as_slice());
    //println!("{:?}", response_data.bytes());

    let path = &*env::var("MINIO_FILE_OUT_NAME").unwrap();

    let mut buffer = File::create(path).unwrap();

    buffer.write_all(response_data.bytes()).unwrap();

    Ok(())
}

pub fn push_data_minio() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            start_push_data_minio().await.expect("Failed push");
        })
}

async fn start_push_data_minio() -> Result<(), S3Error> {

    let bucket = Bucket::new(
        &*env::var("MINIO_BUCKET_NAME").unwrap(),
        Region::Custom {
            region: env::var("MINIO_REGION").unwrap(),
            endpoint: env::var("MINIO_ENDPOINT").unwrap(),
        },
        Credentials::new(
            Option::from(&*env::var("MINIO_ACCESS_KEY").unwrap()),
            Option::from(&*env::var("MINIO_SECRET_KEY").unwrap()),
            Option::from(&*env::var("MINIO_SECURITY_TOKEN").unwrap()),
            Option::from(&*env::var("MINIO_SESSION_TOKEN").unwrap()),
            Option::from(&*env::var("MINIO_PROFILE").unwrap()),
        )?,
    )?
        .with_path_style();

    let path = &*env::var("MINIO_FILE_OUT_NAME").unwrap();

    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;

    let s3_path = &*env::var("MINIO_FILE_NAME").unwrap();

    bucket.put_object(s3_path, &*buffer).await.expect("failed push");

    Ok(())
}