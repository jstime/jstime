// Example: Using import.meta.url to get the current module's URL
// This demonstrates the import.meta.url feature in jstime

console.log('Module URL:', import.meta.url);

// Parse the URL to get information about the current module
const moduleUrl = new URL(import.meta.url);
console.log('\nModule Information:');
console.log('  Protocol:', moduleUrl.protocol);
console.log('  Path:', moduleUrl.pathname);
console.log('  Filename:', moduleUrl.pathname.split('/').pop());

// Use import.meta.url to resolve relative paths
const configPath = new URL('./config.json', import.meta.url);
console.log('\nRelative path resolution:');
console.log('  Config file would be at:', configPath.href);
