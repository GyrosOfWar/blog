var path = require('path');

module.exports = {
    entry: "./src/index.tsx",
    output: {
        filename: "bundle.js",
        path: __dirname + "/dist"
    },

    // Enable sourcemaps for debugging webpack's output.
    devtool: "source-map",

    resolve: {
        // Add '.ts' and '.tsx' as resolvable extensions.
        extensions: ["", ".webpack.js", ".web.js", ".ts", ".tsx", ".js"]
    },

    module: {
        loaders: [
            // All files with a '.ts' or '.tsx' extension will be handled by 'ts-loader'.
            {test: /\.tsx?$/, loader: "ts-loader"}
        ],

        preLoaders: [
            // All output '.js' files will have any sourcemaps re-processed by 'source-map-loader'.
            {test: /\.js$/, loader: "source-map-loader"}
        ]
    },

    devServer: {
        inline: true,
        progress: true,
        stats: "errors-only",
        devtool: "eval-source-map",
        output: {
            path: path.resolve(__dirname, "build"),
            publicPath: "/static/",
            filename: "bundle.js"
        },

        proxy: {
            "/api/*": {
                "target": "http://localhost:5000",
                secure: false
            }
        }
    }
};