use std::{
    env::args,
    fmt::Write,
    thread,
    time::{Duration, Instant},
};

use advance::{ProgressBar, ProgressBarIterable};
use thread::sleep;

#[tokio::main]
async fn main() {
    // match args().nth(1).unwrap().as_str() {
    //     "simple" => simple(),
    //     "simple2" => simple2(),
    //     "multiple" => multiple(),
    //     "split_weighted" => split_weighted(),
    //     "abandonment" => abandonment(),
    //     "indeterminate" => indeterminate(),
    //     "print_during_progress" => print_during_progress(),
    //     "more_splitting" => more_splitting2().await,
    //     "split_each" => split_each(),
    //     "split_summed" => split_summed().await,
    //     "split_sized" => split_sized(),
    //     _ => return,
    // }
    // return;

    for i in ProgressBar::new().wrap(0..) {
        if i > 100 {
            break;
        }
        sleep(Duration::from_millis(10));
    }
    return;

    // thread::spawn(|| {
    //     for i in (0..100).progress() {
    //         if i > 20 {
    //             panic!("Something went wrong!");
    //         }
    //         thread::sleep(Duration::from_millis(50));
    //     }
    // })
    // .join()
    // .ok();

    // let p = ProgressBar::new();
    // for (p, i) in p.split_each(0..10) {
    //     p.set_message(format!("Processing {}", i));
    //     if i == 5 {
    //         continue;
    //     }
    //     for _ in p.wrap(0..100) {
    //         thread::sleep(Duration::from_millis(10));
    //     }
    // }
    let mut p = ProgressBar::new().split_weighted();
    for i in (0..).progress_with(p.take(0.8)) {
        sleep(Duration::from_millis(300));
        if i > 100 {
            break;
        }
    }
    p.take(0.2);
    return;

    let mut p = ProgressBar::new().split_weighted();
    let p1 = p.take(0.5);
    for i in (0..50).progress_with(p1) {
        sleep(Duration::from_millis(30));
    }
    let mut p2 = p.take(0.5);
    p2.set_length(50);
    for i in 0..50 {
        p2.inc();
        sleep(Duration::from_millis(30));
    }
    p2.finish();

    // for i in ProgressBar::new().wrap(0..1) {
    //     thread::sleep(Duration::from_millis(10000));
    // }

    // for i in ProgressBar::new().wrap(0..100) {
    //     thread::sleep(Duration::from_millis(10));
    // }

    // for i in ProgressBar::new().wrap(0..10) {
    //     thread::sleep(Duration::from_millis(1000));
    // }

    // for i in ProgressBar::new().wrap(0..10) {
    //     println!("{}", i);
    //     thread::sleep(Duration::from_millis(1000));
    // }

    // let p1 = ProgressBar::new();
    // let p2 = ProgressBar::new();
    // for i in p2.wrap(p1.wrap(0..100)) {
    //     if i % 2 == 0 {
    //         for _ in 0..1 {
    //             println!("{}", i);
    //         }
    //     }
    //     if i == 50 {
    //         println!("Halfway!");
    //     }
    //     thread::sleep(Duration::from_millis(20));
    // }

    let mut p2 = ProgressBar::new();
    let p1 = ProgressBar::new();
    p2.set_length(50);
    for i in p1.wrap(0..100) {
        if i < 50 {
            p2.inc();
        } else {
            p2.finish();
        }
        // if i % 2 == 0 {
        for _ in 0..1 {
            println!("{}", i);
        }
        // }
        if i == 50 {
            println!("Halfway!");
        }
        sleep(Duration::from_millis(20));
    }

    // {
    //     let mut progress = ProgressBar::new();
    //     progress.set_message("Will be abandoned");
    //     progress.set_length(10);
    //     for i in 0..8 {
    //         progress.inc();
    //         thread::sleep(Duration::from_millis(50));
    //     }
    //     progress.set_message("Abandoned");
    // }

    // let handles = (0..10)
    //     .map(|i| {
    //         thread::spawn(move || {
    //             for _ in ProgressBar::new().wrap(0..10) {
    //                 // println!("{}", i);
    //                 thread::sleep(Duration::from_millis(100 + 1 * i));
    //             }
    //             println!("Done");
    //         })
    //     })
    //     .collect::<Vec<_>>();
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    // println!("All done");

    // for _ in 0..4 {
    //     thread::sleep(Duration::from_millis(100));
    //     {
    //         let mut progress = ProgressBar::new();
    //         progress.set_message("Will be abandoned");
    //         progress.set_length(10);
    //         for i in 0..10 {
    //             progress.inc();
    //             thread::sleep(Duration::from_millis(50));
    //         }
    //         progress.set_message("Abandoned");
    //     }
    // }

    // let p = ProgressBar::new();
    // p.set_message("Early in the progress");
    // p.set_length(2000);
    // for i in 0..2000 {
    //     p.inc();
    //     if i > 1000 {
    //         p.set_message("");
    //     }
    //     thread::sleep(Duration::from_millis(1));
    // }

    // let p = ProgressBar::new();
    // for i in 0..2 {
    //     let p = p.nest_weighted(0.5);
    //     thread::spawn(move || {
    //         p.set_message(format!("Doing step {}", i));
    //         for j in p.wrap(0..10) {
    //             thread::sleep(Duration::from_millis(5 + i * 200));
    //         }
    //     });
    // }

    // thread::sleep(Duration::from_millis(100 * 150));

    let mut p = ProgressBar::new()
        .with_message("Doing stuffs")
        .split_weighted();
    sleep(Duration::from_millis(500));
    let p1 = p.take(0.5).with_message("Step 1");
    let p2 = p.take(0.5).with_message("Step 2");
    // Verify that dropping the original progress bar works
    drop(p);
    for _ in p1.wrap(0..1000) {
        sleep(Duration::from_millis(1));
    }
    for _ in p2.wrap(0..1000) {
        sleep(Duration::from_millis(2));
    }

    let p = ProgressBar::new().with_message("Doing stuffs");
    p.set_length(5);
    let mut p = p.split_sized();
    for i in 0..10 {
        for _ in p
            .take(1)
            .with_message(format!("Doing step {}", i))
            .wrap(0..200)
        {
            sleep(Duration::from_millis(5));
        }
    }
}
