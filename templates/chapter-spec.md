# ${chapter_id}: ${chapter_title}

## Position

- Part: `${part_id}`
- Previous: `${previous_chapter}`
- Next: `${next_chapter}`
- Prerequisites: `${prerequisites}`

## Chapter budget

- Type: `book-chapter`
- Cases: `${case_count}`
- Depth: `${depth_layers}`
- Word budget: `${min_words}-${max_words}`

## Thesis

本章希望读者最终能够独立推导出的核心结论。

## Evidence

- 已验证题目：
- 解法源文件：
- Trace：
- benchmark 报告：
- 外部来源：

## Required sections

- 为什么这很重要
- 模型、状态和不变量
- 多案例推导
- 模式提炼
- 失效边界
- 用户能做什么

## Allowed files

- `${allowed_paths}`

## Prohibited files

- `book/src/generated/`

## Acceptance criteria

- [ ] 章节定位、前置依赖和阅读场景明确
- [ ] 所有技术结论可追溯到代码或证据
- [ ] 案例之间没有重复承担同一教学作用
- [ ] 至少包含一个反例或失效边界
- [ ] 每种算法都有包含时间与峰值额外空间的 Mermaid 复杂度图
- [ ] 交互动画存在静态退化说明
- [ ] 事实、技术和结构审阅完成
