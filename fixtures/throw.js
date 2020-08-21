const message = 'oh no';

function willThrowInTheFuture() {
  meThrow();
}

function meThrow() {
  throw new Error(message);
}

willThrowInTheFuture();
