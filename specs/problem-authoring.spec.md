# Problem Authoring Spec

## Goal

创建一道题的最小知识条目，并把它连接到现有模式和数据结构，而不复制完整题面。

## Inputs

- 题目公开链接和必要元数据；
- 作者自己的问题理解；
- `content/taxonomy.yaml`；
- 已存在的模式和数据结构 ID。

## Allowed files

- `content/problems/<id>-<slug>/`
- 必要时更新关系候选，但不得直接把未经验证的模式标记为已发布。

## Required output

- `problem.yaml`
- `analysis.md`
- `relations.yaml`

## Acceptance criteria

- ID、slug、来源 URL 和访问日期完整；
- 摘要为原创表达；
- 输入约束说明了它们对算法选择的影响；
- 至少定义一个候选不变量；
- 所有引用的稳定 ID 存在，或明确标为待创建；
- 状态保持为 `draft`，直到解法验证完成。

## Prohibited

- 爬取或复制完整 LeetCode 题面；
- 在没有测试和证明时宣称解法正确；
- 用展示名称代替稳定 ID 建立关系。

