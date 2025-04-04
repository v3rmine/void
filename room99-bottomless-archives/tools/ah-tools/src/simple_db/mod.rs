use std::sync::Mutex;
use std::fs::File;
use std::path::Path;
use std::io::{Write, Read};
use base64::{encode, decode};
use std::collections::HashMap;

pub struct SimpleDB<'a> {
    pub inner: Mutex<SimpleDBInner<'a>>
}

#[derive(Debug)]
pub struct SimpleDBInner<'a> {
    pub path: &'a str
}

macro_rules! err {
    ($reason:expr) => {|_| SimpleDBError::new($reason)};
}

macro_rules! string_from_bytes {
    ($bytes:expr) => { String::from_utf8($bytes).unwrap() };
}

const DELIM: char = '@';

impl<'a> SimpleDB<'a> {
    pub fn init(path: &'a str) -> Result<Self, SimpleDBError> {
        if !Path::new(path).is_file() {
            File::create(path).map_err(err!("Cannot create file"))?;
        }
        Ok(Self { inner: Mutex::new(SimpleDBInner { path }) })
    }
    pub fn add(&self, key: &str, val: &str) -> Result<(), SimpleDBError> {
        if self.get(&(*key))?.is_none() {
            let locked = self.inner.lock().unwrap();
            let mut f = std::fs::OpenOptions::new()
                .append(true)
                .open(&locked.path)
                .map_err(err!("Cannot open file"))?;
            f.write(format!("{}{}{}\n",
                            encode(key.as_bytes()),
                            DELIM,
                            encode(val.as_bytes())).as_bytes()
            ).map_err(err!("Cannot write in the file"))?;
        }
        Ok(())
    }
    pub fn get(&self, key: &str) -> Result<Option<String>, SimpleDBError> {
        let locked = self.inner.lock().unwrap();
        let mut f = File::open(locked.path).map_err(err!("Cannot open file"))?;
        let mut file = String::new();
        f.read_to_string(&mut file).map_err(err!("Cannot read file"))?;
        let content: HashMap<String,String> = file.split('\n').map(|line: &str| {
            //println!("{}", line);
            match line.split(DELIM).collect::<Vec<&str>>()[..] {
                [f, s] => Some((string_from_bytes!(decode(f).unwrap()), string_from_bytes!(decode(s).unwrap()))),
                _ => None,
            }
        }).filter_map(|e| match e { Some(e) => Some(e), None => None })
        .collect::<HashMap<String,String>>();
        Ok(match content.get(key) { Some(x) => Some(x.clone()), None => None})
    }
    pub fn remove(&self, key: &str) -> Result<(), SimpleDBError> {
        let locked = self.inner.lock().unwrap();
        let mut f = File::open(locked.path).map_err(err!("Cannot open file"))?;
        let mut file = String::new();
        f.read_to_string(&mut file).map_err(err!("Cannot read file"))?;
        f.set_len(0).map_err(err!("Cannot clean file"))?;
        let mut content: HashMap<String,String> = file.split('\n').map(|line: &str| {
            //println!("{}", line);
            match line.split(DELIM).collect::<Vec<&str>>()[..] {
                [f, s] => Some((string_from_bytes!(decode(f).unwrap()), string_from_bytes!(decode(s).unwrap()))),
                _ => None,
            }
        }).filter_map(|e| match e { Some(e) => Some(e), None => None })
        .collect::<HashMap<String,String>>();
        content.retain(|x, _| x != key);
        for (key, val) in content {
            f.write(format!("{}{}{}\n", encode(key.as_bytes()), DELIM, encode(val.as_bytes())).as_bytes()).map_err(err!("Cannot write to DB"))?;
        }
        Ok(())
    }
    pub fn update(&self, key: &str, new_val: &str) -> Result<(), SimpleDBError> {
        let locked = self.inner.lock().unwrap();
        let mut f = File::open(locked.path).map_err(err!("Cannot open file"))?;
        let mut file = String::new();
        f.read_to_string(&mut file).map_err(err!("Cannot read file"))?;
        f.set_len(0).map_err(err!("Cannot clean file"))?;
        let mut content: HashMap<String,String> = file.split('\n').map(|line: &str| {
            //println!("{}", line);
            match line.split(DELIM).collect::<Vec<&str>>()[..] {
                [f, s] => Some((string_from_bytes!(decode(f).unwrap()), string_from_bytes!(decode(s).unwrap()))),
                _ => None,
            }
        }).filter_map(|e| match e { Some(e) => Some(e), None => None })
        .collect::<HashMap<String,String>>();
        if content.contains_key(key) {
            content.insert(key.to_owned(), new_val.to_owned());
        }
        for (key, val) in content {
            f.write(format!("{}{}{}\n", encode(key.as_bytes()), DELIM, encode(val.as_bytes())).as_bytes()).map_err(err!("Cannot write to DB"))?;
        }
        Ok(())
    }
    pub fn get_all(&self) -> Result<HashMap<String,String>, SimpleDBError> {
        let locked = self.inner.lock().unwrap();
        let mut f = File::open(locked.path).map_err(err!("Cannot open file"))?;
        let mut file = String::new();
        f.read_to_string(&mut file).map_err(err!("Cannot read file"))?;
        let content: HashMap<String,String> = file.split('\n').map(|line: &str| {
            //println!("{}", line);
            match line.split(DELIM).collect::<Vec<&str>>()[..] {
                [f, s] => Some((string_from_bytes!(decode(f).unwrap()), string_from_bytes!(decode(s).unwrap()))),
                _ => None,
            }
        }).filter_map(|e| match e { Some(e) => Some(e), None => None })
        .collect::<HashMap<String,String>>();
        Ok(content)
    }
}

#[derive(Debug)]
pub struct SimpleDBError<'a> {
    _value: &'a str
}

impl<'a> SimpleDBError<'a> {
    fn new(reason: &'a str) -> Self {
        Self { _value: reason }
    }
}

impl<'a> std::fmt::Display for SimpleDBError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", &self._value)
    }
}
impl<'a> std::error::Error for SimpleDBError<'a> {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}