#![doc = include_str!("../README.md")]
use core::convert::TryFrom;
use core::fmt;

// Include the build-script generated ProductCategory enum
include!(concat!(env!("OUT_DIR"), "/enum.rs"));

#[cfg(feature = "with-serde")]
use serde::{
    de::{self, Deserializer},
    ser::Serializer,
    Deserialize, Serialize,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    /// Error for when a product category ID does not exist
    IdNotFound,
    /// Error for when a product category name does not exist
    NameNotFound,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let field = match self {
            Self::IdNotFound => "id",
            Self::NameNotFound => "name",
        };
        write!(f, "the product category {} does not exist", field)
    }
}

impl TryFrom<i8> for ProductCategory {
    type Error = Error;

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let value = u32::try_from(value).map_err(|_| Error::IdNotFound)?;
        Self::from_id(value)
    }
}

impl TryFrom<i16> for ProductCategory {
    type Error = Error;

    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let value = u32::try_from(value).map_err(|_| Error::IdNotFound)?;
        Self::from_id(value)
    }
}

impl TryFrom<i32> for ProductCategory {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let value = u32::try_from(value).map_err(|_| Error::IdNotFound)?;
        Self::from_id(value)
    }
}

impl TryFrom<i64> for ProductCategory {
    type Error = Error;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let value = u32::try_from(value).map_err(|_| Error::IdNotFound)?;
        Self::from_id(value)
    }
}

impl TryFrom<isize> for ProductCategory {
    type Error = Error;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        let value = u32::try_from(value).map_err(|_| Error::IdNotFound)?;
        Self::from_id(value)
    }
}

impl TryFrom<u8> for ProductCategory {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::try_from(value as u32)
    }
}

impl TryFrom<u16> for ProductCategory {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::try_from(value as u32)
    }
}

impl TryFrom<u32> for ProductCategory {
    type Error = Error;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_id(value)
    }
}

impl TryFrom<u64> for ProductCategory {
    type Error = Error;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let value = u32::try_from(value).map_err(|_| Error::IdNotFound)?;
        Self::from_id(value)
    }
}

impl TryFrom<usize> for ProductCategory {
    type Error = Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let value = u32::try_from(value).map_err(|_| Error::IdNotFound)?;
        Self::from_id(value)
    }
}

#[cfg(feature = "with-serde")]
impl<'de> Deserialize<'de> for ProductCategory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_name(&s).map_err(|_| de::Error::custom(Error::IdNotFound.to_string()))
    }
}

#[cfg(feature = "with-serde")]
impl Serialize for ProductCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::convert::TryInto;

    #[test]
    fn casts_to_category_id() {
        assert_eq!(
            ProductCategory::AnimalsAndPetSuppliesLiveAnimals as u32,
            3237
        );
    }

    #[test]
    fn casts_to_category_name() {
        assert_eq!(
            ProductCategory::AnimalsAndPetSuppliesLiveAnimals.to_string(),
            "Animals & Pet Supplies > Live Animals"
        );
    }

    #[test]
    fn id_to_enum() {
        let cat: ProductCategory = 3237.try_into().unwrap();
        assert_eq!(cat, ProductCategory::AnimalsAndPetSuppliesLiveAnimals);
    }

    #[test]
    fn id_u8_to_enum() {
        let cat = ProductCategory::try_from(111_u8).unwrap();
        assert_eq!(cat, ProductCategory::BusinessAndIndustrial);
    }

    #[test]
    fn id_u16_to_enum() {
        let cat = ProductCategory::try_from(3237_u16).unwrap();
        assert_eq!(cat, ProductCategory::AnimalsAndPetSuppliesLiveAnimals);
    }

    #[test]
    fn id_u32_to_enum() {
        let cat = ProductCategory::try_from(3237_u32).unwrap();
        assert_eq!(cat, ProductCategory::AnimalsAndPetSuppliesLiveAnimals);
    }

    #[test]
    fn id_u64_to_enum() {
        let cat = ProductCategory::try_from(3237_u64).unwrap();
        assert_eq!(cat, ProductCategory::AnimalsAndPetSuppliesLiveAnimals);
    }

    #[test]
    fn id_usize_to_enum() {
        let cat = ProductCategory::try_from(111_usize).unwrap();
        assert_eq!(cat, ProductCategory::BusinessAndIndustrial);
    }

    #[test]
    fn id_i8_to_enum() {
        let cat = ProductCategory::try_from(111_i8).unwrap();
        assert_eq!(cat, ProductCategory::BusinessAndIndustrial);
    }

    #[test]
    fn id_i16_to_enum() {
        let cat = ProductCategory::try_from(3237_i16).unwrap();
        assert_eq!(cat, ProductCategory::AnimalsAndPetSuppliesLiveAnimals);
    }

    #[test]
    fn id_i32_to_enum() {
        let cat = ProductCategory::try_from(3237_i32).unwrap();
        assert_eq!(cat, ProductCategory::AnimalsAndPetSuppliesLiveAnimals);
    }

    #[test]
    fn id_i64_to_enum() {
        let cat = ProductCategory::try_from(3237_i64).unwrap();
        assert_eq!(cat, ProductCategory::AnimalsAndPetSuppliesLiveAnimals);
    }

    #[test]
    fn id_isize_to_enum() {
        let cat = ProductCategory::try_from(111_isize).unwrap();
        assert_eq!(cat, ProductCategory::BusinessAndIndustrial);
    }

    #[test]
    fn positive_id_not_found_error() {
        assert_eq!(ProductCategory::try_from(u32::MAX), Err(Error::IdNotFound));
    }

    #[test]
    fn negative_id_not_found_error() {
        assert_eq!(ProductCategory::try_from(-1), Err(Error::IdNotFound));
    }

    #[test]
    #[cfg(feature = "with-serde")]
    fn test_serde_deserialize() {
        let category: ProductCategory = serde_json::from_str(&"\"Baby & Toddler\"").unwrap();

        assert_eq!(category, ProductCategory::BabyAndToddler);
    }

    #[test]
    #[cfg(feature = "with-serde")]
    fn test_serde_serialize() {
        let serialized = serde_json::to_string(&ProductCategory::ApparelAndAccessories).unwrap();
        assert_eq!(serialized, "\"Apparel & Accessories\"");
    }
}

#[cfg(doctest)]
use doc_comment::doctest;
#[cfg(doctest)]
doc_comment::doctest!("../README.md");
