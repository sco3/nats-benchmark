
import { connect, StringCodec, AckPolicy } from 'nats';
import yargs from 'yargs';
import { hideBin } from 'yargs/helpers';

async function main() {
  const argv = await yargs(hideBin(process.argv))
    .option('stream', {
      alias: 's',
      type: 'string',
      description: 'NATS stream name',
      demandOption: true,
    })
    .option('subject', {
      alias: 'j',
      type: 'string',
      description: 'NATS subject name',
      demandOption: true,
    })
    .option('msgSize', {
      alias: 'm',
      type: 'number',
      description: 'Message size in bytes',
      demandOption: true,
    })
    .option('numMessages', {
      alias: 'n',
      type: 'number',
      description: 'Number of messages to publish',
      demandOption: true,
    })
    .help()
    .argv;

  const { stream, subject, msgSize, numMessages } = argv;

  try {
    const nc = await connect({ servers: "nats://localhost:4222" });
    const js = nc.jetstream();
    const sc = StringCodec();

    const msg = 'a'.repeat(msgSize);
    
    console.log(`Publishing ${numMessages} messages of size ${msgSize} to stream "${stream}" on subject "${subject}"...`);

    const start = Date.now();

    for (let i = 0; i < numMessages; i++) {
      await js.publish(subject, sc.encode(msg));
    }

    const end = Date.now();
    const duration = end - start;

    console.log(`Published ${numMessages} messages in ${duration}ms`);
    console.log(`Rate: ${((numMessages / duration) * 1000).toFixed(2)} msgs/sec`);

    await nc.close();
  } catch (err) {
    console.error('Error connecting to or publishing from NATS', err);
  }
}

main();
