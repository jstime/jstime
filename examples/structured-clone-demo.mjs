// Structured Clone API Demo for jstime
// This example demonstrates deep cloning with structuredClone()

console.log('=== jstime Structured Clone API Demo ===\n');

// 1. Basic cloning
console.log('1. Basic cloning:');
const simple = { a: 1, b: 'hello', c: true };
const clonedSimple = structuredClone(simple);

console.log('   Original:', simple);
console.log('   Cloned:', clonedSimple);
console.log('   Are different objects:', simple !== clonedSimple);
console.log('   Have same values:', simple.a === clonedSimple.a);
console.log();

// 2. Deep cloning nested objects
console.log('2. Deep cloning nested objects:');
const nested = {
  user: {
    name: 'Alice',
    address: {
      city: 'New York',
      zip: '10001'
    }
  }
};

const clonedNested = structuredClone(nested);
clonedNested.user.address.city = 'Boston';

console.log('   Original city:', nested.user.address.city);
console.log('   Cloned city:', clonedNested.user.address.city);
console.log('   ✓ Original was not modified');
console.log();

// 3. Cloning arrays
console.log('3. Cloning arrays:');
const arr = [1, 2, { x: 3, y: 4 }, [5, 6]];
const clonedArr = structuredClone(arr);

clonedArr[2].x = 999;
clonedArr[3][0] = 888;

console.log('   Original array:', JSON.stringify(arr));
console.log('   Cloned array:', JSON.stringify(clonedArr));
console.log('   ✓ Deep clone preserves nested structure');
console.log();

// 4. Cloning Date objects
console.log('4. Cloning Date objects:');
const date = new Date('2024-01-01T12:00:00Z');
const clonedDate = structuredClone(date);

console.log('   Original date:', date.toISOString());
console.log('   Cloned date:', clonedDate.toISOString());
console.log('   Is Date object:', clonedDate instanceof Date);
console.log('   Are different objects:', date !== clonedDate);
console.log();

// 5. Cloning RegExp
console.log('5. Cloning RegExp:');
const regex = /test/gi;
const clonedRegex = structuredClone(regex);

console.log('   Original:', regex);
console.log('   Cloned:', clonedRegex);
console.log('   Same pattern:', regex.source === clonedRegex.source);
console.log('   Same flags:', regex.flags === clonedRegex.flags);
console.log();

// 6. Cloning Map
console.log('6. Cloning Map:');
const map = new Map([
  ['key1', 'value1'],
  ['key2', { nested: 'object' }]
]);

const clonedMap = structuredClone(map);
clonedMap.get('key2').nested = 'modified';

console.log('   Original map key2:', map.get('key2').nested);
console.log('   Cloned map key2:', clonedMap.get('key2').nested);
console.log('   ✓ Map contents are deeply cloned');
console.log();

// 7. Cloning Set
console.log('7. Cloning Set:');
const set = new Set([1, 2, 3, { x: 4 }]);
const clonedSet = structuredClone(set);

console.log('   Original set size:', set.size);
console.log('   Cloned set size:', clonedSet.size);
console.log('   Has same values:', clonedSet.has(1) && clonedSet.has(2));
console.log();

// 8. Cloning ArrayBuffer and typed arrays
console.log('8. Cloning ArrayBuffer and typed arrays:');
const buffer = new Uint8Array([1, 2, 3, 4, 5]);
const clonedBuffer = structuredClone(buffer);

clonedBuffer[0] = 99;

console.log('   Original buffer:', Array.from(buffer));
console.log('   Cloned buffer:', Array.from(clonedBuffer));
console.log('   ✓ Binary data is copied');
console.log();

// 9. Handling circular references
console.log('9. Handling circular references:');
const circular = { name: 'circular' };
circular.self = circular;
circular.nested = { parent: circular };

const clonedCircular = structuredClone(circular);

console.log('   Original self-reference:', circular.self === circular);
console.log('   Cloned self-reference:', clonedCircular.self === clonedCircular);
console.log('   Nested parent reference:', clonedCircular.nested.parent === clonedCircular);
console.log('   ✓ Circular references preserved correctly');
console.log();

// 10. Complex nested structure
console.log('10. Complex nested structure:');
const complex = {
  num: 42,
  str: 'hello',
  date: new Date(),
  arr: [1, 2, { nested: true }],
  map: new Map([['key', 'value']]),
  set: new Set([1, 2, 3]),
  regexp: /test/gi,
  buffer: new Uint8Array([10, 20, 30])
};

const clonedComplex = structuredClone(complex);

console.log('   Cloned num:', clonedComplex.num);
console.log('   Cloned date:', clonedComplex.date instanceof Date);
console.log('   Cloned map:', clonedComplex.map.get('key'));
console.log('   Cloned set:', clonedComplex.set.has(2));
console.log('   Cloned regexp:', clonedComplex.regexp.source);
console.log('   ✓ All complex types cloned correctly');
console.log();

// 11. Comparison with JSON
console.log('11. Comparison with JSON.parse(JSON.stringify()):');
const obj = {
  date: new Date(),
  undefined: undefined,
  map: new Map([['a', 1]]),
  set: new Set([1, 2, 3])
};

console.log('   Original has Date:', obj.date instanceof Date);
console.log('   Original has undefined:', 'undefined' in obj);
console.log('   Original has Map:', obj.map instanceof Map);

try {
  const jsonClone = JSON.parse(JSON.stringify(obj));
  console.log('   JSON clone has Date:', jsonClone.date instanceof Date, '(becomes string)');
  console.log('   JSON clone has undefined:', 'undefined' in jsonClone, '(removed)');
  console.log('   JSON clone has Map:', jsonClone.map instanceof Map, '(becomes object)');
} catch (e) {
  console.log('   JSON.stringify failed:', e.message);
}

const structClone = structuredClone(obj);
console.log('   Structured clone has Date:', structClone.date instanceof Date);
console.log('   Structured clone has undefined:', 'undefined' in structClone);
console.log('   Structured clone has Map:', structClone.map instanceof Map);
console.log('   ✓ structuredClone preserves types better than JSON');
console.log();

// 12. Error cases
console.log('12. Error cases:');

// Cannot clone functions
try {
  structuredClone(() => {});
  console.log('   ✗ Function was cloned (unexpected)');
} catch (e) {
  console.log('   ✓ Cannot clone function:', e.message.split('\n')[0]);
}

// Cannot clone symbols
try {
  structuredClone(Symbol('test'));
  console.log('   ✗ Symbol was cloned (unexpected)');
} catch (e) {
  console.log('   ✓ Cannot clone symbol:', e.message.split('\n')[0]);
}

console.log();
console.log('=== Structured Clone Demo Complete ===');
