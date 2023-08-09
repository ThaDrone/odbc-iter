extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse_macro_input, Ident, Type};
use syn::{Data, DeriveInput, Fields};

#[proc_macro_derive(SqlStruct)]
/// Implements TryFromRow for the struct.
/// Will match fields in order they are written and is case sensitive
/// All fields types must implement ['TryFromColumn'].
pub fn sql_struct_derive(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    // Grab all fields from the struct
    let fields = match input.data {
        Data::Struct(s) => s.fields,
        _ => panic!("Expected a struct"),
    };

    // Grab the field names
    let field_names = match fields.clone() {
        Fields::Named(fields) => fields
            .named
            .into_iter()
            .map(|f| f.ident.expect("No Name"))
            .collect::<Vec<Ident>>(),
        _ => panic!("Expected named fields!"),
    };

    // Grab the field types
    let field_types = match fields {
        Fields::Named(fields) => fields
            .named
            .into_iter()
            .map(|f| f.ty)
            .collect::<Vec<Type>>(),
        _ => panic!("Expected named fields with types!"),
    };

    let expanded = quote! {


        impl odbc_iter::TryFromRow<odbc_iter::DefaultConfiguration> for #name {

            // TODO change error handling. Currently the code will crash if there is an error.
            type Error = odbc_iter::OdbcError;

            fn try_from_row<'r, 's, 'c, S>(
                mut row: odbc_iter::Row<'r, 's, 'c, S, odbc_iter::DefaultConfiguration>,
            ) -> Result<Self, Self::Error> {

                use odbc_iter::TryFromColumn;

                #(
                    let column = row.shift_column().expect("Query error! No more columns available!");
                    let column_name = &column.column_type.name;

                    let field = stringify!(#field_names);

                    // Check if the field name and the column name match. Ideally this is done
                    // during compilation.
                     assert_eq!(field, column_name,
                        "Field names do not match! Expected {}, found {} instead.", field, column_name);

                    // Grab the value from the result
                    let #field_names = <#field_types>::try_from_column(column).expect(&format!("Conversion error for field {field}"));
                )*

                Ok(Self {
                        #(#field_names,)*
                        })
            }

    }

        };

    TokenStream::from(expanded)
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        todo!()
    }
}
