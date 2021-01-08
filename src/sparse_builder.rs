use super::*;

#[derive(Debug, Getters, Clone)]
#[getset(get = "pub")]
pub struct SparseRefBuilder {
    pfile_path: Option<PathBuf>,
    pointer: String,
}

impl From<SparseRefRaw> for SparseRefBuilder {
    fn from(ref_: SparseRefRaw) -> SparseRefBuilder {
        SparseRefBuilder::from(&ref_)
    }
}

impl SparseRefBuilder {
    pub fn parse_json_file<'a, S: Serialize + DeserializeOwned>(
        state: &Rc<RefCell<SparseState>>,
        pfile_path: &PathBuf,
    ) -> Result<S, SparseError> {
        let file: File = File::open(pfile_path.to_owned())?;
        let val: Value = serde_json::from_reader(file)?;

        state
            .borrow()
            .get_map()
            .borrow_mut()
            .insert(Some(pfile_path.clone()), Rc::new(RefCell::new(val.clone())));
        Ok(serde_json::from_value(val)?)
    }

    pub fn parse_yaml_file<'a, S: Serialize + DeserializeOwned>(
        state: &Rc<RefCell<SparseState>>,
        pfile_path: &PathBuf,
    ) -> Result<S, SparseError> {
        let file: File = File::open(pfile_path.to_owned())?;
        let val: Value = serde_yaml::from_reader(file)?;

        state
            .borrow()
            .get_map()
            .borrow_mut()
            .insert(Some(pfile_path.clone()), Rc::new(RefCell::new(val.clone())));
        Ok(serde_json::from_value(val)?)
    }

    pub fn build_owned<'a, S: Serialize + DeserializeOwned, T: SparseRefBase<'a, S>>(
        &self,
        state: &Rc<RefCell<SparseState>>,
    ) -> Result<T, SparseError> {
        match &self.pfile_path {
            Some(pfile_path) => {
                if T::can_handle_file() != true {
                    return Err(SparseError::NoDistantFile);
                }
                let extension: &str = pfile_path
                    .extension()
                    .ok_or(SparseError::BadExtension(None))?
                    .to_str()
                    .ok_or(SparseError::BadExtension(None))?;
                let val: S = match extension {
                    "json" => Self::parse_json_file(state, &pfile_path)?,
                    "yaml" | "yml" => Self::parse_json_file(state, &pfile_path)?,
                    _ => return Err(SparseError::BadExtension(Some(extension.to_string()))),
                };
                Ok(T::new(
                    Rc::new(val),
                    state.clone(),
                    self.pointer.clone(),
                    Some(pfile_path.clone()),
                ))
            }
            None => {
                let val: Rc<RefCell<Value>> = state
                    .borrow()
                    .get_val(&None)
                    .ok_or(SparseError::NotInState)?;
                let deref_val: Value = val
                    .borrow()
                    .pointer(self.pointer.as_str())
                    .ok_or(SparseError::UnkownPath(self.pointer.clone()))?
                    .clone();
                let parsed_val: S = serde_json::from_value::<S>(deref_val)?;
                Ok(T::new(
                    Rc::new(parsed_val),
                    state.clone(),
                    self.pointer.clone(),
                    None,
                ))
            }
        }
    }
}

impl From<&SparseRefRaw> for SparseRefBuilder {
    fn from(ref_: &SparseRefRaw) -> SparseRefBuilder {
        let mut pointer_str: String = ref_.get().clone();
        let hash_pos: Option<usize> = pointer_str.find("#");
        let pfile: Option<PathBuf>;
        let pointer_path_str: String;

        match hash_pos {
            Some(pos) => match pos {
                0 => {
                    pfile = None;
                    pointer_path_str = (&pointer_str[0..pos]).to_string();
                }
                _ => {
                    pointer_path_str =
                        (&(pointer_str.split_off(pos))[1..pointer_str.len()]).to_string();
                    pfile = Some(PathBuf::from(pointer_path_str.as_str()));
                }
            },
            None => {
                pfile = None;
                pointer_path_str = pointer_str.clone();
            }
        };
        SparseRefBuilder {
            pfile_path: pfile,
            pointer: pointer_path_str,
        }
    }
}
