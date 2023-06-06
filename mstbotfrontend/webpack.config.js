const path = require('path');

module.exports = {
  entry: './src/index.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bundle.js',
  },
  resolve: {
    fallback: {
      https: require.resolve('https-browserify'),
      zlib: require.resolve('browserify-zlib')
    }
  },
};
