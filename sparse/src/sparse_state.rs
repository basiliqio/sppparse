use super::*;
use getset::{CopyGetters, Getters, MutGetters};
use rand::Rng;
use std::fs;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Clone, Getters, MutGetters, CopyGetters)]
pub struct SparseStateFile {
    /// The value of the file, unparsed.
    #[getset(get = "pub", get_mut = "pub(crate)")]
    val: Value,
    /// The version of the file. It's a random value that is incremented each time
    /// the original object is modified. It forces the pointing [SparseRef](crate::SparseRef) to update
    /// their deserialized value when their version mismatch.
    #[getset(get_copy = "pub")]
    version: u64,
}

impl SparseStateFile {
    /// Create a new state file providing the [Value](serde_json::Value).
    pub fn new(val: Value) -> Self {
        let mut rng = rand::thread_rng();
        SparseStateFile {
            val,
            version: rng.gen_range(1..std::u64::MAX),
        }
    }

    pub fn bump_version(&mut self) {
        self.version += 1;
    }

    /// Replace the [Value](serde_json::Value) of the [SparseStateFile](crate::SparseStateFile) and increment its version.
    pub fn replace(&mut self, val: Value) {
        self.val = val;
        self.bump_version();
    }
}

#[derive(Debug, Clone, Getters, MutGetters)]
pub struct SparseState {
    /// A map between the absolute path (if any), of the file and their [SparseStateFile](SparseStateFile)
    map_raw: HashMap<PathBuf, SparseStateFile>,
    /// The path of the file, if it's not in-memory
    root_base: PathBuf,
    /// True if this is an in-memory state
    in_memory: bool,
}

impl SparseState {
    /// Create a new `SparseState` from a root file
    pub fn new_from_file(path: PathBuf) -> Result<Self, SparseError> {
        let mut map: HashMap<PathBuf, SparseStateFile> = HashMap::new();

        let path = path.absolutize()?.to_path_buf();
        let file = fs::File::open(path.as_path())?;
        let val: Value = serde_json::from_reader(file)?;
        let res = SparseStateFile::new(val);
        map.insert(path.clone(), res);
        Ok(SparseState {
            map_raw: map,
            root_base: path,
            in_memory: false,
        })
    }

    /// Create a new `SparseState` from an in memory Value
    pub fn new_from_value(path: PathBuf, val: Value) -> Result<Self, SparseError> {
        let mut map: HashMap<PathBuf, SparseStateFile> = HashMap::new();

        let path = path.absolutize()?.to_path_buf();
        let res = SparseStateFile::new(val);
        map.insert(path.clone(), res);
        Ok(SparseState {
            map_raw: map,
            root_base: path,
            in_memory: true,
        })
    }

    /// Get the root path of the state, if any
    pub fn get_root_path(&self) -> &PathBuf {
        &self.root_base
    }

    pub fn get_state_file<'a>(
        &'a self,
        path: &PathBuf,
    ) -> Result<&'a SparseStateFile, SparseError> {
        Ok(self.map_raw.get(path).ok_or(SparseError::NotInState)?)
    }

    pub(crate) fn get_state_file_mut(
        &mut self,
        path: &PathBuf,
    ) -> Result<&mut SparseStateFile, SparseError> {
        Ok(self.map_raw.get_mut(path).ok_or(SparseError::NotInState)?)
    }

    /// Deserialize the root document from the state to the type S
    pub fn parse_root<S: DeserializeOwned + Serialize + SparsableTrait>(
        &mut self,
    ) -> Result<S, SparseError> {
        let mut res: S = serde_json::from_value::<S>(
            self.map_raw
                .get(self.get_root_path())
                .ok_or(SparseError::NotInState)?
                .val()
                .clone(),
        )?;
        <S as SparsableTrait>::sparse_init(&mut res, self)?;
        Ok(res)
    }

    /// Deserialize a document from the state to the type S
    pub fn parse_file<S: DeserializeOwned + Serialize + SparsableTrait>(
        &mut self,
        path: &PathBuf,
    ) -> Result<S, SparseError> {
        let mut res: S = serde_json::from_value::<S>(
            self.map_raw
                .get(path)
                .ok_or(SparseError::NotInState)?
                .val()
                .clone(),
        )?;
        <S as SparsableTrait>::sparse_init(&mut res, self)?;
        Ok(res)
    }

    /// Deserialize a file from the state to the type S
    pub fn add_value<S: DeserializeOwned + Serialize + SparsableTrait>(
        &mut self,
        path: PathBuf,
        value: Value,
    ) -> Result<S, SparseError> {
        let mut val: S = serde_json::from_value(value.clone())?;
        self.map_raw
            .insert(path.clone(), SparseStateFile::new(value));
        let res = <S as SparsableTrait>::sparse_init(&mut val, self);
        match res {
            Ok(()) => Ok(val),
            Err(err) => {
                self.map_raw.remove(&path);
                Err(err)
            }
        }
    }

    /// Deserialize a file from the state to the type S
    pub fn add_obj<S: DeserializeOwned + Serialize + SparsableTrait>(
        &mut self,
        path: PathBuf,
        obj: &mut S,
    ) -> Result<(), SparseError> {
        let mut obj = obj;
        <S as SparsableTrait>::sparse_init(&mut obj, self)?;
        self.map_raw
            .insert(path, SparseStateFile::new(serde_json::to_value(obj)?));
        Ok(())
    }

    pub fn add_file_from_memory(&mut self, npath: PathBuf, val: Value) -> Result<(), SparseError> {
        if self.map_raw.contains_key(&npath) {
            return Ok(());
        }
        self.map_raw.insert(npath, SparseStateFile::new(val));
        Ok(())
    }

    pub fn add_file(&mut self, path: &PathBuf) -> Result<(), SparseError> {
        let npath: PathBuf = match path.is_absolute() {
            true => path.absolutize()?.to_path_buf(),
            false => {
                let mut base_path: PathBuf = self.get_root_path().clone();
                base_path.pop(); // Remove the file name
                base_path.push(path.as_path());
                base_path.absolutize()?.to_path_buf()
            }
        };
        if self.map_raw.contains_key(&npath) {
            return Ok(());
        }
        if self.in_memory {
            return Err(SparseError::NoDistantFile);
        }
        let file = fs::File::open(npath.as_path())?;
        let val: Value = serde_json::from_reader(file)?;
        self.map_raw.insert(npath, SparseStateFile::new(val));
        Ok(())
    }

    // /// Write all the files in the states to disks
    // /// It'll try not to modify anything until it's sure it can open every file
    // /// for writing
    // pub fn save_to_disk(&self, pretty: bool) -> Result<(), SparseError> {
    //     let mut files: Vec<(fs::File, &SparseStateFile)> = Vec::new();

    //     for (path_buf, val) in self.map_raw.iter() {
    //         let path = path_buf.as_ref().ok_or(SparseError::NoDistantFile)?;
    //         let mut file = fs::OpenOptions::new().append(true).open(path)?;
    //         file.seek(SeekFrom::Start(0))?;
    //         files.push((fs::OpenOptions::new().append(true).open(path)?, val));
    //     }
    //     for (mut file, state_file) in files.into_iter() {
    //         let val = match pretty {
    //             true => serde_json::to_string_pretty(state_file.val())?,
    //             false => serde_json::to_string(state_file.val())?,
    //         };
    //         file.write_all(val.as_bytes())?;
    //         file.set_len(val.len() as u64)?;
    //     }
    //     Ok(())
    // }
}
