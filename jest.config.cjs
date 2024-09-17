module.exports = {
	preset: 'ts-jest',
	testEnvironment: 'node',
	transform: {
		'^.+\\.tsx?$': 'babel-jest', // Ensure TypeScript files are transformed
		'^.+\\.js$': 'babel-jest', // Add this line to transform JavaScript files
		'^.+\\.ts?$': 'babel-jest', // Add this line to transform JavaScript files
	  },
	transformIgnorePatterns: [
		"/node_modules/(?!node-fetch|data-uri-to-buffer)" // Updated this line to include data-uri-to-buffer
	],
	// moduleNameMapper: {
	//   'node-fetch': 'node-fetch/lib/index.js',
	// },
  };