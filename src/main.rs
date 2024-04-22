//! #hash_finder
//!
//! `hash_finder` - это консольное приложение, которое находит числа,
//! хеш (sha256) которого оканчивается N-символами нуля.
//!

#[cfg(test)]
mod tests;

use sha2::{Digest, Sha256};
use std::{
    env,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Instant,
};

/// Производиться поиск hash-чисел по заданным CLI аргументам.
fn main() {
    let [N, F, T] = initial_args();
    let start = Instant::now();
    hash_search(N, F, T);
    println!("\nВыполнено за {:?}", Instant::now() - start);
}

/// # Инициализация аргументов.
/// Аргументы
/// * -N - количество нулей в конце хеша
/// * -F - количество искомых чисел
/// * -T - число параллельных потоков (по умолчанию 1)
/// 
/// ## Примеры
/// ```
/// cargo run -- -N 4 -F 100
/// ```
/// по умолчанию число потоков равен 1, поэтому нет необходисости явно указывать аргумент -T
/// 
/// ```
/// cargo run -- -N 4 -F 100 -T 100
/// ```

fn initial_args() -> [usize; 3] {
    let args = env::args().collect::<Vec<String>>().into_iter();

    let get_arg = |name, description, default: Option<usize>| -> Result<usize, String> {
        match args.clone().position(|arg| arg == name) {
            Some(arg_i) => {
                let error_message =
                    format!("Укажите значение для аргумента {name}. Например: {name} 3");

                args.clone()
                    .nth(arg_i + 1)
                    .ok_or_else(|| error_message.clone())?
                    .parse::<usize>()
                    .map_err(|_| format!("{}", error_message))
            }
            None => default.ok_or_else(|| format!("Укажите аргумент {name} ({description}).")),
        }
    };

    // определние CLI аргументов в переменные с именем аргументов,
    // если аргументы были введены правльно
    macro_rules! arg_to_var {
        ($($var:ident => $decr:expr =>  $def:expr),+) => {
             let (arg_vals, errors) = [$(get_arg(format!("-{}", stringify!($var)), $decr, $def)),+].into_iter()
                .fold((vec![], vec![]), | (mut arg_vals, mut errors), arg| {
                    match arg {
                        Ok(val) => arg_vals.push(val),
                        Err(mess) => errors.push(mess)
                    }
                    (arg_vals, errors)
                });

            if !errors.is_empty() {
                panic!("{}", errors.join("\n"));
            }

            let [$($var),+] = arg_vals.try_into().unwrap();
        };
    }

    arg_to_var!(
        N => "колчиество нулей с конца хэш-кода" => None,
        F => "колчиество искомых чисел" => None,
        T => "количество потоков" => Some(1)
    );

    [N, F, T]
}

/// # Поиск хеш-чисел.
///
/// ## Параметры
///
/// 1. zero_count - количество нулей в конце дайджест-хеша
/// 2. hash_nums_count - количество искомых чисел
/// 3. threads_count - количество параллельных потоков
fn hash_search(zero_count: usize, hash_nums_count: usize, threads_count: usize) {
    let finded_count = Arc::new(Mutex::new(0));

    // Партия чисел, которые будет обрабатывать потоки.
    // Нужно для того, чтобы знать точное колчиестов чисел, для праспределения между потоками.
    // То есть если не все искомые числа не были найдены, то потоки будут обрабатывать следующую партию.
    // Тем самым достигается бесконечный поиск и одновременно распределение нагрузки на множество потоков.
    for butch_number in 1.. {
        let butch_count: usize = butch_number * 100_000_000;

        let handlers = (0..threads_count)
            .map(|thread_i| {
                let finded_count = finded_count.clone();

                thread::spawn(move || {
                    let butch_part = butch_count / threads_count;
                    for number in (thread_i * butch_part)..(thread_i + 1) * butch_part {
                        // если искомое количество чисел найдено, завершить поиск
                        let mut finded_count = finded_count.lock().unwrap();
                        if *finded_count == hash_nums_count {
                            break;
                        }

                        let hash = get_hash(number as u32);

                        if &hash[hash.len() - zero_count..] == "0".repeat(zero_count) {
                            *finded_count += 1;
                            println!("{number}, {hash}");
                        }
                    }
                })
            })
            .collect::<Vec<JoinHandle<()>>>();

        for hangle in handlers {
            hangle.join().unwrap();
        }

        // если найденное количество чисел удволетворят требуемому, то завершить,
        // иначе перейти к следующей партии чисел
        if *finded_count.lock().unwrap() == hash_nums_count {
            break;
        }
    }
}

/// Получение дайджест-хеш числа.
fn get_hash(number: u32) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&number.to_be_bytes());
    hasher
        .finalize()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}
