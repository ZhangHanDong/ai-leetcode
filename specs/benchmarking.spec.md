# Benchmarking Spec

## Goal

在可复现条件下比较实现，并把算法因素与语言、运行时和机器因素分开。

## Required metadata

- Git commit；
- 操作系统和 CPU；
- 编译器或解释器版本；
- 优化参数；
- 输入生成器、seed、规模与分布；
- warm-up 和测量次数；
- 原始样本或可追溯的结果文件。

## Comparison rules

- 正式 benchmark 必须使用优化构建；
- 同语言实现优先直接比较；
- 跨语言结果单独分组，并说明运行时差异；
- 默认报告中位数和 P95，不只报告最快一次；
- 把进程启动、输入解析与核心算法时间分开；
- 浏览器即时运行只用于体验，不进入正式比较表；
- LeetCode 的运行时间百分位不作为项目 benchmark 证据。

## Acceptance criteria

- 至少包含典型、边界和对抗分布；
- 结论没有超出数据支持范围；
- 每个“更快/更省内存”的说法链接到报告；
- 通过后状态可推进到 `benchmarked`。

