
mod models;
mod package_reader;
mod package;
mod file_info;
mod error;
mod meta;

pub use package::Package;
pub use package_reader::PackageReader;
pub use error::PackageReadError;
pub use meta::{Meta, MetaProperty, Version};



pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
