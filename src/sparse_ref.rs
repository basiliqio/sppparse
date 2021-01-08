use super::*;
use std::marker::PhantomData;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SparseRefRaw {
    #[serde(rename = "$ref")]
    ref_: String,
}

impl SparseRefRaw {
    pub fn get(&self) -> &String {
        &self.ref_
    }

    pub fn builder(&self) -> SparseRefBuilder {
        SparseRefBuilder::from(self)
    }
}

#[derive(Debug, Clone)]
pub struct SparseRefLocal<'a, S: Serialize + Deserialize<'a>> {
    val: Rc<S>,
    pointer: String,
    state: Rc<RefCell<SparseState>>,
    _l: PhantomData<&'a S>,
}

#[derive(Debug, Clone)]
pub struct SparseRef<'a, S: Serialize + Deserialize<'a>> {
    val: Rc<S>,
    pfile_path: Option<PathBuf>,
    pointer: String,
    state: Rc<RefCell<SparseState>>,
    _l: PhantomData<&'a S>,
}

pub trait SparseRefBase<'a, S: Serialize + Deserialize<'a>> {
    fn new(
        val: Rc<S>,
        state: Rc<RefCell<SparseState>>,
        pointer: String,
        pfile_path: Option<PathBuf>,
    ) -> Self;
    // fn new_from_value(val: Rc<Value>, pointer: String, pfile_path: Option<Rc<File>>) -> Self;
    fn get(&self) -> Rc<S>;
    fn state(&self) -> Rc<RefCell<SparseState>>;
    fn pointer(&self) -> &'_ String;
    fn can_handle_file() -> bool {
        false
    }
}

impl<'a, S> SparseRefBase<'a, S> for SparseRefLocal<'a, S>
where
    S: Serialize + Deserialize<'a>,
{
    fn get(&self) -> Rc<S> {
        self.val.clone()
    }

    fn pointer(&self) -> &'_ String {
        &self.pointer
    }

    fn state(&self) -> Rc<RefCell<SparseState>> {
        self.state.clone()
    }

    fn new(
        val: Rc<S>,
        state: Rc<RefCell<SparseState>>,
        pointer: String,
        _pfile_path: Option<PathBuf>,
    ) -> Self {
        SparseRefLocal {
            val,
            pointer,
            state,
            _l: PhantomData::default(),
        }
    }
}

impl<'a, S> SparseRefBase<'a, S> for SparseRef<'a, S>
where
    S: Serialize + Deserialize<'a>,
{
    fn get(&self) -> Rc<S> {
        self.val.clone()
    }

    fn pointer(&self) -> &'_ String {
        &self.pointer
    }

    fn state(&self) -> Rc<RefCell<SparseState>> {
        self.state.clone()
    }

    fn can_handle_file() -> bool {
        true
    }

    fn new(
        val: Rc<S>,
        state: Rc<RefCell<SparseState>>,
        pointer: String,
        pfile_path: Option<PathBuf>,
    ) -> Self {
        SparseRef {
            val,
            pointer,
            state,
            pfile_path,
            _l: PhantomData::default(),
        }
    }
}

impl<'a, S> SparseRef<'a, S>
where
    S: Serialize + Deserialize<'a>,
{
    pub fn pfile_path(&self) -> &Option<PathBuf> {
        &self.pfile_path
    }
}
