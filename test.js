function willThrowInTheFuture() {
  meThrow();
}

function meThrow() {
  throw new Error('me throwing so bad!');
}

willThrowInTheFuture();