# Google Taxonomy / Product Categories

The purpose of this crate is to more easily work with [Google Product Categories / Taxonomy](https://support.google.com/merchants/answer/6324436).
This is provided via the `google_taxonomy::ProductCategory` struct which contains all categories that exist as of 2021-08-13.

## Associated constants naming

The ProductCategory contains each product category as an associated constant. They are translated from the [taxonomy-with-ids.en-US.txt](https://www.google.com/basepages/producttype/taxonomy-with-ids.en-US.txt) file as follows:
1. The leading ID is removed (and can be obtained using the `id` method).
2. The immediately following characters ` - ` are removed
3. All occurrences of `&` are replaced with `And`
4. Finally, all non-alphanumeric ascii characters are removed

For example `1604 - Apparel & Accessories > Clothing` becomes `ProductCategory::ApparelAndAccessoriesClothing`.

Each constant takes up 2 bytes of memory.

## Examples

### Try to parse an integer as a product category (i.e. from its ID)

```rust
use std::convert::TryInto;
use google_taxonomy::ProductCategory;

let cat: ProductCategory = 3237.try_into().unwrap();
assert_eq!(cat, ProductCategory::AnimalsAndPetSuppliesLiveAnimals);
```

### Get the number representation of the product category
```rust
use google_taxonomy::ProductCategory;
assert_eq!(ProductCategory::AnimalsAndPetSuppliesLiveAnimals.id(), 3237);
```

### Get the name of a product category
```rust
use google_taxonomy::ProductCategory;
assert_eq!(ProductCategory::AnimalsAndPetSuppliesLiveAnimals.to_string(), "Animals & Pet Supplies > Live Animals");
```

### Serialize / deserialize with Serde

```rust
#[cfg(feature = "serde")]
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
