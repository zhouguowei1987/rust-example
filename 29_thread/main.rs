use std::thread;
use std::time::Duration;
fn main(){
    // 子线程
    // thread::spawn(|| {
    //     for i in 1..10{
    //         println!("子线程 : {i}");
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    // 主线程，当主线程执行结束，子线程就自动结束
    // for i in 1..5{
    //     println!("主线程 : {i}");
    //     thread::sleep(Duration::from_millis(1));
    // }

    // thread::sleep() 会让线程睡眠一段时间，
    // 某个线程睡眠的时候会让出 CPU，可以让不同的线程交替执行，
    // 要看操作系统如何调度线程。

    // 上面的例子主线程结束后，子线程还没有运行完，但是子线程也结束了。
    // 如果想让子线程结束后，主线程再结束，
    // 我们就要使用Join 方法，把子线程加入主线程等待队列。

    // 子线程
    let handler = thread::spawn(|| {
        for i in 1..10{
            println!("子线程 : {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });
    // 主线程
    for i in 1..5{
        println!("主线程 : {i}");
        thread::sleep(Duration::from_millis(1));
    };
    // 主线程让子线程执行完毕后，主线程才退出
    handler.join().unwrap();

}