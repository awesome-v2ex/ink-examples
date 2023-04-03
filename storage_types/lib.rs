#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod storage_types {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;

    /// Errors that can occur upon calling this contract.
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(::scale_info::TypeInfo))]
    pub enum Error {
        ThisIsAnErrorEnum,
    }

    pub type Result<T> = core::result::Result<T, Error>;

    #[derive(Default, scale::Decode, scale::Encode, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum EnumWithoutValues {
        #[default]
        A,
        B,
        C,
    }

    #[derive(scale::Decode, scale::Encode, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum EnumWithValues {
        OneValue(u32),
        TwoValues(u32, u32),
        TheeValues(u32, u32, u32),
    }

    #[derive(scale::Decode, scale::Encode, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(Debug, scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct PrimitiveTypes {
        bool_value: bool,
        enum_without_values: EnumWithoutValues,
        enum_with_values: EnumWithValues,
        array_value: [u32; 3],
        tuple_value: (u32, u32),
    }

    #[derive(scale::Decode, scale::Encode, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(
            Debug,
            PartialEq,
            Eq,
            scale_info::TypeInfo,
            ink::storage::traits::StorageLayout
        )
    )]
    pub struct SignedIntegers {
        i128_value_max: i128,
        i128_value_min: i128,
        i16_value_max: i16,
        i16_value_min: i16,
        i32_value_max: i32,
        i32_value_min: i32,
        i64_value_max: i64,
        i64_value_min: i64,
        i8_value_max: i8,
        i8_value_min: i8,
    }

    #[derive(scale::Decode, scale::Encode, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(
            Debug,
            PartialEq,
            Eq,
            scale_info::TypeInfo,
            ink::storage::traits::StorageLayout
        )
    )]
    pub struct SubstrateTypes {
        account_id_value: AccountId,
        balance_value_max: Balance,
        balance_value_min: Balance,
        hash_value: Hash,
    }

    #[derive(scale::Decode, scale::Encode, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(
            Debug,
            PartialEq,
            Eq,
            scale_info::TypeInfo,
            ink::storage::traits::StorageLayout
        )
    )]
    pub struct PreludeTypes {
        string_value: String,
        vec_string_value: Vec<String>,
        vec_vec_string_value: Vec<Vec<String>>,
    }

    #[derive(scale::Decode, scale::Encode, Clone)]
    #[cfg_attr(
        feature = "std",
        derive(
            Debug,
            PartialEq,
            Eq,
            scale_info::TypeInfo,
            ink::storage::traits::StorageLayout
        )
    )]
    pub struct UnsignedIntegers {
        u128_value_max: u128,
        u128_value_min: u128,
        u16_value_max: u16,
        u16_value_min: u16,
        u32_value_max: u32,
        u32_value_min: u32,
        u64_value_max: u64,
        u64_value_min: u64,
        u8_value_max: u8,
        u8_value_min: u8,
    }

    #[ink(storage)]
    pub struct StorageTypes {
        prelude_types: PreludeTypes,
        primitive_types: PrimitiveTypes,
        signed_integers: SignedIntegers,
        substrate_types: SubstrateTypes,
        unsigned_integers: UnsignedIntegers,
    }

    impl StorageTypes {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                unsigned_integers: UnsignedIntegers {
                    u128_value_max: u128::MAX,
                    u128_value_min: u128::MIN,
                    u16_value_max: u16::MAX,
                    u16_value_min: u16::MIN,
                    u32_value_max: u32::MAX,
                    u32_value_min: u32::MIN,
                    u64_value_max: u64::MAX,
                    u64_value_min: u64::MIN,
                    u8_value_max: u8::MAX,
                    u8_value_min: u8::MIN,
                },
                signed_integers: SignedIntegers {
                    i128_value_max: i128::MAX,
                    i128_value_min: i128::MIN,
                    i16_value_max: i16::MAX,
                    i16_value_min: i16::MIN,
                    i32_value_max: i32::MAX,
                    i32_value_min: i32::MIN,
                    i64_value_max: i64::MAX,
                    i64_value_min: i64::MIN,
                    i8_value_max: i8::MAX,
                    i8_value_min: i8::MIN,
                },
                prelude_types: PreludeTypes {
                    string_value: String::from("This is a string"),
                    vec_string_value: Vec::new(), // TODO add elements
                    vec_vec_string_value: Vec::new(), // TODO add nested elements
                },
                primitive_types: PrimitiveTypes {
                    bool_value: true,
                    enum_with_values: EnumWithValues::TheeValues(1, 2, 3),
                    enum_without_values: EnumWithoutValues::A,
                    array_value: [3, 2, 1],
                    tuple_value: (7, 8),
                },
                substrate_types: SubstrateTypes {
                    account_id_value: AccountId::from([0x00; 32]),
                    balance_value_max: Balance::MAX,
                    balance_value_min: Balance::MIN,
                    hash_value: Hash::from([0x00; 32]),
                },
            }
        }

        #[ink(message)]
        pub fn get_unsigned_integers(&self) -> UnsignedIntegers {
            self.unsigned_integers.clone()
        }

        #[ink(message)]
        pub fn get_signed_integers(&self) -> SignedIntegers {
            self.signed_integers.clone()
        }

        #[ink(message)]
        pub fn get_prelude_types(&self) -> PreludeTypes {
            self.prelude_types.clone()
        }

        #[ink(message)]
        pub fn get_substrate_types(&self) -> SubstrateTypes {
            self.substrate_types.clone()
        }

        #[ink(message)]
        pub fn get_primitive_types(&self) -> PrimitiveTypes {
            self.primitive_types.clone()
        }

        #[ink(message)]
        pub fn get_error(&self) -> Result<()> {
            Err(Error::ThisIsAnErrorEnum)
        }
    }

    #[cfg(test)]
    mod tests {}
}