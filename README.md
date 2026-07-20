# ai-leetcode

一本由可验证代码、算法 Trace 和基准测试驱动生成的算法学习书。

本项目不把题解 Markdown 当作唯一真源。题目元数据、算法模式、解法代码、测试、动画 Trace 和性能结果分别保存，再由构建工具投影为 mdBook 页面。

## 项目目标

- 从多道题中提炼可迁移的算法模式、不变量和证明方法。
- 为同一道题保留多种实现，并在可复现环境中比较性能与工程取舍。
- 用统一的语义事件生成可单步、回退和调速的算法动画。
- 让 AI 参与研究和写作，但让测试、基准数据和审阅状态决定内容是否可发布。
- 最终在 mdBook 中统一呈现正文、代码、动画、关联题目和运行入口。

## 快速开始

```bash
mdbook serve book --open
```

当前阶段提供书籍大纲、内容模型和创作模板。代码运行服务、生成器和完整动画组件将在后续阶段实现。

## 内容从哪里开始

1. 复制 `content/problems/_template/` 创建一道题的知识条目。
2. 复制 `solutions/_template/` 创建可执行解法、测试、Trace 和 benchmark 配置。
3. 为题目关联 `content/patterns/` 与 `content/structures/` 中的稳定 ID。
4. 按 `specs/` 中的门禁依次完成验证、基准测试、可视化和编辑审阅。
5. 生成的书页只能写入 `book/src/generated/`，不要覆盖知识源文件。

完整架构见 [docs/architecture.md](docs/architecture.md)，全书规划见 [docs/book-outline.md](docs/book-outline.md)。

## 内容边界

仓库只保存题目链接、必要元数据、原创摘要、原创测试、代码、证明和可视化，不自动抓取或完整复制 LeetCode 题面。

