# LYSH
Lysh是一个轻量级的编程语言，它的主要目的是开箱即用地实现需要的功能以及提供干净强大的Shell

它的涵义是 Lysh's Lisp Shell

因此它**不适用于任何的重量级应用的开发。**

## 用Rust封装而言目前的效率损耗

- 基本Value单元长度为16byte（即u128），很多平台上CPU总线得跑两次甚至更多
- 每次操作变量时的运行时类型检查。虽然这一步可以在拿到足够多的类型信息后干掉（TODO: LirExecEngine, TypeInfer, JIT）
- Mutable值读写的效率损耗：LYSH是线程安全的，为了实现这一点，包括LFunction在内的Mutable值都包裹了一层RwLock。因此在使用Mutable值时有额外的开销
- 由RC造成的内存碎片，严重性较高。后期可考虑重写Ref

## 未来计划完成的

- [x] AstExecEngine
- [x] LirExecEngine
- [x] TypeInfer
- [x] JIT
- [x] MemoryPool
