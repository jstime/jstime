async function main() {
  const [first, second] = await Promise.all([
    Promise.resolve('hello'),
    Promise.resolve('world')
  ]);

  console.log(`${first} ${second}`);
}

main().catch(e => console.error(e));
