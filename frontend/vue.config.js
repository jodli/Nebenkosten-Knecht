module.exports = {
    outputDir: 'dist',
    publicPath: '/',
    devServer: {
        proxy: {
            '/api': {
                target: 'http://localhost:8080',
                changeOrigin: true
            }
        }
    }
};
