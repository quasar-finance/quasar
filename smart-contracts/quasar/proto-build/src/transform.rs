use heck::ToUpperCamelCase;
use log::debug;
use prost_types::FileDescriptorSet;

use regex::Regex;
use std::ffi::OsStr;
use std::path::Path;
use std::{fs, io};
use syn::{File, Item, ItemMod};
use walkdir::WalkDir;

use crate::transformers;

/// Protos belonging to these Protobuf packages will be excluded
/// (i.e. because they are sourced from `tendermint-proto`)
const EXCLUDED_PROTO_PACKAGES: &[&str] = &["cosmos_proto", "gogoproto", "google"];

pub fn transform_all(from_dir: &Path, descriptor: &FileDescriptorSet) {
    let mut filenames = Vec::new();
    let errors = WalkDir::new(from_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| {
            let filename = e.file_name().to_os_string().to_str().unwrap().to_string();
            filenames.push(filename.clone());
            transform(e.path(), descriptor)
        })
        .filter_map(|e| e.err())
        .collect::<Vec<_>>();

    if !errors.is_empty() {
        for e in errors {
            eprintln!("[error] Error while transforming compiled file: {}", e);
        }

        panic!("[error] Aborted.");
    }
}

fn transform(src: &Path, descriptor: &FileDescriptorSet) -> io::Result<()> {
    // Skip proto files belonging to `EXCLUDED_PROTO_PACKAGES`
    for package in EXCLUDED_PROTO_PACKAGES {
        if let Some(filename) = src.file_name().and_then(OsStr::to_str) {
            if filename.starts_with(&format!("{}.", package)) {
                return Ok(());
            }
        }
    }

    let mut contents = match fs::read_to_string(src) {
        Ok(c) => c,
        Err(e) => {
            debug!("{:?} – {}, transform skipped", src, e);
            return Ok(());
        }
    };

    for &(regex, replacement) in transformers::REPLACEMENTS {
        contents = Regex::new(regex)
            .unwrap_or_else(|_| panic!("invalid regex: {}", regex))
            .replace_all(&contents, replacement)
            .to_string();
    }

    let file = syn::parse_file(&contents);
    if let Ok(file) = file {
        // only transform rust file (skipping `*_COMMIT` file)
        let items = transform_module(file.items, src, &[], descriptor, false);
        contents = prettyplease::unparse(&File { items, ..file });
    }

    fs::write(src, &*contents)
}

fn transform_module(
    items: Vec<Item>,
    src: &Path,
    ancestors: &[String],
    descriptor: &FileDescriptorSet,
    nested_mod: bool,
) -> Vec<Item> {
    let items = transform_items(items, src, ancestors, descriptor);
    let items = prepend(items);

    append(items, src, descriptor, nested_mod)
}

fn prepend(items: Vec<Item>) -> Vec<Item> {
    items
}

fn append(
    items: Vec<Item>,
    src: &Path,
    descriptor: &FileDescriptorSet,
    nested_mod: bool,
) -> Vec<Item> {
    transformers::append_querier(items, src, nested_mod, descriptor)
}

fn transform_items(
    items: Vec<Item>,
    src: &Path,
    ancestors: &[String],
    descriptor: &FileDescriptorSet,
) -> Vec<Item> {
    items
        .into_iter()
        .map(|i| match i {
            Item::Struct(s) => Item::Struct(transformers::append_attrs_struct(src, &s, descriptor)),

            Item::Enum(e) => Item::Enum({
                let e = transformers::add_derive_eq_enum(&e);
                transformers::append_attrs_enum(src, &e, descriptor)
            }),

            // This is a temporary hack to fix the issue with clashing stake authorization validators
            Item::Mod(m) => Item::Mod(transformers::fix_clashing_stake_authorization_validators(m)),

            i => i,
        })
        .map(|i: Item| transform_nested_mod(i, src, ancestors, descriptor))
        .collect::<Vec<Item>>()
}

fn transform_nested_mod(
    i: Item,
    src: &Path,
    ancestors: &[String],
    descriptor: &FileDescriptorSet,
) -> Item {
    match i.clone() {
        Item::Mod(m) => {
            let parent = &m.ident.to_string().to_upper_camel_case();
            let content = m.content.map(|(brace, items)| {
                (
                    brace,
                    transform_module(
                        items,
                        src,
                        &[ancestors, &[parent.to_string()]].concat(),
                        descriptor,
                        true,
                    ),
                )
            });

            Item::Mod(ItemMod { content, ..m })
        }
        _ => i,
    }
}
