# CSS API

This API exposes [lightningcss](https://github.com/parcel-bundler/lightningcss) as an API running with AWS Lambda.

API URL: https://27uhytumuulrysydgmak3tlsgu0giwff.lambda-url.eu-central-1.on.aws

## Requesting

Request JSON:

The only required parameter is `stylesheet`

```json
{
    "stylesheet": "CSS",
    // Minify the CSS (default true)
    "minify": true,
    // CSS Target. Multiply version with 65536
    "targets": {
        "android": 0,
        "chrome": 0,
        "edge": 0,
        "firefox": 0,
        "ie": 0,
        "ios_saf": 0,
        "opera": 0,
        "safari": 0,
        "samsung": 0
    },
    // Or targets as browserlist format
    "browserlist": "chrome: >= 100\nfirefox: >= 100"
    // Support for https://www.w3.org/TR/mediaqueries-5/
    "custom_media_queries": true,
    // Enable CSS nesting support
    "css_nesting": true
}
```

Output JSON:

```json
{
    "compiled": "....."
}
```