# Open Ping++ Payment Gateway

复刻了 Ping++ 的支付网关，让你可以自己架设一套聚合支付平台。用 Rust 重写了整个后端 🦀，完全兼容 Ping++ 的接口格式，原来的客户端代码一行都不用改。

## 为什么要做这个项目？

- 想自己掌控支付渠道的参数，不共享给第三方
- 买不起 Ping++ 等支付网关服务 💰
- 觉得 Ping++ 的接口设计挺好，但是想开源一个可以自由部署的版本
- 顺便用 Rust 重写了一遍，性能优秀，🦀 RIIR ✌️

## 支持什么支付方式？

完全替代不大可能，Ping++ 依然是目前接触到的接入最全的支付网关，主要实现：

第一优先级

- 支付宝 openapi 和 mapi 两种接口格式和 rsa/rsa256 两种签名方式
- 微信公众号和小程序
- 退款

第二优先级

- 支付宝微信代扣
- 当面付，App 支付（比较难测试）
- 境外支付宝微信
- PayPal
- 查账接口

第三阶段

- Dashboard 界面
- 分叉，不再兼容

## 启动 server

日志用了 tracing 库，需要设置环境变量 RUST_LOG，比如

```bash
RUST_LOG=pingxx_proxy_server=debug cargo watch -x "run"
```

## 已实现的接口

### 接口授权

- [x] 沿用 Ping++ 的 `Bearer [API_LIVE_KEY]` 格式

### 商户系统

- [x] `/v1/apps/:app_id/sub_apps/:sub_app_id`
- [x] `/v1/apps/:app_id/sub_apps/:sub_app_id/channels/:channel`
- [x] `/v1/apps/:app_id/sub_apps/:sub_app_id/channels`

- [x] `/v1/orders`
- [x] `/v1/orders/:order_id`
- [x] `/v1/orders/:order_id/pay`
- [x] `/v1/orders/:order_id/order_refunds`
- [x] `/v1/orders/:order_id/order_refunds/:refund_id`

### 基础支付

- [x] `/v1/charges`
- [x] `/v1/charges/:charge_id`
- [x] `/v1/charges/:charge_id/refunds`
- [x] `/v1/charges/:charge_id/refunds/:refund_id`

### 支付渠道异步通知

- [x] `/notify/charges/:charge_id`
- [x] `/notify/charges/:charge_id/refunds/:refund_id`
- [x] `/notify/:id/retry` 测试用途

## 数据结构

**Credential**

前端需要的参数，用于客户端打开支付控件、支付页面、显示二维码等。

```js
{
    object: "credential",
    alipay_pc_direct: {
        // alipay_pc_direct 所需的参数
    },
    wx_pub: {
        // wx_pub 所需的参数
    },
}
```

**Charge**

```js
{
    id,
    object: "charge",
    channel,  // 支付渠道
    credential,  // Credential 对象
}
```

**Order**

```js
{
    id,
    object: "order",
    charge_essentials: {
        // 最近一次请求的支付所需的支付要素，是 Charge 上的部分数据，但不是完整的 Charge 对象
        channel,
        credential,  // Credential 对象
    },
    charges: {
        data: [
            // Charge 列表, 和前面 Charge 结构完全一样
        ]
    },
}
```
