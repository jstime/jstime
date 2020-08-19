async function main() {
  await Promise.resolve();
  const first = 'hello';
  const second = 'world';

  console.log(`${first} ${second}`);
}

main().catch(e => console.error(e));
