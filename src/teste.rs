mod scraper;
mod threadpool;
mod error;
mod traits;
mod utils;
mod data;

use std::sync::{Arc, Mutex};

fn main() -> Result<(), error::Error> {

    let threadpool = threadpool::ThreadPool::new();
    threadpool.workers(4);

    let data = Arc::new(Mutex::new(scraper::get_monsters()?));
    let monsters = (*data.lock().unwrap()).clone();

    for monster in monsters {
        let data = data.clone();
        let monster_name = monster.name.clone();
        threadpool.add(|| {

            println!("{}", std::mem::size_of_val(data));

            for monster in &*data.lock().unwrap() {
                if monster.name == monster_name {
                    monster = scraper::get_monster(monster).unwrap();
                    break;
                }
            }
        });
    }

    Ok(())

}
