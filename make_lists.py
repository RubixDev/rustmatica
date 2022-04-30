"""
This script uses the data extracted from Minecraft by DataExtractor.java in data.txt
to create the following Rust source files:

- `src/block_state/list.rs`
- `src/block_state/types.rs`
- `src/entity/list.rs`
"""

import re
import json


def pascal(name: str) -> str:
    return name.title().replace('_', '')


type_map = {
    'byte': 'i8',
    'short': 'i16',
    'int': 'i32',
    'long': 'i64',
    'UUID': 'u128',
    'float': 'f32',
    'double': 'f64',
    'String': 'Cow<\'a, str>',
    'NbtByteArray': 'ByteArray',
    'NbtIntArray': 'IntArray',
    'NbtLongArray': 'LongArray',
    'boolean': 'bool',
    'NBTElement': 'Value',
}

with open('data.txt', 'r') as file:
    data_file = file.read()
with open('entityData.json', 'r') as file:
    entities = json.load(file)

blocks = {
    match.group(1): [
        prop.split(':') for prop in [e for e in match.group(2).split(' ') if e]
    ] if match.group(2) else []
    for match in re.compile(r'BLOCKINFO --- (\w+) - (.*)').finditer(data_file)
}
enums = {
    match.group(1): match.group(2).split(',')
    for match in re.compile(r'ENUMINFO --- (\w+) - (.*)').finditer(data_file)
}

n = '\n'
indent = '    '


list_rs = f"""use super::types::*;

blocks! {{
    {(n+indent).join([
        f'"minecraft:{name}", {pascal(name)}'
        + (
            ''
            if not props
            else (
                ' - ' + ', '.join([
                    f'{prop if prop != "type" else "r#type"}: {_type}'
                    + (f' as "{prop}"' if prop == 'type' else '')
                    for prop, _type in props
                ])
            )
        ) + ';'
        for name, props in blocks.items()
    ])}
}}
"""
types_rs = f"""\
enums! {{
    {(n+indent).join([
        name + ' => ' + ', '.join([
            pascal(variant) for variant in variants
        ]) + ';'
        for name, variants in enums.items()
    ])}
}}
"""
# TODO: automatically detect what to import
entities_rs = f"""\
use fastnbt::{{IntArray, Value}};

entities! {{
    {(n+indent).join([
        f'"minecraft:{entity}", {pascal(entity)} - '
        + ', '.join([
            f'{prop}: ' + (
                type_map[_type[:-1]] + ' as opt'
                if _type[-1] == '?'
                else type_map[_type]
            )
            for prop, _type in props if prop not in ['id', 'UUID']
        ]) + ';'
        for entity, props in entities.items()
    ])}
}}
"""

with open('src/block_state/list.rs', 'w') as file:
    file.write(list_rs)
with open('src/block_state/types.rs', 'w') as file:
    file.write(types_rs)
with open('src/entity/list.rs', 'w') as file:
    file.write(entities_rs)
