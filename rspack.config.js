// TODO: Remove this rspack.config.js once RSPack Rust crates are properly integrated
// The configuration below shows what should be implemented in Rust using:
// - rspack_core for compilation 
// - rspack_plugin_javascript for TypeScript/JSX handling
// - rspack_plugin_css for CSS processing

const path = require('path');

module.exports = {
  entry: './frontend/index.tsx',
  mode: 'development',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'main.js',
  },
  resolve: {
    extensions: ['.js', '.jsx', '.ts', '.tsx'],
  },
  module: {
    rules: [
      {
        test: /\.(js|jsx|ts|tsx)$/,
        use: {
          loader: 'builtin:swc-loader',
          options: {
            jsc: {
              parser: {
                syntax: 'typescript',
                tsx: true,
              },
              transform: {
                react: {
                  runtime: 'automatic',
                },
              },
            },
          },
        },
      },
      {
        test: /\.css$/,
        use: [
          {
            loader: 'builtin:lightningcss-loader',
          },
        ],
        type: 'css',
      },
    ],
  },
};