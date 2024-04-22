use crate::{get_hash, hash_search};
use std::io::Write;
use std::thread;
use std::time::{Duration, Instant};

/// проверка производительности функции поиска хешей
/// фунция выполняется с разными занчениями по двум параметрам:
/// * число искомых чисел
/// * число потоков
#[test]
#[ignore]
fn hash_search_performance() {
    // выполнение по разному колчиесву искомых чисел
    [50, 100]
        .map(|hash_nums_count| {
            thread::spawn(move || {
                let mut output = Vec::new();

                writeln!(output, "{hash_nums_count} чисел:").unwrap();
                let mut prev_end = Duration::new(0, 0);

                // выполнение по разному числу потоков
                [1, 100, 1000]
                    .map(|threads_count| {
                        thread::spawn(move || {
                            let mut output = Vec::new();

                            let start = Instant::now();
                            hash_search(3, hash_nums_count, threads_count);
                            let end = Instant::now() - start;

                            writeln!(
                                output,
                                "   {} потоков за {:?}. {}",
                                threads_count,
                                end,
                                if prev_end > end {
                                    format!(
                                        "в {:.2} раза быстрее",
                                        prev_end.as_secs_f64() / end.as_secs_f64()
                                    )
                                } else if prev_end == Duration::new(0, 0) {
                                    "".to_string()
                                } else {
                                    format!(
                                        "в {:.2} раза медленнее",
                                        end.as_secs_f64() / prev_end.as_secs_f64()
                                    )
                                }
                            )
                            .unwrap();
                            prev_end = end;

                            output
                        })
                    })
                    .map(|handle| output.extend(handle.join().unwrap()));

                output
            })
        })
        .map(|handle| {
            println!("{}", String::from_utf8(handle.join().unwrap()).unwrap());
        });
}

/// проверка чисел, хеш которых содержит 3 нуля в конце
#[test]
fn get_zero_hashes() {
    let check = |num| {
        let hash = get_hash(num);
        assert!(&hash[hash.len() - 3..] == "000")
    };

    check(25698);
    check(37024399);
    check(2022754);
    check(43024485);
    check(94027734);
    check(79024506);
}
