# Tools

后续阶段将在这里实现：

- `validate`：校验 YAML/JSON schema、稳定 ID 和引用；
- `mdbook-algo`：从知识源生成 mdBook 页面；
- `tracegen`：执行带 instrumentation 的参考实现并生成 Trace；
- `benchmark`：运行统一数据集并记录环境与原始结果。

工具必须输出确定性产物，并在失败时返回非零状态。

