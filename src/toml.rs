use std::{
    fs::File,
    io::{Error, Read},
};
use toml_edit::DocumentMut;

pub fn merge_toml_documents(base: &mut DocumentMut, new: &DocumentMut) {
    // TODO: Merge correctly
    for (key, item) in new.as_table() {
        base[key] = item.clone();
    }
}

pub fn open_toml_document(path: &str) -> Result<DocumentMut, Error> {
    let mut content = String::new();

    {
        let mut file = File::open(path)?;
        file.read_to_string(&mut content)?;
    }

    content
        .parse::<DocumentMut>()
        .map_err(|_| Error::new(std::io::ErrorKind::InvalidData, "Could not parse toml file"))
}
