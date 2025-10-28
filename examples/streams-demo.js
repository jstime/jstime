// Streams API Demo
// This example demonstrates the WHATWG Streams API implementation in jstime

console.log("=== Streams API Demo ===\n");

// 1. ReadableStream - For reading data chunks
console.log("1. ReadableStream Example:");
console.log("Creating a readable stream with data chunks...");

const readable = new ReadableStream({
  start(controller) {
    console.log("  - Enqueuing 'Hello'");
    controller.enqueue("Hello");
    console.log("  - Enqueuing 'World'");
    controller.enqueue("World");
    console.log("  - Closing stream");
    controller.close();
  }
});

const reader = readable.getReader();

console.log("  - Stream is locked:", readable.locked);
console.log("  - Reading chunks...");

reader.read().then(result => {
  console.log("  - Chunk 1:", result.value, "(done:", result.done + ")");
  return reader.read();
}).then(result => {
  console.log("  - Chunk 2:", result.value, "(done:", result.done + ")");
  return reader.read();
}).then(result => {
  console.log("  - Stream closed (done:", result.done + ")");
});

// 2. WritableStream - For writing data chunks
console.log("\n2. WritableStream Example:");
console.log("Creating a writable stream that collects data...");

const chunks = [];
const writable = new WritableStream({
  write(chunk) {
    console.log("  - Writing:", chunk);
    chunks.push(chunk);
  },
  close() {
    console.log("  - Stream closed");
    console.log("  - All chunks:", chunks.join(", "));
  }
});

const writer = writable.getWriter();

console.log("  - Stream is locked:", writable.locked);

writer.write("First")
  .then(() => writer.write("Second"))
  .then(() => writer.write("Third"))
  .then(() => writer.close());

// 3. TransformStream - For transforming data
console.log("\n3. TransformStream Example:");
console.log("Creating a transform stream to uppercase text...");

const transform = new TransformStream({
  transform(chunk, controller) {
    const transformed = chunk.toUpperCase();
    console.log("  - Transforming:", chunk, "->", transformed);
    controller.enqueue(transformed);
  }
});

console.log("  - Has readable side:", transform.readable instanceof ReadableStream);
console.log("  - Has writable side:", transform.writable instanceof WritableStream);

const transformWriter = transform.writable.getWriter();
const transformReader = transform.readable.getReader();

transformWriter.write("hello");
transformWriter.write("streams");
transformWriter.close();

transformReader.read().then(result => {
  console.log("  - Transformed 1:", result.value);
  return transformReader.read();
}).then(result => {
  console.log("  - Transformed 2:", result.value);
});

// 4. Chaining streams with pipeTo (conceptual example)
console.log("\n4. Response.body as ReadableStream:");
console.log("The Fetch API Response.body returns a ReadableStream:");

const response = new Response("Example response body data");

console.log("  - Response.body is a ReadableStream:", response.body instanceof ReadableStream);

const bodyReader = response.body.getReader();

bodyReader.read().then(result => {
  console.log("  - Body content:", result.value);
  console.log("  - Stream done:", result.done);
});

// 5. Stream with error handling
console.log("\n5. Stream Error Handling:");

const errorStream = new ReadableStream({
  start(controller) {
    try {
      controller.enqueue("Valid data");
      console.log("  - Enqueued valid data");
      controller.error(new Error("Stream error!"));
      console.log("  - Stream errored");
    } catch (e) {
      console.log("  - Caught error:", e.message);
    }
  }
});

console.log("  - Error stream created");

// 6. Complex transformation pipeline
console.log("\n6. Complex Pipeline Example:");
console.log("Creating a pipeline: Read -> Transform -> Collect");

const pipeline = new TransformStream({
  transform(chunk, controller) {
    // Add prefix and suffix to each chunk
    const transformed = `[${chunk}]`;
    controller.enqueue(transformed);
  }
});

const collected = [];
const pipelineWriter = pipeline.writable.getWriter();
const pipelineReader = pipeline.readable.getReader();

// Write data
pipelineWriter.write("A");
pipelineWriter.write("B");
pipelineWriter.write("C");
pipelineWriter.close();

// Read and collect
function readAll() {
  return pipelineReader.read().then(result => {
    if (result.done) {
      console.log("  - Pipeline complete! Collected:", collected.join(", "));
      return;
    }
    collected.push(result.value);
    return readAll();
  });
}

readAll();

console.log("\n=== Demo Complete ===");
console.log("\nNote: All stream operations are asynchronous and use Promises.");
console.log("The Streams API provides a standard way to handle streaming data.");
