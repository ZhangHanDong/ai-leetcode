# ch03-lc0003: 无重复字符的最长子串

## Position

- Part: `第三篇：数组、字符串与指针技巧`
- Previous: `从约束到模型；让正确性可以被解释`
- Next: `后续滑动窗口题族综合章节`
- Prerequisites: `半开区间、循环不变量、HashSet、HashMap`

## Chapter budget

- Type: `book-chapter / first complete problem specimen`
- Cases: `1 problem, 4 Rust implementations`
- Depth: `4 layers: model -> invariant/proof -> Rust representation -> measured trade-offs`
- Word budget: `3500-5000 Chinese characters`

## Thesis

“最长无重复连续区间”可以通过维护一个始终合法的半开窗口在线求解；保存字符最后位置能够把逐步收缩压缩为一次安全跳跃，而定长表优化只在 ASCII 前提成立时适用。

## Evidence

- 已验证题目：`lc-0003`
- 解法源文件：`solutions/0003/src/*.rs`
- Trace：`artifacts/traces/lc-0003/last-seen-hash-map/abba.json`
- benchmark：`solutions/0003/benches/compare.rs` 与审核后的结果快照
- 外部来源：LeetCode 原题链接、Rust `str` 标准库文档

## Required sections

- 一句话抽象与输入约束
- 从暴力基线到合法窗口
- 窗口不变量及正确性证明
- 四种 Rust 实现与字符串语义
- Trace 动画及静态退化说明
- 复杂度和 benchmark 比较
- 失效边界与后续题族

## Allowed files

- `content/problems/lc-0003-longest-substring-without-repeating-characters/`
- `content/patterns/variable-size-sliding-window/`
- `content/structures/{hash-set,hash-map,fixed-ascii-table}/`
- `solutions/0003/`
- `artifacts/traces/lc-0003/`
- `artifacts/benchmark-results/lc-0003/`
- `book/src/problems/lc-0003.md`
- 与这些内容直接相关的目录、索引和 Rust-only 约定

## Prohibited files

- `book/src/generated/`
- 与 LC 3 无关的题目正文

## Acceptance criteria

- [x] 四个实现共享同一语义测试集
- [x] Unicode 通用实现与 ASCII 特化实现的边界写清楚
- [x] 使用穷举差分测试比较优化解与暴力基线
- [x] Trace 可由 Rust 示例程序确定性生成
- [x] benchmark 记录工具链、CPU、数据分布和 source revision
- [x] 章节在预算内，且每个性能结论有数据或明确标记为推导
- [x] mdBook、Rust 测试、Clippy 和格式检查通过
