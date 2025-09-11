use async_nats::jetstream;
use bytes::Bytes;
use clap::Parser;
use futures::future;

use std::time::Instant;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The JetStream stream name
    #[arg(long)]
    stream: String,

    /// The subject to publish messages to
    #[arg(long)]
    subject: String,

    /// Size of each message in bytes
    #[arg(long)]
    msg_size: usize,

    /// Number of messages to publish
    #[arg(long)]
    num_messages: usize,

    /// Maximum number of concurrent pushes
    #[arg(long, default_value_t = 500)]
    concurrency: usize,
}

#[tokio::main]
async fn main() -> Result<(), async_nats::Error> {
    let args = Args::parse();

    println!("Connecting to NATS...");
    let client = async_nats::connect("nats://localhost:4222").await?;
    let jetstream = jetstream::new(client);
    //let jetstream_clone = jetstream.clone();
    println!("Connected to NATS.");

    // Create a dummy message payload
    let payload = Bytes::from(vec![b'A'; args.msg_size]);

    println!(
        "Starting benchmark: publishing {} messages of {} bytes to stream '{}' subject '{}'",
        args.num_messages, args.msg_size, args.stream, args.subject
    );

    let start_time = Instant::now();

    let num_messages = args.num_messages;
    let concurrency = args.concurrency;

    for i in (0..num_messages).step_by(concurrency) {
        let mut tasks = Vec::new();
        let end = std::cmp::min(i + concurrency, num_messages);
        for _ in i..end {
            let jetstream = jetstream.clone();
            let payload = payload.clone();
            let subject = args.subject.clone();

            tasks.push(tokio::spawn(async move {
                jetstream.publish(subject, payload).await?.await
            }));
        }
        future::try_join_all(tasks).await?;
    }

    let elapsed_time = start_time.elapsed();

    let total_bytes = (args.num_messages * args.msg_size) as f64;
    let messages_per_second = args.num_messages as f64 / elapsed_time.as_secs_f64();
    let megabytes_per_second = total_bytes / (1024.0 * 1024.0) / elapsed_time.as_secs_f64();

    println!("\nBenchmark Results:");
    println!("  Total messages: {}", args.num_messages);
    println!("  Message size: {} bytes", args.msg_size);
    println!("  Total time: {:.2?} ", elapsed_time);
    println!("  Messages/sec: {:.2}", messages_per_second);
    println!("  MB/sec: {:.2}", megabytes_per_second);
    println!("  Concurrency: {:.2}", args.concurrency);

    Ok(())
}
