## [测试的组织结构](https://kaisery.github.io/trpl-zh-cn/ch11-03-test-organization.html#测试的组织结构)

> [ch11-03-test-organization.md](https://github.com/rust-lang/book/blob/main/src/ch11-03-test-organization.md)
> commit cfb2c3cce7c20d4ad523dafdbf90ae3b25b1ba2c

本章一开始就提到，测试是一个复杂的概念，而且不同的开发者也采用不同的技术和组织。Rust 社区倾向于根据测试的两个主要分类来考虑问题：**单元测试**（*unit tests*）与 **集成测试**（*integration tests*）。<u>单元测试倾向于更小而更集中，在隔离的环境中一次测试一个模块，或者是测试私有接口。而集成测试对于你的库来说则完全是外部的。它们与其他外部代码一样，通过相同的方式使用你的代码，只测试公有接口而且每个测试都有可能会测试多个模块。</u>

为了保证你的库能够按照你的预期运行，从独立和整体的角度编写这两类测试都是非常重要的。