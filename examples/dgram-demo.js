// jstime dgram API Demo
// This example demonstrates UDP socket functionality using the Node.js-compatible dgram module

import dgram from 'node:dgram';

console.log('=== jstime dgram (UDP) API Demo ===\n');

// Create a UDP socket
console.log('1. Creating UDP sockets...');
const server = dgram.createSocket('udp4');
const client = dgram.createSocket('udp4');

// Set up server event handlers
server.on('error', (err) => {
  console.error(`Server error: ${err.message}`);
  server.close();
});

server.on('message', (msg, rinfo) => {
  console.log(`\n   Server received: "${new TextDecoder().decode(msg)}" from ${rinfo.address}:${rinfo.port}`);
  
  // Echo the message back
  const response = `Echo: ${new TextDecoder().decode(msg)}`;
  server.send(response, rinfo.port, rinfo.address, (err) => {
    if (err) console.error(`   Error sending response: ${err.message}`);
    else console.log(`   Server sent response: "${response}"`);
  });
});

server.on('listening', () => {
  const address = server.address();
  console.log(`   Server listening on ${address.address}:${address.port}`);
  
  // Now send some messages from the client
  sendMessages();
});

// Bind server to a random port
server.bind(0);

// Send test messages
function sendMessages() {
  const serverAddr = server.address();
  
  console.log('\n2. Sending messages from client...');
  
  // Send string message
  client.send('Hello, UDP!', serverAddr.port, '127.0.0.1', (err) => {
    if (err) console.error(`   Error: ${err.message}`);
    else console.log('   Client sent: "Hello, UDP!"');
  });
  
  // Send buffer message
  setTimeout(() => {
    const buffer = new TextEncoder().encode('Binary message');
    client.send(buffer, 0, buffer.length, serverAddr.port, '127.0.0.1', (err) => {
      if (err) console.error(`   Error: ${err.message}`);
      else console.log('   Client sent binary: "Binary message"');
    });
  }, 100);
  
  // Cleanup after tests
  setTimeout(() => {
    console.log('\n3. Closing sockets...');
    client.close(() => console.log('   Client socket closed'));
    server.close(() => console.log('   Server socket closed'));
    console.log('\n=== Demo complete ===');
  }, 500);
}

// Demonstrate socket options
console.log('\n4. Socket configuration options:');
const optionsSocket = dgram.createSocket({
  type: 'udp4',
  recvBufferSize: 1024 * 64,
  sendBufferSize: 1024 * 64
});

optionsSocket.bind(0, () => {
  const addr = optionsSocket.address();
  console.log(`   Options socket bound to port ${addr.port}`);
  console.log(`   Receive buffer size: ${optionsSocket.getRecvBufferSize()}`);
  console.log(`   Send buffer size: ${optionsSocket.getSendBufferSize()}`);
  
  // Set broadcast option
  optionsSocket.setBroadcast(true);
  console.log('   Broadcast enabled');
  
  // Set TTL
  optionsSocket.setTTL(64);
  console.log('   TTL set to 64');
  
  optionsSocket.close();
});
