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
    entries: &[RawProductCategory],
) -> Result<(), std::io::Error> {
    writeln!(file, "/// ")?;
    writeln!(file, "#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]")?;
    writeln!(file, "pub enum ProductCategory {{")?;
    for entry in entries.iter() {
        writeln!(file, "    {} = {},", &entry.variant_name(), &entry.id)?;
    }
    writeln!(file, "}}")?;
    writeln!(file)?;
    writeln!(file, "impl ProductCategory {{")?;
    writeln!(file, "    pub fn from_id(id: u32) -> Result<Self, Error> {{")?;
    writeln!(file, "        match id {{")?;
    for entry in entries.iter() {
        writeln!(
            file,
            "            {} => Ok(Self::{}),",
            entry.id,
            entry.variant_name()
        )?;
    }
    writeln!(file, "            _ => Err(Error::IdNotFound)")?;
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "")?;
    writeln!(file, "    pub fn from_name(name: &str) -> Result<Self, Error> {{")?;
    writeln!(file, "        match name {{")?;
    for entry in entries.iter() {
        writeln!(
            file,
            "            \"{}\" => Ok(Self::{}),",
            entry.name,
            entry.variant_name()
        )?;
    }
    writeln!(file, "            _ => Err(Error::NameNotFound)")?;
    writeln!(file, "        }}")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;
    writeln!(file, "impl fmt::Display for ProductCategory {{")?;
    writeln!(file, "    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {{")?;
    writeln!(file, "        let x = match self {{")?;
    for entry in entries.iter() {
        writeln!(
            file,
            "            Self::{} => \"{}\",",
            &entry.variant_name(),
            &entry.name
        )?;
    }
    writeln!(file, "        }};")?;
    writeln!(file, "        write!(f, \"{{}}\", x)")?;
    writeln!(file, "    }}")?;
    writeln!(file, "}}")?;
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let out_path = Path::new(&env::var("OUT_DIR").unwrap()).join("enum.rs");
    let entries = parse();
    let mut file = BufWriter::new(File::create(&out_path).expect("Couldn't write to output file"));
    write_enum(&mut file, &entries)?;
    Ok(())
}
