# 一、什么是 TiHC？
TiHC (TiDB Healthy Check) 是 rust 写的巡检 TiDB 的工具，目的是为缩短询价时间，提高交付效率。
* TiHC 可直接生成 Docx 巡检文档；


# 二、为什么需要 TiHC？
* 节约 TiDB 巡检过程中大量可模版化工作；
* 非常容易上手，如果你是一名 DBA（遵循 terminal tool 使用风格）；
* 对新手学习友好，如果你接触 TiDB 不久，提供一套基础 healthy check 思路判定当前集群是否健康；


## 2.1 如何 TiHC？

可以直接获取从该网址：[releases section]() :

## 2.2 如何手动 build TiHC？

Run:

```bash
make
```

One has to make sure, that pkg-config, mysql_config, pcre-config are all in $PATH

Binlog dump is disabled by default to compile with it you need to add -DWITH_BINLOG=ON to cmake options

To build against mysql libs < 5.7 you need to disable SSL adding -DWITH_SSL=OFF

# 三、如何使用 TiHC？ 

See [Usage](docs/mydumper_usage.rst)

## How does consistent snapshot work?

This is all done following best MySQL practices and traditions:

* As a precaution, slow running queries on the server either abort the dump, or get killed
* Global read lock is acquired ("FLUSH TABLES WITH READ LOCK")
* Various metadata is read ("SHOW SLAVE STATUS","SHOW MASTER STATUS")
* Other threads connect and establish snapshots ("START TRANSACTION WITH CONSISTENT SNAPSHOT")
** On pre-4.1.8 it creates dummy InnoDB table, and reads from it.
* Once all worker threads announce the snapshot establishment, master executes "UNLOCK TABLES" and starts queueing jobs.

This for now does not provide consistent snapshots for non-transactional engines - support for that is expected in 0.2 :)
