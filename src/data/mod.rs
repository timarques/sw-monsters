pub mod structs;

use crate::error::Error;
use crate::scraper;

use structs::monster::Monster;
use structs::collection::Collection;
use std::path::Path;
use std::fs::{create_dir_all};
use std::cell::UnsafeCell;

pub struct Data {
    data: UnsafeCell<Vec<Monster>>,
    path: String
}

impl Data {

    pub fn new<A: AsRef<Path>>(path: A) -> Self {
        let path = path.as_ref();
        assert!(path.is_file(), true);
        if !path.exists() {
            let mut path_buf = path.to_path_buf();
            path_buf.pop();
            create_dir_all(path_buf).unwrap();
        }

        let data = std::fs::read_to_string(&path).ok()
            .and_then(|string| serde_json::from_str::<Vec<Monster>>(&string).ok())
            .unwrap_or(Vec::new());

        Self {
            data: UnsafeCell::new(data),
            path: path.to_str().unwrap().to_string()
        }
    }

    pub fn init(&self) -> Result<(), Error> {
        if self.data().is_empty() {
            let fusions: Vec<Collection<String>> = serde_json::from_str(include_str!("../../data/fusions.json")).unwrap();
            *self.data_mut() = scraper::all()?.into_iter()
                .map(|monster| {
                    for fusion in fusions {
                        if monster.name == fusion.name {
                            monster.set_fusion(fusion.elements);
                            break;
                        }
                    }
                    monster
                }).collect();
        }
        Ok(())
    }

    pub fn save(&self) {
        if let Ok(json) = serde_json::to_string(self.data()) {
            std::fs::write(&self.path, json.as_bytes()).unwrap();
        }
    }

    pub fn data(&self) -> &Vec<Monster> {
        unsafe { &*self.data.get() }
    }

    fn data_mut(&self) -> &mut Vec<Monster> {
        unsafe { &mut *self.data.get() }
    }

    pub fn get<A: Into<String>>(&self, monster_name: A) -> Option<&mut Monster> {
        self.data_mut().iter_mut().find(|monster| monster.name == monster_name.into())
    }

}
