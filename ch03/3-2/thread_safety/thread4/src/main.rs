use std::sync::mpsc;
use std::thread;

fn main() {
    let mut handles = Vec::new();
    let mut data = vec![1; 10];
    let mut snd_channels = Vec::new();
    let mut rcv_channels = Vec::new();

    for xx in 0..10 {
        // mainから各スレッドへのチャンネル
        let (snd_tx, snd_rx) = mpsc::channel();
        // 各スレッドからmainへのチャンネル
        let (rcv_tx, rcv_rx) = mpsc::channel();
        snd_channels.push(snd_tx);
        rcv_channels.push(rcv_rx);
        println!("Start{}",xx);

        handles.push(thread::spawn(move || {
            // mainからの受け取り
            println!("{}::::0.RECV: main -> thread",xx);
            let mut data = snd_rx.recv().unwrap();
            data += xx;
            // mainへ送信
            println!("{}::::2.SEND: thread -> main",xx);
            let _ = rcv_tx.send(data);
        }));
    }

    println!("Middle-1");

    // 各スレッドにdataの値を送信
    for x in 0..10 {
        println!("{}::::1.SEND: main -> thread",x);
        let _ = snd_channels[x].send(data[0]);
    }

    println!("Middle-2");

    // 各スレッドからの結果をdataに格納
    for x in 0..10 {
        println!("{}::::3.RECV: thread -> main",x);
        data[x] = rcv_channels[9-x].recv().unwrap();
    }

    println!("Middle-3");

    for handle in handles {
        let _ = handle.join();
    }

    println!("End");

    dbg!(data);
}
