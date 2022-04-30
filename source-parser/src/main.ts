import {
    BlockStatementsCstNode,
    CstNode,
    parse,
    VariableInitializerCtx,
} from 'java-parser'
import stringify from 'json-stringify-pretty-compact'
import fs from 'fs'
import { SaveMembersVisitor } from './member-visitor'
import { FindPropsVisitor } from './props-visitor'

const data = fs.readFileSync('../data.txt') + ''
let entities: { [key: string]: string } = {}
for (const m of data.matchAll(/ENTITYINFO --- minecraft:(\w+) - (.*)/g)) {
    entities[m[1]] = m[2]
}
let entityClasses: { [key: string]: string[] } = {}
for (const m of data.matchAll(/ENTITYCLASSINFO --- (.*?) - (.*)/g)) {
    entityClasses[m[1]] = m[2].split(',').filter(c => c !== '')
}

let classFields: { [key: string]: [string, string, VariableInitializerCtx?][] } = {}
let classMethods: { [key: string]: [string, string][] } = {}
let classNbtMethods: { [key: string]: [string, BlockStatementsCstNode[]][] } = {}

for (const entityClass of Object.keys(entityClasses)) {
    console.log(`\x1b[90mParsing class ${entityClass}\x1b[0m`)
    let cst: CstNode
    try {
        cst = parse(fs.readFileSync('./mc_src_cfr/' + entityClass) + '')
    } catch {
        console.warn(`\x1b[1;33mWarning: Failed to parse class ${entityClass} from CFR decompilation. Trying FernFlower...\x1b[0m`)
        try {
            cst = parse(fs.readFileSync('./mc_src_ff/' + entityClass) + '')
        } catch {
            console.error(`\x1b[1;31mError: Failed to parse class ${entityClass}. Please manually provide properties\x1b[0m`)
            continue
        }
    }

    const visitor = new SaveMembersVisitor()
    visitor.visit(cst)
    classFields[entityClass] = visitor.fields
    classMethods[entityClass] = visitor.methods
    classNbtMethods[entityClass] = visitor.nbtMethods
}

let entityProps: { [key: string]: [string, string][] } = {}
for (const [entityClass, superClasses] of Object.entries(entityClasses)) {
    console.log(`\x1b[90mSearching NBT properties for ${entityClass}\x1b[0m`)

    const visitor = new FindPropsVisitor(
        entityClass,
        superClasses,
        classFields,
        classMethods,
        classNbtMethods[entityClass],
    )
    visitor.run()
    entityProps[entityClass] = visitor.props
}

let allEntityProps: { [key: string]: [string, string][] } = {}
for (const [name, clazz] of Object.entries(entities)) {
    allEntityProps[name] = entityProps[clazz]
    for (const superClass of entityClasses[clazz]) {
        allEntityProps[name] = [...allEntityProps[name], ...entityProps[superClass]]
    }
}
fs.writeFileSync('../entityData.json', stringify(allEntityProps, { maxLength: 2000 }))
