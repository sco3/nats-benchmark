# NATS Publisher Benchmark (TypeScript)

This benchmark measures the performance of publishing messages to a NATS stream.

## Installation

Navigate to the `benchmark-ts` directory and install the dependencies:

```bash
npm install
```

## Usage

Use the `npm run pub` script to run the benchmark. You must provide the following parameters:

- `--stream` or `-s`: The NATS stream name.
- `--subject` or `-j`: The NATS subject name.
- `--msgSize` or `-m`: The message size in bytes.
- `--numMessages` or `-n`: The number of messages to publish.

### Example

```bash
npm run pub -- --stream my-stream --subject my-subject --msgSize 1024 --numMessages 10000
```
