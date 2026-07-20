# Visualization Spec

## Goal

从算法的语义状态变化生成确定、可解释、可回放的动画。

## Trace contract

- 每个事件包含 `step`、受控 `type`、`payload` 和 `explanation`；
- step 严格递增；
- 事件类型来自 `content/taxonomy.yaml`；
- 初始状态和最终状态可独立校验；
- 同一输入和实现必须产生相同 Trace；
- Trace 记录语义动作，不记录像素位置和颜色。

## Renderer contract

- 支持播放、暂停、单步、回退、调速和复位；
- 当前状态必须能用文本说明；
- 颜色不是唯一的信息通道；
- 长 Trace 允许采样，但必须标记省略区间；
- 交互组件失效时仍提供静态图或文字退化内容。

## Acceptance criteria

- 每个关键状态转移能映射回算法代码；
- 动画结束状态与测试期望一致；
- 动画不声称可以自动解释任意用户修改后的代码；
- 通过后状态可推进到 `visualized`。

