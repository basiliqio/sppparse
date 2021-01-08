use super::*;

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
pub struct SparseRefLocal {
    val: Rc<Value>,
    pointer: String,
}

#[derive(Debug, Clone)]
pub struct SparseRef {
    val: Rc<Value>,
    pfile_path: Option<Rc<File>>,
    pointer: String,
}

pub trait SparseRefBase {
    fn new(val: Rc<Value>, pointer: String, pfile_path: Option<Rc<File>>) -> Self;
    fn get(&self) -> Rc<Value>;
    fn pointer(&self) -> &'_ String;
    fn can_handle_file() -> bool {
        false
    }
}

impl SparseRefBase for SparseRefLocal {
    fn get(&self) -> Rc<Value> {
        self.val.clone()
    }

    fn pointer(&self) -> &'_ String {
        &self.pointer
    }

    fn new(val: Rc<Value>, pointer: String, _pfile_path: Option<Rc<File>>) -> Self {
        SparseRefLocal { val, pointer }
    }
}

impl SparseRefBase for SparseRef {
    fn get(&self) -> Rc<Value> {
        self.val.clone()
    }

    fn pointer(&self) -> &'_ String {
        &self.pointer
    }

    fn can_handle_file() -> bool {
        true
    }

    fn new(val: Rc<Value>, pointer: String, pfile_path: Option<Rc<File>>) -> Self {
        SparseRef {
            val,
            pointer,
            pfile_path,
        }
    }
}
