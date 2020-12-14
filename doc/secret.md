# 密钥管理
***

### 根据uid获取密钥

用户--->调用js--->js查库--->js调用wasm(main)--->wasm调用js(notify)

js调用wasm请求参数(main)
```json
{
  "method": "get_secret",
  "param": "{\"keypair\":\"string\",\"cert\":\"string\",\"secret_type\":\"string\",\"uid\":\"string\",\"seed\":\"string\"}"
}
```
js调用wasm请求 param参数
```json
{
  "keypair": "string",
  "cert": "string",
  "secret_type": "string",
  "uid": "string",
  "seed": "string"
}
```

wasm调用js(notify)
```json
{
  "method": "notify_get_secret",
  "code": 0,
  "msg": "success",
  "param": "{\"keypair\":\"string\",\"cert\":\"string\",\"secret_type\":\"string\",\"uid\":\"string\",\"seed\":\"string\"}"
}
```

wasm调用js param参数
```json
{
  "keypair": "string",
  "cert": "string",
  "secret_type": "string",
  "uid": "string",
  "seed": "string"
}
```

### 分页获取密钥列表

用户--->调用js--->js查库--->js调用wasm(main)--->wasm调用js(notify)

js调用wasm请求参数(main)
```json
{
  "method": "get_secret_list",
  "param": "{\"list\":[{\"keypair\":\"string\",\"cert\":\"string\",\"secret_type\":\"string\",\"uid\":\"string\",\"seed\":\"string\"},{\"keypair\":\"string\",\"cert\":\"string\",\"secret_type\":\"string\",\"uid\":\"string\",\"seed\":\"string\"}]}"
}
```
js调用wasm请求 param参数
```json
{
  "list": [
    {
      "keypair": "string",
      "cert": "string",
      "secret_type": "string",
      "uid": "string",
      "seed": "string"
    }
  ]
}
```

wasm调用js(notify)
```json
{
  "method": "notify_get_secret_list",
  "code": 0,
  "msg": "success",
  "param": "{\"list\":[{\"keypair\":\"string\",\"cert\":\"string\",\"secret_type\":\"string\",\"uid\":\"string\",\"seed\":\"string\"},{\"keypair\":\"string\",\"cert\":\"string\",\"secret_type\":\"string\",\"uid\":\"string\",\"seed\":\"string\"}]}"
}
```

wasm调用js param参数
```json
{
  "list": [
    {
      "keypair": "string",
      "cert": "string",
      "secret_type": "string",
      "uid": "string",
      "seed": "string"
    }
  ]
}
```

### 生成并注册密钥

用户--->调用js--->js调用wasm(main)--->wasm调用js(request)--->js调用wasm(notify)--->wasm调用js(notify)

js调用wasm请求参数(main)

```json
{
  "method": "gen_and_register",
  "param": "{\"url\":\"string\",\"timeout\":\"string\",\"info\":{\"account\":\"string\",\"password\":\"string\"}}"
}
```
js调用wasm请求 param参数

```json
{
  "url": "",
  "timeout": "",
  "info": {
    "account": "",
    "password": ""
  }
}
```

wasm调用js请求(request)

```json
{
  "method": "gen_and_register",
  "param": "{\"cert\":\"string\",\"info\":{\"account\":\"string\",\"password\":\"string\"},\"hold\":{\"seed\":\"string\"}}"
}
```

wasm调用js请求 param
hold位json串，里边的参数不定，目前只有一个seed
```json
{
  "cert": "string",
  "info": {
    "account": "",
    "password": ""
  },
  "hold": {
    "seed": "string"
  }
}
```

js调用wasm(notify)

```json
{
  "method": "notify_gen_and_register",
  "code": 0,
  "msg": "success",
  "param": "{\"cert\":\"string\",\"uid\":\"string\",\"hold\":{\"seed\":\"string\"}}"
}
```

js调用wasm param

```json
{
  "cert": "string",
  "uid": "string",
  "hold": {
    "seed": "string"
  }
}
```

wasm调用js(notify)
```json
{
  "method": "notify_gen_and_register",
  "code": 0,
  "msg": "success",
  "param": ""
}
```
wasm调用js param

```json
{
  "result": {
    "keypair": "string",
    "cert": "string",
    "secret_type": "string",
    "uid": "string",
    "seed": "string"
  },
  "sql": "string"
}
```


