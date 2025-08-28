use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitStr, Path, Type, parse_macro_input};

#[proc_macro_attribute]
pub fn profile(args: TokenStream, input: TokenStream) -> TokenStream {
    let profile_path = parse_macro_input!(args as LitStr);
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;
    let profile_str = profile_path.value();

    // Extract field information
    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => &fields_named.named,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    // Separate required and optional fields
    let mut required_fields = Vec::new();
    let mut optional_fields = Vec::new();

    for (i, field) in fields.iter().enumerate() {
        let field_name = &field.ident;
        let field_type = &field.ty;

        if is_option_type(field_type) {
            optional_fields.push((i, field_name, field_type));
        } else {
            required_fields.push((i, field_name, field_type));
        }
    }

    let required_count = required_fields.len();

    // Generate parsing logic for required fields
    let required_parsers: Vec<_> = required_fields.iter().map(|(i, field_name, field_type)| {
        quote! {
            #field_name: {
                let arg = args.get(#i)
                    .ok_or_else(|| format!("Missing required argument {} for field {}", #i, stringify!(#field_name)))?;
                match arg {
                    rosc::OscType::Int(val) => *val as #field_type,
                    rosc::OscType::Float(val) => *val as #field_type,
                    _ => return Err(format!("Invalid type for required field {}", stringify!(#field_name))),
                }
            }
        }
    }).collect();

    // Generate parsing logic for optional fields
    let optional_parsers: Vec<_> = optional_fields.iter().map(|(i, field_name, _field_type)| {
        quote! {
            #field_name: if args.len() > #i {
                match args.get(#i) {
                    Some(rosc::OscType::Int(val)) => Some(*val as f32),
                    Some(rosc::OscType::Float(val)) => Some(*val),
                    Some(_) => return Err(format!("Invalid type for optional field {}", stringify!(#field_name))),
                    None => None,
                }
            } else {
                None
            }
        }
    }).collect();

    // Generate OSC type conversions for serialization
    let field_to_osc: Vec<_> = fields
        .iter()
        .map(|field| {
            let field_name = &field.ident;
            let field_type = &field.ty;

            if is_option_type(field_type) {
                quote! {
                    if let Some(val) = self.#field_name {
                        Some(rosc::OscType::Float(val))
                    } else {
                        None
                    }
                }
            } else {
                let type_str = quote!(#field_type).to_string();
                if type_str.contains("i32") {
                    quote! { Some(rosc::OscType::Int(self.#field_name)) }
                } else if type_str.contains("f32") {
                    quote! { Some(rosc::OscType::Float(self.#field_name)) }
                } else {
                    quote! { Some(rosc::OscType::Float(self.#field_name as f32)) }
                }
            }
        })
        .collect();

    let expanded = quote! {
        #input

        impl #struct_name {
            pub fn from_osc_message(msg: &rosc::OscMessage) -> Result<Self, String> {
                if msg.addr != #profile_str {
                    return Err(format!("Address mismatch: expected {}, got {}", #profile_str, msg.addr));
                }

                let args = &msg.args;

                // Check minimum required arguments
                if args.len() < #required_count {
                    return Err(format!("Not enough arguments: expected at least {}, got {}", #required_count, args.len()));
                }

                Ok(Self {
                    #(#required_parsers,)*
                    #(#optional_parsers,)*
                })
            }

            pub fn to_osc_message(&self) -> rosc::OscMessage {
                let mut args = Vec::new();

                // Collect all arguments (required and optional)
                let all_args: Vec<Option<rosc::OscType>> = vec![
                    #(#field_to_osc,)*
                ];

                // Add arguments up to the last Some value
                let mut last_some_index = 0;
                for (i, arg) in all_args.iter().enumerate() {
                    if arg.is_some() {
                        last_some_index = i;
                    }
                }

                for (i, arg) in all_args.iter().enumerate() {
                    if i <= last_some_index {
                        if let Some(osc_arg) = arg {
                            args.push(osc_arg.clone());
                        } else {
                            // For None values in the middle, we might want to add a default
                            // This depends on TUIO protocol requirements
                            args.push(rosc::OscType::Float(0.0));
                        }
                    }
                }

                rosc::OscMessage {
                    addr: #profile_str.to_string(),
                    args,
                }
            }

            pub fn addr() -> String {
                #profile_str.to_string()
            }

            pub fn profile() -> &'static str {
                #profile_str
            }
        }
    };

    TokenStream::from(expanded)
}

// Helper function to check if a type is Option<T>
fn is_option_type(ty: &Type) -> bool {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            return segment.ident == "Option";
        }
    }
    false
}
