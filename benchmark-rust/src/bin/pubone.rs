use async_nats::jetstream;
use bytes::Bytes;
use clap::Parser;
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("Connecting to NATS...");
    let client = match async_nats::connect("nats://localhost:4222").await {
        Ok(client) => client,
        Err(err) => {
            eprintln!("Error connecting to NATS: {}", err);
            return Err(err.into());
        }
    };
    let jetstream = jetstream::new(client.clone());
    println!("Connected to NATS.");

    // Create a dummy message payload
    let payload = Bytes::from(vec![b'A'; args.msg_size]);

    println!(
        "Starting benchmark: publishing {} messages of {} bytes to stream '{}' subject '{}'",
        args.num_messages, args.msg_size, args.stream, args.subject
    );

    let start_time = Instant::now();

    for _ in 0..args.num_messages {
        match jetstream
            .publish(args.subject.clone(), payload.clone())
            .await
        {
            Ok(ack_future) => {
                match ack_future.await {
                    Ok(_ack) => {
                        // println!("Acknowledged sequence number: {}", ack.sequence);
                    }
                    Err(err) => {
                        eprintln!("Error waiting for publish ack: {}", err);
                        return Err(err.into());
                    }
                }
            }
            Err(err) => {
                eprintln!("Error publishing message: {}", err);
                return Err(err.into());
            }
        }
    }

    if let Err(err) = client.flush().await {
        eprintln!("Error flushing client: {}", err);
        return Err(err.into());
    }

    let elapsed_time = start_time.elapsed();

    let total_bytes = (args.num_messages * args.msg_size) as f64;
    let messages_per_second = args.num_messages as f64 / elapsed_time.as_secs_f64();
    let megabytes_per_second = total_bytes / (1024.0 * 1024.0) / elapsed_time.as_secs_f64();

    println!("
Benchmark Results:");
    println!("  Total messages: {}", args.num_messages);
    println!("  Message size: {} bytes", args.msg_size);
    println!("  Total time: {:.2?} ", elapsed_time);
    println!("  Messages/sec: {:.2}", messages_per_second);
    println!("  MB/sec: {:.2}", megabytes_per_second);

    Ok(())
}
