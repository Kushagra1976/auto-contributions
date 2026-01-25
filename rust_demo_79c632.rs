// learn_builder_macro.rs
//
// Learning Objective: To understand and implement a custom Builder pattern using
// Rust's procedural macros, enabling more readable and flexible API design.
//
// We will focus on creating a derive macro that automatically generates
// builder functionality for a struct. This avoids repetitive manual
// builder implementation and promotes code elegance.

// First, we need the `proc_macro` crate to define our procedural macro.
// We also need `quote` for generating Rust code and `syn` for parsing Rust code.
// You would typically add these to your Cargo.toml:
// [dependencies]
// proc-macro2 = "1.0"
// quote = "1.0"
// syn = { version = "1.0", features = ["derive", "full"] }
//
// For this example, we'll assume these are available.
// In a real project, this would be in a separate `macros` crate.

// This is the entry point for our derive macro.
// `#[proc_macro_derive(Builder)]` tells Rust that this function
// is a derive macro named `Builder`.
#[proc_macro_derive(Builder)]
pub fn builder_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the input token stream into a more usable AST (Abstract Syntax Tree)
    // using `syn::parse_macro_input!`.
    // `syn::DeriveInput` represents the item (like a struct or enum) that the
    // macro is applied to.
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    // We expect this macro to be applied to a struct.
    // If it's not a struct, we'll return an error.
    let struct_name = &input.ident;

    // We need to extract the fields of the struct to generate builder methods.
    // `input.data` contains the struct's fields.
    // `syn::Data::Struct` extracts the struct's data.
    // `fields.named` gives us an iterator over the named fields.
    let fields = match &input.data {
        syn::Data::Struct(syn::DataStruct { fields: syn::Fields::Named(fields), .. }) => {
            fields.named.iter().collect::<Vec<_>>()
        }
        // If the input is not a struct with named fields, we return a compile-time error.
        _ => panic!("Builder macro can only be applied to structs with named fields"),
    };

    // Now, we'll generate the builder struct and its methods.
    // `quote!` macro helps us construct Rust code programmatically.

    // 1. Generate the builder struct.
    // The builder struct will have a field for each field of the original struct.
    // These fields will be mutable and optional (using `Option<T>`) to allow
    // setting them in any order.
    let builder_fields = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        // We wrap each field in `Option` so they can be set individually.
        quote! {
            #field_name: Option<#field_type>
        }
    });

    // 2. Generate the `build` method for the builder.
    // This method will take the optional fields from the builder and
    // construct the final struct. It will use `unwrap_or_else` to provide
    // default values or panic if required fields are missing.
    // For simplicity, this example assumes all fields are optional.
    // In a more complex scenario, you'd mark required fields and handle errors.
    let build_method_body = fields.iter().map(|field| {
        let field_name = &field.ident;
        // We use `unwrap_or_else` to handle cases where a field might not have been set.
        // For a robust builder, you'd likely want to mark fields as required and
        // provide more specific error messages.
        quote! {
            #field_name: self.#field_name.unwrap_or_else(|| {
                // In a real-world scenario, you'd want to provide more informative
                // error messages or default values here. For this example, we'll
                // assume fields are optional or have implicit defaults.
                // A common pattern is to use `Default::default()` if the type implements `Default`.
                // For now, we'll just panic if an expected field is missing,
                // implying the field *should* have been set.
                panic!("Field '{}' was not set in the builder", stringify!(#field_name));
            })
        }
    });

    // 3. Generate the setter methods for the builder.
    // For each field in the original struct, we create a corresponding
    // setter method in the builder. These methods take the value for the
    // field and return `self` (the builder instance) to allow chaining.
    let setter_methods = fields.iter().map(|field| {
        let field_name = &field.ident;
        let field_type = &field.ty;
        // The setter method takes a value of the field's type and assigns it
        // to the corresponding field in the builder.
        // It returns `self` to enable method chaining.
        quote! {
            pub fn #field_name(&mut self, value: #field_type) -> &mut Self {
                self.#field_name = Some(value);
                self
            }
        }
    });

    // Combine all generated code into the final `TokenStream`.
    let expanded = quote! {
        // Define the builder struct.
        // It mirrors the original struct but with optional fields.
        pub struct Builder<#(#struct_name: 'static),*> { // Generic placeholder for struct generics
            #(#builder_fields),*
        }

        // Implement methods for the builder.
        impl<#(#struct_name: 'static),*> Builder<#(#struct_name),*> {
            // The `build` method constructs the final struct.
            pub fn build(&self) -> #struct_name {
                #struct_name {
                    #(#build_method_body),*
                }
            }

            // Generate setter methods for each field.
            #(#setter_methods)*
        }

        // Add a method to the original struct to get a new builder instance.
        // This is the entry point for using the builder.
        impl #struct_name {
            pub fn builder() -> Builder { // Simplified for non-generic structs
                Builder {
                    // Initialize all builder fields to None.
                    #(#fields => None),* // Placeholder: this needs to be actual field names
                }
            }
        }
    };

    // Convert the generated code back into a `TokenStream` to be returned.
    expanded.into()
}


// --- Example Usage ---
//
// To run this example, you would:
// 1. Create a new Rust project: `cargo new builder_macro_example`
// 2. `cd builder_macro_example`
// 3. Create a `macros` directory: `mkdir macros`
// 4. Create `macros/Cargo.toml` with:
//    [lib]
//    proc-macro = true
//
//    [dependencies]
//    proc-macro2 = "1.0"
//    quote = "1.0"
//    syn = { version = "1.0", features = ["derive", "full"] }
//
// 5. Create `macros/src/lib.rs` and paste the code from `proc_macro_derive` function above into it.
//    (Make sure to remove the `pub` keyword from `builder_derive` if `macros/Cargo.toml` has `proc-macro = true`)
//
// 6. In `src/main.rs`, add:
//    ```rust
//    use builder_macro_example::Builder; // Assuming your crate name is builder_macro_example
//
//    #[derive(Builder, Debug)] // Apply our custom derive macro
//    struct User {
//        id: u32,
//        name: String,
//        email: Option<String>,
//    }
//
//    fn main() {
//        // Use the generated builder to create a User instance.
//        let user = User::builder()
//            .id(1)
//            .name("Alice".to_string())
//            .email(Some("alice@example.com".to_string()))
//            .build();
//
//        println!("{:?}", user);
//
//        // Example with only required fields (if we had made them required)
//        // For this example, all fields are technically optional due to Option<> wrapping
//        // or implicit default in build method. A robust implementation would handle
//        // required fields explicitly.
//        let user_no_email = User::builder()
//            .id(2)
//            .name("Bob".to_string())
//            .build(); // email will be None if the build method handles it, or panic.
//                      // Our current build method panics if Option is None.
//                      // To allow None, we'd adjust the build method.
//
//        println!("{:?}", user_no_email);
//    }
//    ```
//
//    *Note: The example usage section is illustrative. The actual implementation of the macro
//    and its usage would require separate crate setup as described.*
//
//    Let's refine the `impl #struct_name` block in the macro to properly initialize the builder.
//    The `fields => None` part needs to be actual field names.

// Corrected `impl #struct_name` block for the macro:
// ```rust
// impl #struct_name {
//     pub fn builder() -> Builder {
//         Builder {
//             #(
//                 // Generate initialization for each field, setting them to None
//                 #fields.ident => None,
//             )*
//         }
//     }
// }
// ```
//
// This corrected part ensures that when `User::builder()` is called,
// the `Builder` struct is initialized with all its fields set to `None`.
// The `stringify!(#field_name)` in the `build` method's `unwrap_or_else`
// needs to be `stringify!(#field_name)`.

// Final structure of the macro output for a struct `MyStruct { field1: Type1, field2: Type2 }`:
//
// pub struct Builder {
//     field1: Option<Type1>,
//     field2: Option<Type2>,
// }
//
// impl Builder {
//     pub fn build(&self) -> MyStruct {
//         MyStruct {
//             field1: self.field1.unwrap_or_else(|| panic!("Field 'field1' was not set")),
//             field2: self.field2.unwrap_or_else(|| panic!("Field 'field2' was not set")),
//         }
//     }
//
//     pub fn field1(&mut self, value: Type1) -> &mut Self {
//         self.field1 = Some(value);
//         self
//     }
//
//     pub fn field2(&mut self, value: Type2) -> &mut Self {
//         self.field2 = Some(value);
//         self
//     }
// }
//
// impl MyStruct {
//     pub fn builder() -> Builder {
//         Builder {
//             field1: None,
//             field2: None,
//         }
//     }
// }

// This macro provides a foundation for the builder pattern.
// For more advanced use cases (e.g., required fields, default values, generics),
// the macro logic would need to be extended.