# CSS API using Cloudflare Workers

This worker uses [lightningcss](https://github.com/parcel-bundler/lightningcss) to prefix css on the Cloudflare edge node.


### Prefixing

```http
POST https://css-api.fos.gg/prefix
Content-Type: application/json

{
    "code": "body { -webkit-border-radius: 2px; -moz-border-radius: 2px; border-radius: 2px; }"
}
```


Additional Options:

```js
targets: {
    chrome: 95 * 65536, // multiply version with 65536
    firefox: 101 * 65536,
    safari: 15 * 65536,
    opera: 85 * 65536,
    edge: 102 * 65536,
    ios: 15 * 65536,
    android: 103 * 65536,
    samsung: 16 * 65536
},
drafts: {
    nesting: true, // enable css nesting
    customMedia: true // enable css custom media
},
cssModules: false, // enable css modules
unusedSymbols: [] // A list of known unused symbols, including CSS class names
```