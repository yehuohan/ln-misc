//! rust线程同步机制
//!
//! Channel: 线程间的消息通道；
//! Mutex: 线程间的共享访问；
//! Send trait: 允许在线程间转移所有权；
//! Sync trait: 允许多线程访问；
//!

use std::thread;
use std::time::Duration;
use std::sync::{mpsc, Mutex, Arc, Condvar};


/// 使用mpsc::channel在线程间传递消息；
/// 一个channel可以有多个发送者，但只能有一个接收者。
fn ln_channel() {
    let (txa, rx) = mpsc::channel(); // mpsc是多个生产者，单个消费者（multiple producer, single consumer）的缩写
    let txb = mpsc::Sender::clone(&txa); // 使用clone创建多个生产者

    thread::spawn(move || {
        let vals = vec![
            String::from("txa1"),
            String::from("txa2"),
            String::from("txa3"),
        ];
        for v in vals {
            txa.send(v).unwrap(); // tx是发送端，send会转移所有权，保证线程间数据一致性
            thread::sleep(Duration::from_millis(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("txb1"),
            String::from("txb2"),
            String::from("txb3"),
        ];
        for v in vals {
            txb.send(v).unwrap(); // tx是发送端，send会转移所有权，保证线程间数据一致性
            thread::sleep(Duration::from_millis(1));
        }
    });

    println!("Got by recv: {}", rx.recv().unwrap()); // recv接收会阻塞当前线程，直至收到数据，或发送端关闭通道

    loop {
        if let Ok(r) = rx.try_recv() { // try_recv是非阻塞接收，收到数据时，放入Ok中
            println!("Got by try_recv: {}", r);
            break;
        }
    }

    loop {
        let mut ri = rx.try_iter(); // 使用非阻塞迭代器
        if let Some(r) = ri.next() {
            println!("Got by try_iter: {}", r);
            break;
        }
    }

    for r in rx { // rx也可以当作一个阻塞迭代器
        println!("Got by iter: {}", r);
    }
}

/// 互斥器（mutex）是mutual exclusion的缩写，保证同一时刻只允许一个线程访问某些数据。
/// 在rust中，Mutex<T>.lock()返回MutexGuard智能指针；
/// MutexGuard Deref trait实现其内部数据的访问（即不获取lock，就不能访问到内部数据）；
/// MutexGuard Drop trait实现离开作用域时，自动释放lock。
/// 要实现多个线程中共享使用Mutex<T>，需要用到原子引用计数（atomically reference counted）Arc<T>；
/// Arc<T>是一个类似Rc<T>，但可以安全的用于并发环境的类型。
fn ln_mutex() {
    let m = Mutex::new(5); // Mutex内部放一个i32数据
    println!("m: {:?}", m);
    {
        let mut num = m.lock().unwrap(); // lock会阻塞当前线程，并返回指向内部数据的可变引用
        *num = 6;
        std::mem::drop(num); // Drop智能指针，达到主动释放锁的目的
        println!("num: {}", m.lock().unwrap()); // 不释放锁的话，这里会死锁
    }
    println!("num: {}", m.lock().unwrap()); // 或者通过{}构造作用域，防止死锁

    let cnt = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let cnt = Arc::clone(&cnt);
        let h = thread::spawn(move || {
            let mut num = cnt.lock().unwrap();
            *num += 1;
        });
        handles.push(h);
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("cnt: {}", cnt.lock().unwrap());
}


/// 条件量（condvar），用于同步。
fn ln_condvar() {
    let pair = Arc::new((Mutex::new(0), Condvar::new()));
    let pairc = pair.clone();

    let wait = thread::spawn(move || {
        let (lock, cvar) = &*pairc;
        loop {
            let mut val = lock.lock().unwrap();
            println!("wait cvar");
            //val = cvar.wait_timeout(val, Duration::from_millis(10)).unwrap().0;
            val = cvar.wait(val).unwrap();
            println!("wait: {}", val);
            if *val >= 5 { break; } // 直到val足够大时，退出线程
        }
    });

    let (lock, cvar) = &*pair;
    loop {
        // notify线程中需要sleep，不然notify break后，wait线程才wait cvar，
        // 从而导致cvar.wait()会一直等待下去（除非使用wait_timeout）
        thread::sleep(Duration::from_millis(100)); // 这里还未获取锁，故wait线程中可以获取锁，且运行到wait cvar
        let mut val = lock.lock().unwrap();
        *val += 1;
        cvar.notify_one();
        println!("notify: {}", val);
        if *val >= 5 {
            println!("notify break");
            break;
        }

        // 若在这里sleep，需要主动释放锁，不然notify线程在sleep期间依然占着锁，
        // 从而导到wait线程中拿不到锁，运行不到cvar.wait()
        //std::mem::drop(val);
        //thread::sleep(Duration::from_millis(100));
    }
    wait.join().unwrap();
}

pub fn run() {
    ln_channel();
    ln_mutex();
    ln_condvar()
}
