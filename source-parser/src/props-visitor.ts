import {
    ArgumentListCtx,
    BaseJavaCstVisitorWithDefaults,
    BlockStatementsCstNode,
    CstNode,
    FqnOrRefTypeCtx,
    FqnOrRefTypePartCommonCtx,
    IfStatementCtx,
    LiteralCtx,
    PrimaryCtx,
    VariableInitializerCtx,
} from 'java-parser'

enum Step {
    SearchPutCalls,
    TestIfNbtCall,
    SavePutMethod,
    GetPropKey,
}

export class FindPropsVisitor extends BaseJavaCstVisitorWithDefaults {
    props: [string, string][] = []
    #currentClass: string
    #superClasses: string[]
    #classFields: { [key: string]: [string, string, VariableInitializerCtx?][] }
    #classMethods: { [key: string]: [string, string][] }
    #nbtMethods: [string, BlockStatementsCstNode[]][]
    #steps: Step[] = [Step.SearchPutCalls]
    #nbtParam: string = ''
    #_isNbtCall: boolean = false
    #_nbtPutMethod: string = ''
    #_propKey: string = ''
    #_propType: string = ''
    #_isOptional: boolean = false

    constructor(
        currentClass: string,
        superClasses: string[],
        classFields: { [key: string]: [string, string, VariableInitializerCtx?][] },
        classMethods: { [key: string]: [string, string][] },
        nbtMethods: [string, BlockStatementsCstNode[]][],
    ) {
        super()
        this.#currentClass = currentClass
        this.#superClasses = superClasses
        this.#classFields = classFields
        this.#classMethods = classMethods
        this.#nbtMethods = nbtMethods
        this.validateVisitor()
    }

    #getStep(): Step {
        return this.#steps.slice(-1)[0]
    }
    #pushStep(step: Step) {
        this.#steps.push(step)
    }
    #popStep(step?: Step) {
        if (step !== undefined && this.#getStep() !== step) {
            console.warn(
                `\x1b[1;33mWarning: Expected to pop step ${Step[step]}, but current step is ${
                    Step[this.#getStep()]
                }\x1b[0m`,
            )
        }
        this.#steps.pop()
    }

    visitAll(nodes: CstNode[] | undefined) {
        if (nodes === undefined) return
        for (const node of nodes) {
            this.visit(node)
        }
    }

    run() {
        if (this.#nbtMethods === undefined) {
            console.warn('\x1b[1;33mWarning: No NBT methods found for class\x1b[0m')
            return
        }
        if (this.#nbtMethods.length === 0) console.log('\x1b[1;90m UNIMPORTANT \x1b[0m')
        for (const nbtMethod of this.#nbtMethods) {
            this.#nbtParam = nbtMethod[0]
            // TODO: maybe test for super call
            this.visitAll(nbtMethod[1])
        }
    }

    /* --------------------------- */

    ifStatement(ctx: IfStatementCtx) {
        if (this.#getStep() === Step.SearchPutCalls) {
            if (ctx.statement.length === 0) return
            this.#_isOptional = true
            this.visit(ctx.statement[0])
            this.#_isOptional = false
            this.visitAll(ctx.statement.slice(1))
        } else {
            this.visitAll(ctx.statement)
        }
    }

    primary(ctx: PrimaryCtx) {
        if (this.#getStep() === Step.SearchPutCalls) {
            this.#_isNbtCall = false
            this.#pushStep(Step.TestIfNbtCall)
            this.visitAll(ctx.primaryPrefix)
            this.#popStep(Step.TestIfNbtCall)
            if (!this.#_isNbtCall) return

            this.#pushStep(Step.GetPropKey)
            this.visitAll(ctx.primarySuffix)

            if (this.#_nbtPutMethod === 'putByte') {
                this.#_propType = 'byte'
            } else if (this.#_nbtPutMethod === 'putShort') {
                this.#_propType = 'short'
            } else if (this.#_nbtPutMethod === 'putInt') {
                this.#_propType = 'int'
            } else if (this.#_nbtPutMethod === 'putLong') {
                this.#_propType = 'long'
            } else if (this.#_nbtPutMethod === 'putUuid') {
                this.#_propType = 'UUID'
            } else if (this.#_nbtPutMethod === 'putFloat') {
                this.#_propType = 'float'
            } else if (this.#_nbtPutMethod === 'putDouble') {
                this.#_propType = 'double'
            } else if (this.#_nbtPutMethod === 'putString') {
                this.#_propType = 'String'
            } else if (this.#_nbtPutMethod === 'putByteArray') {
                this.#_propType = 'NbtByteArray'
            } else if (this.#_nbtPutMethod === 'putIntArray') {
                this.#_propType = 'NbtIntArray'
            } else if (this.#_nbtPutMethod === 'putLongArray') {
                this.#_propType = 'NbtLongArray'
            } else if (this.#_nbtPutMethod === 'putBoolean') {
                this.#_propType = 'boolean'
            } else {
                this.#_propType = 'NBTElement' // TODO: try to get type from value
            }

            const idx = this.props.findIndex(p => p[0] === this.#_propKey)
            if (this.#_isOptional) {
                // Only push prop if not yet saved
                if (idx === -1) this.props.push([this.#_propKey, this.#_propType + '?'])
            } else {
                // Only push prop if not yet saved
                if (idx === -1) this.props.push([this.#_propKey, this.#_propType])
                // Else reassign the prop type in case it was marked optional before
                else if (this.props[idx][1].endsWith('?')) this.props[idx][1] = this.#_propType
            }
        } else {
            this.visitAll(ctx.primaryPrefix)
            this.visitAll(ctx.primarySuffix)
        }
    }

    fqnOrRefType(ctx: FqnOrRefTypeCtx) {
        if (this.#getStep() === Step.TestIfNbtCall) if (ctx.fqnOrRefTypePartRest === undefined) return
        this.visitAll(ctx.fqnOrRefTypePartFirst)
        this.visitAll(ctx.fqnOrRefTypePartRest)
    }

    fqnOrRefTypePartCommon(ctx: FqnOrRefTypePartCommonCtx) {
        if (this.#getStep() === Step.TestIfNbtCall) {
            if (ctx.Identifier !== undefined && ctx.Identifier[0].image === this.#nbtParam) {
                this.#_isNbtCall = true
                this.#pushStep(Step.SavePutMethod)
            }
        } else if (this.#getStep() === Step.SavePutMethod) {
            this.#popStep(Step.SavePutMethod)
            if (ctx.Identifier === undefined) return
            this.#_nbtPutMethod = ctx.Identifier[0].image
        } else if (this.#getStep() === Step.GetPropKey) {
            if (ctx.Identifier === undefined) return
            const varName = ctx.Identifier[0].image
            for (const clazz of [this.#currentClass, ...this.#superClasses]) {
                const field = this.#classFields[clazz].find(f => f[0] === varName && f[1] === 'String')
                if (field === undefined) continue
                if (field[2] === undefined) continue
                this.visitAll(field[2].expression)
                if (this.#getStep() !== Step.GetPropKey) break
            }
        }
    }

    argumentList(ctx: ArgumentListCtx) {
        if (this.#getStep() === Step.GetPropKey) {
            this.visit(ctx.expression[0])
        }
    }

    literal(ctx: LiteralCtx) {
        if (this.#getStep() !== Step.GetPropKey) return
        if (ctx.StringLiteral !== undefined) {
            this.#_propKey = ctx.StringLiteral[0].image.slice(1, -1)
            this.#popStep(Step.GetPropKey)
        }
    }
}
