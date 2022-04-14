"""
Disclaimer:
This file is not currently very clean made with very unintuitive
variable names. I did not originally plan to publish this but
have it as a temporary script.
"""

import re
import json

with open('block_list.json', 'r') as file:
    block_list = json.load(file)
    m = block_list['default_states']
    enums = block_list['enum_properties']
    property_clarity = block_list['property_clarity']

m = {k: [p.split('=') for p in v.split(',')] if v != '' else [] for k, v in m.items()}

n = '\n'
q = '"'
i = '    '
nn = n + i


def get_type(val: str, block: str) -> str:
    if val in ['true', 'false']:
        return 'bool'
    if re.compile(r'\d+').fullmatch(val):
        return 'u8'
    found = []
    for name, vals in enums.items():
        if val in vals:
            found.append(name)
    while len(found) > 1:
        for i, f in enumerate(found):
            if f in property_clarity:
                if block in property_clarity[f]:
                    return f
                del found[i]
            continue
    if len(found) == 1:
        return found[0]
    return '(), /* TODO: ' + val + ' */'


def get_properties(props: list[list[str]], block: str) -> str:
    if not props:
        return ''

    return ' { ' + ', '.join([
        f'{k if k != "type" else "r#type"}: {get_type(v, block)}'
        for k, v in props
    ]) + ' }'


list_rs = r"""use std::{borrow::Cow, collections::HashMap};

use super::types::*;
""" + f"""
#[derive(Debug, Clone)]
pub enum BlockState<'a> {{
    {nn.join([
        f'{k.title().replace("_", "")}{get_properties(v, k)},'
        for k, v in m.items()
    ])}
    Other {{ name: Cow<'a, str>, properties: Option<HashMap<Cow<'a, str>, Cow<'a, str>>> }},
}}
"""
ser_rs = r"""use std::str::FromStr;

use crate::schema;
use super::{types::*, list::BlockState};

macro_rules! try_make {
    ($block:ident, $state:ident; $($name:ident => $type:ty),+) => {
        match $state.properties.as_ref() {
            Some(props) => Self::$block {
                $(
                    $name: match props.get(stringify!($name)) {
                        Some(val) => match <$type>::from_str(val).ok() {
                            Some(val) => val,
                            None => return Self::Other { name: $state.name.to_owned(), properties: $state.properties.to_owned() },
                        },
                        None => return Self::Other { name: $state.name.to_owned(), properties: $state.properties.to_owned() },
                    }
                ),+
            },
            None => Self::Other { name: $state.name.to_owned(), properties: $state.properties.to_owned() },
        }
    };
}
""" + f"""
impl <'a> From<&schema::BlockState<'a>> for BlockState<'a> {{
    fn from(state: &schema::BlockState<'a>) -> Self {{
        match state.name.as_ref() {{
            {(nn+i+i).join([
                f'"minecraft:{k}" => '
                + (
                    f'Self::{k.title().replace("_", "")}'
                    if v == []
                    else (
                        f'try_make!(' + k.title().replace('_', '') + ', state; '
                        + ', '.join([
                            f'{pk if pk != "type" else "r#type"} => {get_type(pv, k)}'
                            for pk, pv in v
                        ]) + ')'
                    )
                ) + ','
                for k, v in m.items()
            ])}
            _ => Self::Other {{ name: state.name.to_owned(), properties: state.properties.to_owned() }},
        }}
    }}
}}
"""
de_rs = r"""use std::{borrow::Cow, collections::HashMap};

use crate::schema;
use super::list::BlockState;
""" + f"""
impl <'a> From<&BlockState<'a>> for schema::BlockState<'a> {{
    fn from(state: &BlockState<'a>) -> Self {{
        match state {{
            {(nn+i+i).join([
                'BlockState::' + k.title().replace('_', '')
                + (
                    '' if v == []
                    else ' { ' + ', '.join([
                        pk if pk != 'type' else 'r#type' for pk, _ in v
                    ]) + ' }'
                ) + ' => Self { ' + f'name: Cow::Borrowed("minecraft:{k}"), properties: '
                + (
                    'None' if v == []
                    else (
                        'Some(HashMap::from(['
                        + ', '.join([
                            f'(Cow::Borrowed("{pk}"), Cow::Owned({pk if pk != "type" else "r#type"}.to_string()))'
                            for pk, _ in v
                        ]) + '])),'
                    )
                ) + ' },'
                for k, v in m.items()
            ])}
            BlockState::Other {{ name, properties }} => Self {{ name: name.to_owned(), properties: properties.to_owned() }},
        }}
    }}
}}
"""
types_rs = n.join([
    r'#[derive(Debug, strum::Display, strum::EnumString, Clone, PartialEq, Eq)]' + n
    + r'#[strum(serialize_all = "snake_case")]' + n
    + 'pub enum ' + k + ' {'
    + (
        ' '+', '.join([n.title().replace('_', '') for n in v])+' '
        if len(v) <= 8
        else n+i+nn.join([n.title().replace('_', '')+',' for n in v])+n
    ) + '}'
    for k, v in enums.items()
]) + '\n'

with open('src/block_state/list.rs', 'w') as file:
    file.write(list_rs)
with open('src/block_state/ser.rs', 'w') as file:
    file.write(ser_rs)
with open('src/block_state/de.rs', 'w') as file:
    file.write(de_rs)
with open('src/block_state/types.rs', 'w') as file:
    file.write(types_rs)
