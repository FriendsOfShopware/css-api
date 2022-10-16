import * as localWasm from './lightningcss_node.js';

let enc = new TextEncoder();
let dec = new TextDecoder();
let loaded = 0

class JsonResponse extends Response {
  constructor(obj: any, statusCode: number) {
    super(JSON.stringify(obj), {
      status: statusCode
    })
  }
}

export default {
  async fetch(request: Request) {
    const url = new URL(request.url);

    switch (url.pathname) {
      case '/':
      case '':
        return new Response(null, {
          status: 302,
          headers: {
            'location': 'https://github.com/FriendsOfShopware/css-api'
          }
        })
      case '/prefix':
        return await prefix(request);
      default:
        return new Response('', {
          status: 404
        });
    }
  },
};

async function prefix(request: Request): Promise<Response> {
  let json: any;
  try {
    json = await request.json() as any;
  } catch (e) {
    return new JsonResponse('Invalid JSON body', 200);
  }

  const defaultOptions = {
    filename: 'test.css',
    code: enc.encode(json.code),
    minify: true,
    targets: {
      chrome: 6225920
    },
    drafts: {
      nesting: true,
      customMedia: true
    },
    cssModules: false,
    analyzeDependencies: false,
    unusedSymbols: []
  };

  delete json.code;

  if (loaded === 0) {
    await localWasm.default()
    loaded = 1;
  }

  let res;
  try {
    res = localWasm.transform({ ...defaultOptions, ...json });
  } catch (e) {
    return new JsonResponse('Cannot process css failed with: ' + e.toString(), 500);
  }

  res.code = dec.decode(res.code);

  return new Response(JSON.stringify(res), {
    headers: {
      "content-type": 'application/json'
    }
  });
}