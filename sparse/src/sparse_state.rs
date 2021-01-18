use super::*;
use getset::{CopyGetters, Getters, MutGetters};
use rand::Rng;
use std::fs;
use std::io::{Seek, SeekFrom, Write};
use std::path::PathBuf;

#[derive(Clone, Debug, Copy)]
pub enum SparseFileFormat {
    Json(bool),
    Yaml,
}

impl std::default::Default for SparseFileFormat {
    fn default() -> Self {
        SparseFileFormat::Yaml
    }
}

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

    #[getset(get_copy = "pub")]
    ftype: SparseFileFormat,
}

impl SparseStateFile {
    /// Create a new state file providing the [Value](serde_json::Value).
    pub fn new(val: Value, ftype: SparseFileFormat) -> Self {
        let mut rng = rand::thread_rng();
        SparseStateFile {
            val,
            version: rng.gen_range(1..std::u64::MAX),
            ftype,
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
    fn read_file(path: PathBuf) -> Result<SparseStateFile, SparseError> {
        let file_json = fs::File::open(path.as_path())?;

        let json_res = serde_json::from_reader(file_json);
        if let Err(json_err) = json_res {
            if json_err.is_syntax() || json_err.is_data() {
                let file_yaml = fs::File::open(path.as_path())?;
                let val = serde_yaml::from_reader(file_yaml)?;
                let res = SparseStateFile::new(val, SparseFileFormat::Yaml);
                Ok(res)
            } else {
                Err(SparseError::SerdeJson(json_err))
            }
        } else {
            let res = SparseStateFile::new(json_res?, SparseFileFormat::Json(true));
            Ok(res)
        }
    }

    /// Create a new `SparseState` from a root file
    pub fn new_from_file(path: PathBuf) -> Result<Self, SparseError> {
        let mut map: HashMap<PathBuf, SparseStateFile> = HashMap::new();
        let path = SparseRefUtils::normalize_path(path, std::env::current_dir()?)?;
        let res = SparseState::read_file(path.clone())?;
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
        let path = SparseRefUtils::normalize_path(path, std::env::current_dir()?)?;
        let res = SparseStateFile::new(val, SparseFileFormat::Yaml);
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
        <S as SparsableTrait>::sparse_init(
            &mut res,
            self,
            &SparseRefUtils::new(String::from("/"), self.get_root_path().clone()),
        )?;
        Ok(res)
    }

    /// Deserialize a document from the state to the type S
    pub fn parse_file<S: DeserializeOwned + Serialize + SparsableTrait>(
        &mut self,
        path: PathBuf,
    ) -> Result<S, SparseError> {
        let path = SparseRefUtils::normalize_path(path, self.get_root_path().clone())?;
        let mut res: S = serde_json::from_value::<S>(
            self.map_raw
                .get(&path)
                .ok_or(SparseError::NotInState)?
                .val()
                .clone(),
        )?;
        <S as SparsableTrait>::sparse_init(
            &mut res,
            self,
            &SparseRefUtils::new(String::from("/"), self.get_root_path().clone()),
        )?;
        Ok(res)
    }

    /// Deserialize a file from the state to the type S
    pub fn add_value(&mut self, path: PathBuf, value: Value) -> Result<(), SparseError> {
        let path = SparseRefUtils::normalize_path(path, self.get_root_path().clone())?;
        if self.map_raw.contains_key(&path) {
            return Ok(());
        }
        self.map_raw
            .insert(path, SparseStateFile::new(value, SparseFileFormat::Yaml));
        Ok(())
    }

    /// Deserialize a file from the state to the type S
    pub fn add_obj<S: DeserializeOwned + Serialize + SparsableTrait>(
        &mut self,
        path: PathBuf,
        obj: &mut S,
    ) -> Result<(), SparseError> {
        let mut obj = obj;
        let path = SparseRefUtils::normalize_path(path, self.get_root_path().clone())?;
        <S as SparsableTrait>::sparse_init(
            &mut obj,
            self,
            &SparseRefUtils::new(String::from("/"), self.get_root_path().clone()),
        )?;
        self.map_raw.insert(
            path,
            SparseStateFile::new(serde_json::to_value(obj)?, SparseFileFormat::Yaml),
        );
        Ok(())
    }

    pub fn add_file(&mut self, path: &PathBuf) -> Result<(), SparseError> {
        let npath: PathBuf = match path.is_absolute() {
            true => path.clone(),
            false => SparseRefUtils::normalize_path(path.clone(), self.get_root_path().clone())?,
        };
        if self.map_raw.contains_key(&npath) {
            return Ok(());
        }
        if self.in_memory {
            return Err(SparseError::NoDistantFile);
        }
        let file = fs::File::open(npath.as_path())?;
        let val: Value = serde_json::from_reader(file)?;
        self.map_raw
            .insert(npath, SparseStateFile::new(val, SparseFileFormat::Yaml));
        Ok(())
    }

    fn write_file(
        file: &mut fs::File,
        state_file: &SparseStateFile,
        format: Option<SparseFileFormat>,
    ) -> Result<(), SparseError> {
        match format {
            Some(SparseFileFormat::Json(pretty)) => {
                let val = match pretty {
                    true => serde_json::to_string_pretty(state_file.val())?,
                    false => serde_json::to_string(state_file.val())?,
                };
                file.set_len(0)?;
                file.write_all(val.as_bytes())?;
                file.sync_all()?;
            }
            Some(SparseFileFormat::Yaml) => {
                let val = serde_yaml::to_string(state_file.val())?;
                file.set_len(0)?;
                file.write_all(val.as_bytes())?;
                file.sync_all()?;
            }
            None => {
                SparseState::write_file(file, state_file, Some(state_file.ftype()))?;
            }
        }
        Ok(())
    }

    /// Write all the files in the states to disks
    /// It'll try not to modify anything until it's sure it can open every file
    /// for writing
    pub fn save_to_disk(&self, format: Option<SparseFileFormat>) -> Result<(), SparseError> {
        let mut files: Vec<(fs::File, &SparseStateFile)> = Vec::new();

        for (path_buf, val) in self.map_raw.iter() {
            let mut file: fs::File = fs::OpenOptions::new()
                .append(true)
                .open(path_buf.as_path())?;
            file.seek(SeekFrom::Start(0))?;
            files.push((
                fs::OpenOptions::new()
                    .append(true)
                    .open(path_buf.as_path())?,
                val,
            ));
        }
        for (mut file, sparse_state_file) in files.into_iter() {
            SparseState::write_file(&mut file, sparse_state_file, format)?;
        }
        Ok(())
    }
}
