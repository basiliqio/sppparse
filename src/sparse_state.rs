use super::*;

#[derive(Debug, Clone)]
pub struct SparseState {
    map: Rc<HashMap<Option<PathBuf>, (File, Rc<Value>)>>,
}

impl SparseState {
    pub fn new() -> Self {
        SparseState {
            map: Rc::new(HashMap::new()),
        }
    }

    pub fn get_val(&self, s: &Option<PathBuf>) -> Option<Rc<Value>> {
        self.map.get(s).map(|x| x.1.clone())
    }

    pub fn get_file(&self, s: &Option<PathBuf>) -> Option<&File> {
        self.map.get(s).map(|x| &x.0)
    }
}
