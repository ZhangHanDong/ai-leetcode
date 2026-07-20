# Publishing Spec

## Goal

只把证据完整、链接有效并通过审阅的内容发布到 mdBook。

## Build inputs

- `content/`
- `solutions/`
- 已审核的 `artifacts/`
- 手写的书级导航和前言

## Generated outputs

- 题目页；
- 模式页；
- 数据结构页；
- 题目、模式和关系索引；
- benchmark 表格和 Trace 组件声明。

## Review gates

- 事实审阅：来源、约束、复杂度和数据一致；
- 技术审阅：代码、证明、Trace 和 benchmark 结论一致；
- 复杂度审阅：每种算法都有包含时间与额外空间的 Mermaid 图，且变量、边界类型和前提完整；
- 结构审阅：不重复、交叉引用正确、章节预算合理；
- 构建审阅：mdBook 构建成功，无断链和缺失资源。

## Acceptance criteria

- 仅 `published` 内容进入默认索引；
- `draft` 内容只能在预览构建中显示；
- 生成目录可被完整删除并重新构建；
- 每一种发布算法都有 Mermaid 复杂度图，且时间、峰值额外空间、变量、边界类型和前提完整；
- 页面清楚区分运行体验与正式性能数据。
