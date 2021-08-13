# Google Taxonomy / Product Categories

The purpose of this crate is to more easily work with [Google Product Categories / Taxonomy](https://support.google.com/merchants/answer/6324436).
This is provided via the `google_taxonomy::ProductCategory` enum which contains all categories that exist as of 2021-08-13.

## Examples

### From

```rust
use std::convert::TryInto;
use google_taxonomy::ProductCategory;

// Try to parse an integer as a product category (i.e. from its ID)
let cat: ProductCategory = 3237.try_into().unwrap();
assert_eq!(cat, ProductCategory::AnimalsAndPetSuppliesLiveAnimals);
```

### Get the number representation of the product category
```rust
use google_taxonomy::ProductCategory;
assert_eq!(ProductCategory::AnimalsAndPetSuppliesLiveAnimals as u32, 3237);
```

### Get the name of a product category
```rust
use google_taxonomy::ProductCategory;
assert_eq!(ProductCategory::AnimalsAndPetSuppliesLiveAnimals.to_string(), "Animals & Pet Supplies > Live Animals");
```

### Serialize / deserialize with Serde

```rust
#[cfg(feature = "with-serde")]
{
    use serde::{Deserialize, Serialize};
    use google_taxonomy::ProductCategory;

    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    struct Product {
        category: ProductCategory,
    }
    let serialized = r#"{"category":"Animals & Pet Supplies"}"#;

    // Deserialize, e.g. with serde_json
    let deserialized: Product = serde_json::from_str(&serialized).unwrap();

    assert_eq!(deserialized, Product { category: ProductCategory::AnimalsAndPetSupplies });

    // And back to its original serialized form again...
    assert_eq!(serde_json::to_string(&deserialized).unwrap(), serialized);
}
```
