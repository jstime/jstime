// Test Temporal API
const date = new Temporal.PlainDate(2025, 10, 13);
console.log('Date:', date.year, date.month, date.day);

const time = new Temporal.PlainTime(15, 30, 45);
console.log('Time:', time.hour, time.minute, time.second);

const dt = new Temporal.PlainDateTime(2025, 10, 13, 15, 30, 45);
console.log('DateTime:', dt.year, dt.month, dt.day, dt.hour, dt.minute);
