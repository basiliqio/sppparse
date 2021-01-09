use super::*;
use std::borrow::BorrowMut;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct SparseState<'a> {
    map: Rc<RefCell<HashMap<Option<PathBuf>, RefCell<Value>>>>,
    _l: PhantomData<&'a str>,
}

impl<'a> SparseState<'a> {
    pub fn new() -> Self {
        SparseState {
            map: Rc::new(RefCell::new(HashMap::new())),
            _l: PhantomData::default(),
        }
    }

    pub fn get_val(&self, s: &Option<PathBuf>) -> Option<RefCell<Value>> {
        self.map.borrow().get(s).map(|x| x.clone())
    }

    pub fn get_map(&self) -> Rc<RefCell<HashMap<Option<PathBuf>, RefCell<Value>>>> {
        self.map.clone()
    }

    pub fn parse<S: DeserializeOwned>(
        &self,
        path: Option<PathBuf>,
        value: Value,
    ) -> Result<S, SparseError> {
        let val: S = serde_json::from_value(value.clone())?;
        (*self.map).borrow_mut().insert(path, RefCell::new(value));
        Ok(val)
    }
}
