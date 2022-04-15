"""
This script uses the data extracted from Minecraft by DataExtractor.java in data.txt
to create the following Rust source files:

- `src/block_state/list.rs`
- `src/block_state/types.rs`
"""

import re


def pascal(name: str) -> str:
    return name.title().replace('_', '')


with open('data.txt', 'r') as file:
    raw_block_info = file.read()

blocks = {
    match.group(1): [
        prop.split(':') for prop in [e for e in match.group(2).split(' ') if e]
    ] if match.group(2) else []
    for match in re.compile(r'BLOCKINFO --- (\w+) - (.*)').finditer(raw_block_info)
}
enums = {
    match.group(1): match.group(2).split(',')
    for match in re.compile(r'ENUMINFO --- (\w+) - (.*)').finditer(raw_block_info)
}

n = '\n'
indent = '    '


list_rs = r"""use core::marker::PhantomData;
use std::{borrow::Cow, collections::HashMap, str::FromStr};
use serde::{Deserialize, de::Visitor};
use super::types::*;
""" + f"""
blocks! {{
    {(n+indent).join([
        f'"minecraft:{name}", {pascal(name)}'
        + (
            ''
            if not props
            else (
                ', ' + ' '.join([
                    f'{prop if prop != "type" else "r#type"} => {_type}'
                    for prop, _type in props
                ])
            )
        ) + ';'
        for name, props in blocks.items()
    ])}
}}
"""
types_rs = n.join([
    r'#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]' + n
    + r'#[strum(serialize_all = "snake_case")]' + n
    + 'pub enum ' + name + ' {'
    + (
        ' ' + ', '.join([pascal(variant) for variant in variants]) + ' '
        if len(variants) <= 8
        else n+indent + (n+indent).join([pascal(variant)+',' for variant in variants]) + n
    ) + '}'
    for name, variants in enums.items()
]) + '\n'

with open('src/block_state/list.rs', 'w') as file:
    file.write(list_rs)
with open('src/block_state/types.rs', 'w') as file:
    file.write(types_rs)
