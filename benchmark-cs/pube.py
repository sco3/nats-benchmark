import os 
os.environ["LOG_LEVEL"]="error"


import asyncio
import argparse
import time
from nats.aio.client import Client as NATS
from conversationstream.eventbus.conversation_stream_events import EventsBus


async def run(loop):
    parser = argparse.ArgumentParser()
    parser.add_argument("-s", "--stream", required=True)
    parser.add_argument("-sub", "--subject", required=True)
    parser.add_argument("--msg-size", type=int, required=True)
    parser.add_argument("--num-messages", type=int, required=True)
    args = parser.parse_args()
    event_bus: EventsBus = EventsBus()

    payload = ("*" * args.msg_size).encode("utf-8")

    start_time = time.time()
    await event_bus.connect()

    for _ in range(args.num_messages):
        await event_bus.nats_integration.publish_bytes(
            args.subject,
            payload,
            headers={},
            ensure_subject=True,
        )

    end_time = time.time()

    duration = end_time - start_time
    msgs_per_sec = args.num_messages / duration

    print(
        f"Published {args.num_messages} messages of size {args.msg_size} bytes to subject '{args.subject}' in {duration:.2f} seconds."
    )
    print(f"Rate: {msgs_per_sec:,.2f} msgs/sec")

    await nc.close()


if __name__ == "__main__":
    os.environ["LOG_LEVEL"]="error"
    os.environ["NATS_URL"] = "nats://localhost:4222"
    loop = asyncio.get_event_loop()
    loop.run_until_complete(run(loop))
