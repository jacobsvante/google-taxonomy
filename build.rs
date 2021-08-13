use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;

// From http://www.google.com/basepages/producttype/taxonomy-with-ids.en-US.txt
static TAXONOMY_PATH: &str = "taxonomy-with-ids.en-US.txt";

struct RawProductCategory {
    id: u32,
    name: String,
}

impl RawProductCategory {
    fn variant_name(&self) -> String {
        self.name
            .replace("&", "And")
            .chars()
            .filter(|&c| c.is_ascii_alphanumeric())
            .collect()
    }
}

fn parse() -> Vec<RawProductCategory> {
    let reader = BufReader::new(File::open(TAXONOMY_PATH).expect("Couldn't read data"));
    reader
        .lines()
        .skip(1)
        // .take(1000)  // NOTE: Can be used to speed up compilation...
        .into_iter()
        .enumerate()
        .map(|(line_no, line)| {
            let line =
                line.unwrap_or_else(|_| panic!("Problems reading line {} from text file", line_no));

            let columns: Vec<&str> = line.splitn(2, " - ").collect();

            RawProductCategory {
                id: columns[0]
                    .parse()
                    .unwrap_or_else(|_| panic!("Could not parse id for {}", &line)),
                name: columns[1].into(),
            }
        })
        .collect()
}

#[rustfmt::skip]
fn write_enum(
    file: &mut BufWriter<File>,
    entries: &mut [RawProductCategory],
) -> Result<(), std::io::Error> {
    entries.sort_by(|a, b| a.name.cmp(&b.name) );
    writeln!(file, "    pub static ALL_NAME_SORTED: [ProductCategory; {}] = [", entries.len())?;
    for entry in entries.iter() {
        writeln!(file, "        ProductCategory::{},", &entry.variant_name())?;
    }
    writeln!(file, "    ];")?;
    entries.sort_by(|a, b| a.id.cmp(&b.id) );
    writeln!(file, "#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]")?;
    writeln!(file, "pub struct ProductCategory {{ id: u32, name: &'static str, variant_name: &'static str }}")?;
    writeln!(file, "impl ProductCategory {{")?;
    writeln!(file, "    pub const ALL: [ProductCategory; {}] = [", entries.len())?;
    for entry in entries.iter() {
        writeln!(file, "        ProductCategory::{},", &entry.variant_name())?;
    }
    writeln!(file, "    ];")?;
    for entry in entries.iter() {
        writeln!(file, "    #[allow(nonstandard_style)]")?;
        writeln!(file, "    pub const {}: ProductCategory = ProductCategory {{ id: {}, name: \"{}\", variant_name: \"{}\" }};", &entry.variant_name(), &entry.id, &entry.name, &entry.variant_name())?;
    }
    writeln!(file, "}}")?;
    writeln!(file)?;
    writeln!(file, "impl ProductCategory {{")?;
    writeln!(file, "    pub fn id(&self) -> u32 {{")?;
    writeln!(file, "        self.id")?;
    writeln!(file, "    }}")?;
    writeln!(file, "    pub fn name(&self) -> &str {{")?;
    writeln!(file, "        self.name")?;
    writeln!(file, "    }}")?;
    writeln!(file, "    pub fn from_id(id: u32) -> Result<Self, Error> {{")?;
    writeln!(file, "        Self::ALL.binary_search_by_key(&id, |a| a.id).map(|idx| Self::ALL[idx].clone()).map_err(|_| Error::IdNotFound)")?;
    writeln!(file, "    }}")?;
    writeln!(file, "    pub fn from_name(name: &str) -> Result<Self, Error> {{")?;
    writeln!(file, "        ALL_NAME_SORTED.binary_search_by_key(&name, |a| a.name).map(|idx| ALL_NAME_SORTED[idx].clone()).map_err(|_| Error::NameNotFound)")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;
    writeln!(file, "impl fmt::Display for ProductCategory {{")?;
    writeln!(file, "    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{")?;
    writeln!(file, "        write!(f, \"{{}}\", self.name)")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;
    writeln!(file, "impl fmt::Debug for ProductCategory {{")?;
    writeln!(file, "    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{")?;
    writeln!(file, "        write!(f, \"{{}}\", self.variant_name)")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("enum.rs");
    let mut entries = parse();
    let mut file = BufWriter::new(File::create(&out_path).expect("Couldn't write to output file"));
    write_enum(&mut file, &mut entries)?;
    Ok(())
}
