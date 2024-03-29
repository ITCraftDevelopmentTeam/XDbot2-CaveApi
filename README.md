# XDbot2-CaveApi

XDbot2 回声洞接口源码

## 运行

```bash
cargo run --release
```

## 环境变量

> XDbot2-CaveApi 支持 `.env`

| 键名          | 默认值            | 说明                         |
|--------------|------------------|------------------------------|
| `PORT`       | `8080`           | 服务器端口                     |
| `HOST`       | `0.0.0.0`        | 服务器 IP                     |
| `SOURCE`     | `/`              | XDbot2 Data 目录              |
| `IMPLEMENTS` |                  | OneBot V11 实现地址 (`,`分隔)  |

## 接口

### GET `/`

#### 参数

无

#### 响应

```json
{"version":"0.1.0","count":{"total":2167,"valid":1917},"code":200}
```

- **`version` (字符串):**
  - 描述：API 的版本号。
  - 作用：标识当前 API 使用的版本。

- **`count` (对象):**
  - 描述：包含有关返回数据计数的信息。
  - 作用：
    - `total` (整数): 总计数，表示返回结果的总数。
    - `valid` (整数): 有效计数，表示符合特定条件的结果数。

- **`code` (整数):**
  - 描述：HTTP 状态码。
  - 作用：指示 API 请求的执行状态，通常 `200` 表示成功。



### GET `/random`

#### 参数

- **`no_image` (布尔值):**
  - 描述：控制是否包含图像的参数.
  - 作用：如果设置为 `true`，则 API 返回的结果中将不包含图像；如果设置为 `false`，则包含图像。

- **`max_length` (整数):**
  - 描述：指定返回结果的最大长度.
  - 作用：限制返回结果的字符数，确保结果不超过指定的最大长度。

> [!NOTE]
> `max_length` 只能被设置为正整数

#### 响应

```json
{"code":200,"id":586,"content":"这是一条 阿嚏~ 的冷知识","time":0.0,"sender":"ㄧㄈㄢㄓㄨㄓYifanZhuZhu","images":{}}
```

- **`code` (整数):**
  - 描述：HTTP 状态码。
  - 作用：指示 API 请求的执行状态，`200` 表示成功。

- **`id` (整数):**
  - 描述：唯一标识符。
  - 作用：用于标识返回结果的唯一 ID。

- **`content` (字符串):**
  - 描述：包含冷知识的文本内容。
  - 作用：提供一条有趣或冷知识的文本信息。

- **`time` (浮点数):**
  - 描述：时间戳。
  - 作用：表示消息的时间戳，通常使用 UNIX 时间格式。

- **`sender` (字符串):**
  - 描述：消息发送者的用户名。
  - 作用：标识消息的发送者。

- **`images` (对象):**
  - 描述：包含图像信息的对象。
  - 作用：存储与消息相关的图像信息。如果回声洞包含图像，此字段类似于`{"<图像ID>": "<图像BASE64>"}`

## 接口错误

```json
{"code":500,"message":"没有符合要求的回声洞"}
```

- **`code` (整数):**
  - 描述：HTTP 状态码。
  - 作用：指示 API 请求的执行状态，`500` 表示服务器内部错误。

- **`message` (字符串):**
  - 描述：错误消息。
  - 作用：提供了关于发生的错误的详细说明。

## 注意事项

当状态码在 `500`、`200` 之外时，API 响应不是一个 Json

---

[![wakatime](https://wakatime.com/badge/github/ITCraftDevelopmentTeam/XDbot2-CaveApi.svg)](https://wakatime.com/badge/github/ITCraftDevelopmentTeam/XDbot2-CaveApi)
[![Rust Release binary](https://github.com/ITCraftDevelopmentTeam/XDbot2-CaveApi/actions/workflows/release.yml/badge.svg)](https://github.com/ITCraftDevelopmentTeam/XDbot2-CaveApi/actions/workflows/release.yml)
![GitHub License](https://img.shields.io/github/license/ITCraftDevelopmentTeam/XDbot2-CaveApi)
![GitHub Release](https://img.shields.io/github/v/release/ITCraftDevelopmentTeam/XDbot2-CaveApi)
![GitHub commit activity](https://img.shields.io/github/commit-activity/m/ITCraftDevelopmentTeam/XDbot2-CaveApi)
![GitHub repo size](https://img.shields.io/github/repo-size/ITCraftDevelopmentTeam/XDbot2-CaveApi)

