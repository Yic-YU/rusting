use std::sync::Arc;

use tokio::sync::Mutex;

#[cfg(test)]
mod tests {
    use tokio::sync::Notify;

    use super::*;
    // 忙等待 (Busy-Waiting) CPU 消耗 🥵
    // 想象一下当轮到 handle1 执行时（flag 为 false），handle2 会做什么？

    // handle2 争抢并成功获得锁。

    // 它检查 if counter.flag，发现 flag 是 false，条件不满足。

    // handle2 什么也不做，到达代码块末尾，释放锁。

    // loop 循环立即开始下一轮，handle2 再次去争抢锁。

    // handle2 陷入了一个疯狂的循环：“获取锁 -> 检查条件 -> 失败 -> 释放锁 -> 立即再次获取锁”。这个过程会持续进行，直到 handle1 有机会运行并将 flag 翻转。

    // 这个过程就是 “忙等待” 或 “自旋（Spinning）”。任务没有在等待时“休息”，而是在徒劳地空转，白白消耗大量的 CPU 资源。这在实际应用中是需要极力避免的。
    #[tokio::test]
    async fn test_counter_byturn() {
        struct Counter {
            value: i32,
            flag: bool,
        }
        let counter = Arc::new(Mutex::new(Counter {
            value: 0,
            flag: false,
        }));

        let counter1 = counter.clone();
        let counter2 = counter.clone();

        let handle1 = tokio::spawn(async move {
            loop {
                let mut counter = counter1.lock().await;

                if counter.value >= 100 && !counter.flag {
                    break;
                }
                if !counter.flag {
                    counter.value += 1;
                    counter.flag = !counter.flag;
                    println!("thread 1: {}", counter.value);
                }
            }
        });

        let handle2 = tokio::spawn(async move {
            loop {
                let mut counter = counter2.lock().await;
                if counter.value >= 100 {
                    break;
                }
                if counter.flag {
                    counter.value += 1;
                    counter.flag = !counter.flag;
                    println!("thread 2: {}", counter.value);
                }
            }
        });

        handle1.await.unwrap();
        handle2.await.unwrap();
    }
    #[tokio::test]
    async fn test_counter_with_notify() {
        struct Counter {
            value: i32,
            flag: bool, 
        }

        let state = Arc::new((
            Mutex::new(Counter { value: 0, flag: false }),
            Notify::new(),
        ));

        let state1 = state.clone();
        let handle1 = tokio::spawn(async move {
            loop {
                let mut guard = state1.0.lock().await;

                if guard.value >= 100 {
                    state1.1.notify_one();
                    break;
                }

                if !guard.flag { 
                    guard.value += 1;
                    guard.flag = true; 
                    println!("thread 1: {}", guard.value);
                    drop(guard);
                    state1.1.notify_one();
                } else { 
                    drop(guard);
                    state1.1.notified().await;
                }
            }
        });

        let state2 = state.clone();
        let handle2 = tokio::spawn(async move {
            loop {
                let mut guard = state2.0.lock().await;

                if guard.value >= 100 {
                    state2.1.notify_one();
                    break;
                }

                if guard.flag {
                    guard.value += 1;
                    guard.flag = false; 
                    println!("thread 2: {}", guard.value);

                    drop(guard);
                    state2.1.notify_one();
                } else {
                    drop(guard);
                    state2.1.notified().await;
                }
            }
        });

        handle1.await.unwrap();
        handle2.await.unwrap();
    }
}
