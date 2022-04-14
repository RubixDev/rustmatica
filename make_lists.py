"""
This script uses the data extracted from Minecraft by DataExtractor.java in data.txt
to create the following Rust source files:

- `src/block_state/list.rs`
- `src/block_state/ser.rs`
- `src/block_state/de.rs`
- `src/block_state/types.rs`
"""

import re


def pascal(name: str) -> str:
    return name.title().replace('_', '')


def chunked(data: dict, size: int):
    items = list(data.items())
    for i in range(0, len(data), size):
        yield dict(items[i:i + size])


with open('data.txt', 'r') as file:
    raw_block_info = file.read()

blocks = {
    match.group(1): [
        prop.split(':') for prop in [e for e in match.group(2).split(' ') if e]
    ] if match.group(2) else []
    for match in re.compile(r'BLOCKINFO --- (\w+) - (.*)').finditer(raw_block_info)
}
blocks_chunked = [d for d in chunked(blocks, 200)]
enums = {
    match.group(1): match.group(2).split(',')
    for match in re.compile(r'ENUMINFO --- (\w+) - (.*)').finditer(raw_block_info)
}

n = '\n'
indent = '    '


list_rs = r"""use std::{borrow::Cow, collections::HashMap};

use super::types::*;
""" + f"""
#[derive(Debug, Clone)]
pub enum BlockState<'a> {{
    {(n+indent).join([
        pascal(name)
        + (
            ''
            if not props
            else (
                ' { '
                + ', '.join([
                    f'{prop if prop != "type" else "r#type"}: {_type}'
                    for prop, _type in props
                ])
                + ' }'
            )
        ) + ','
        for name, props in blocks.items()
    ])}
    Other {{ name: Cow<'a, str>, properties: Option<HashMap<Cow<'a, str>, Cow<'a, str>>> }},
}}
"""
ser_rs = r"""use std::str::FromStr;

use crate::schema;
use super::list::BlockState;

macro_rules! try_make {
    ($block:ident, $state:ident; $($name:ident),+) => {
        match $state.properties.as_ref() {
            Some(props) => _Self::$block {
                $(
                    $name: match props.get(stringify!($name)) {
                        Some(val) => match <_>::from_str(val).ok() {
                            Some(val) => val,
                            None => return _Self::Other { name: $state.name.to_owned(), properties: $state.properties.to_owned() },
                        },
                        None => return _Self::Other { name: $state.name.to_owned(), properties: $state.properties.to_owned() },
                    }
                ),+
            },
            None => _Self::Other { name: $state.name.to_owned(), properties: $state.properties.to_owned() },
        }
    };
}

type _Self<'a> = BlockState<'a>;
""" + f"""
{n.join([
    f'fn from_chunk_{index}'"<'a>(state: &schema::BlockState<'a>) -> _Self<'a> {"+n
    + indent+'match state.name.as_ref() {'+n
    + 2*indent + (n+2*indent).join([
        f'"minecraft:{name}" => '
        + (
            f'_Self::{pascal(name)}'
            if not props
            else (
                    f'try_make!({pascal(name)}, state; '
                    + ', '.join([
                f'{prop if prop != "type" else "r#type"}'
                for prop, _type in props
            ]) + ')'
            )
        ) + ','
        for name, props in chunk.items()
    ]) + n
    + 2*indent+'_ => ' + (
        f'from_chunk_{index + 1}(state)'
        if index != len(blocks_chunked) - 1
        else '_Self::Other { name: state.name.to_owned(), properties: state.properties.to_owned() }'
    ) + ','+n+indent+'}'+n+'}'
    for index, chunk in enumerate(blocks_chunked)
])}

impl <'a> From<&schema::BlockState<'a>> for BlockState<'a> {{
    fn from(state: &schema::BlockState<'a>) -> Self {{
        from_chunk_0(state)
    }}
}}
"""
de_rs = r"""use std::{borrow::Cow, collections::HashMap};

use crate::schema;
use super::list::BlockState;

type _Self<'a> = schema::BlockState<'a>;
""" + f"""
{n.join([
    f'fn from_chunk_{index}'"<'a>(state: &BlockState<'a>) -> _Self<'a> {"+n
    + indent+'match state {'+n
    + 2*indent + (n+2*indent).join([
        f'BlockState::{pascal(name)}'
        + (
            '' if not props
            else ' { ' + ', '.join([
                prop if prop != 'type' else 'r#type' for prop, _ in props
            ]) + ' }'
        ) + ' => _Self { name: Cow::Borrowed("minecraft:' + name + '"), properties: '
        + (
            'None' if not props
            else (
                    'Some(HashMap::from(['
                    + ', '.join([
                f'(Cow::Borrowed("{prop}"), Cow::Owned({prop if prop != "type" else "r#type"}.to_string()))'
                for prop, _ in props
            ]) + '])),'
            )
        ) + ' },'
        for name, props in chunk.items()
    ]) + n
    + 2*indent + (
        f'_ => from_chunk_{index + 1}(state),'
        if index != len(blocks_chunked) - 1
        else ('BlockState::Other { name, properties } => '
            + '_Self { name: name.to_owned(), properties: properties.to_owned() },'+n+2*indent
            + '_ => unreachable!(),')
    ) + n+indent + '}' + n
    + '}'
    for index, chunk in enumerate(blocks_chunked)
])}

impl <'a> From<&BlockState<'a>> for schema::BlockState<'a> {{
    fn from(state: &BlockState<'a>) -> Self {{
        from_chunk_0(state)
    }}
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
with open('src/block_state/ser.rs', 'w') as file:
    file.write(ser_rs)
with open('src/block_state/de.rs', 'w') as file:
    file.write(de_rs)
with open('src/block_state/types.rs', 'w') as file:
    file.write(types_rs)
