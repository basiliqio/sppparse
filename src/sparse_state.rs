use super::*;
use std::borrow::BorrowMut;

#[derive(Debug, Clone)]
pub struct SparseState {
    map: Rc<RefCell<HashMap<Option<PathBuf>, Rc<RefCell<Value>>>>>,
}

impl SparseState {
    pub fn new() -> Self {
        SparseState {
            map: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn get_val(&self, s: &Option<PathBuf>) -> Option<Rc<RefCell<Value>>> {
        self.map.borrow().get(s).map(|x| x.clone())
    }

    pub fn get_map(&self) -> Rc<RefCell<HashMap<Option<PathBuf>, Rc<RefCell<Value>>>>> {
        self.map.clone()
    }
}
