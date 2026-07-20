# 项目架构

## 一句话设计

结构化知识和可执行代码是源数据，mdBook 是它们的阅读投影。

## 分层

```text
content + solutions
        |
        v
validate -> test -> benchmark -> trace
        |          |            |
        +----------+------------+
                   v
             generated pages
                   |
                   v
             mdBook + Web Components
                   |
          static site + optional runner API
```

### 知识层

`content/` 保存题目摘要、模式、不变量、数据结构与关系。显示标题可以改变，稳定 ID 不应改变。

### 证据层

`solutions/` 和 `artifacts/` 保存代码、测试、Trace 与 benchmark 结果。正文中的正确性和性能结论必须能够追溯到这里。

### 生成层

后续的 `tools/mdbook-algo` 负责：

- 校验元数据引用；
- 生成 `SUMMARY.md` 中的题目与模式索引；
- 将代码、Trace、benchmark 快照嵌入页面；
- 将自定义指令转换为 `<algo-lab>` 和 `<algo-viz>` 组件。

### 阅读层

mdBook 承担长文阅读、目录、搜索和静态发布。浏览器组件承担编辑器、动画播放器和运行结果展示。

### 执行层

- Rust 示例可先接 mdBook Playground。
- Python 小程序可在后续阶段接 Pyodide。
- C++、Java、Go 或任意用户代码必须进入独立沙箱服务。
- 页面展示的正式性能数据来自 CI，不使用读者浏览器的即时跑分。

## 核心实体

```text
Problem --has_solution--> Solution
Problem --uses_pattern--> Pattern
Problem --uses_structure-> DataStructure
Problem --related_to-----> Problem
Solution --emits---------> Trace
Solution --measured_by---> BenchmarkReport
```

关系边使用 `same_invariant`、`variation_of`、`generalizes`、`transforms_to`、`contrasts_with` 等受控类型。

## 发布门禁

| 状态 | 必须满足 |
|---|---|
| `draft` | 元数据和原创摘要完整 |
| `tested` | 固定测试通过 |
| `verified` | 边界测试、性质测试与复杂度推导通过审阅 |
| `benchmarked` | 存在可复现的环境和结果记录 |
| `visualized` | Trace schema 有效，关键步骤可解释 |
| `editorial-reviewed` | 事实、技术和结构审阅完成 |
| `published` | 构建、链接和内容边界检查全部通过 |

