# Agent Specs

这些 spec 把内容生产过程变成可检查的工作流。Agent 应先读取与当前阶段对应的 spec，再修改文件。

| 阶段 | Spec | 主要输出 |
|---|---|---|
| 题目建档 | `problem-authoring.spec.md` | 元数据、原创摘要、关系候选 |
| 解法验证 | `solution-verification.spec.md` | 可执行代码、测试、证明 |
| 动画 | `visualization.spec.md` | Trace 与可视化说明 |
| 性能 | `benchmarking.spec.md` | 原始结果、环境、结论 |
| 共性总结 | `synthesis.spec.md` | 模式章节、关系边、反例 |
| 发布 | `publishing.spec.md` | 生成页面、索引、审阅记录 |

新增长章节前，复制 `templates/chapter-spec.md`，明确预算、依赖、证据和验收标准。

