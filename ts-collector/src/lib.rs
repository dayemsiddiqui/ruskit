pub fn add(left: u64, right: u64) -> u64 {
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

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemMod};

#[proc_macro_attribute]
pub fn collect_ts_exports(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemMod);
    
    let output = quote! {
        #input

        pub fn export_all_types(output_file: &str) -> Result<(), ts_rs::ExportError> {
            use ts_rs::TS;
            Ok(())
        }
    };
    
    output.into()
}
