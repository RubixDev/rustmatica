import {
    BaseJavaCstVisitorWithDefaults,
    BlockStatementsCstNode,
    ClassBodyCtx,
    ClassMemberDeclarationCtx,
    ConstantDeclarationCtx,
    CstNode,
    FieldDeclarationCtx,
    FloatingPointTypeCtx,
    FormalParameterListCtx,
    IntegralTypeCtx,
    InterfaceBodyCtx,
    InterfaceMemberDeclarationCtx,
    InterfaceMethodDeclarationCtx,
    MethodDeclarationCtx,
    MethodDeclaratorCtx,
    MethodHeaderCtx,
    ResultCtx,
    UnannClassTypeCtx,
    UnannPrimitiveTypeCtx,
    UnannPrimitiveTypeWithOptionalDimsSuffixCtx,
    UnannReferenceTypeCtx,
    VariableDeclaratorCtx,
    VariableInitializerCtx,
    VariableParaRegularParameterCtx,
} from 'java-parser'

enum Step {
    SearchClassDeclaration,
    SearchClassMembers,
    GetMemberInfo,
    GetType,
    GetMemberName,
    TestForNbtParameters,
}

export class SaveMembersVisitor extends BaseJavaCstVisitorWithDefaults {
    fields: [string, string, VariableInitializerCtx?][] = []
    methods: [string, string][] = []
    nbtMethods: [string, BlockStatementsCstNode[]][] = []
    #steps: Step[] = [Step.SearchClassDeclaration]
    #_savedType: string = ''
    #_shouldSaveMethod: boolean = false
    #_savedParameterName: string = ''

    constructor() {
        super()
        this.validateVisitor()
    }

    visitAll(nodes: CstNode[] | undefined) {
        if (nodes === undefined) return
        for (const node of nodes) {
            this.visit(node)
        }
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

    #body(node?: CstNode[]) {
        if (this.#getStep() !== Step.SearchClassDeclaration) return
        this.#popStep(Step.SearchClassDeclaration)
        this.#pushStep(Step.SearchClassMembers)
        this.visitAll(node)
        this.#popStep(Step.SearchClassMembers)
    }
    classBody(ctx: ClassBodyCtx) {
        this.#body(ctx.classBodyDeclaration)
    }
    interfaceBody(ctx: InterfaceBodyCtx) {
        this.#body(ctx.interfaceMemberDeclaration)
    }

    #memberDeclaration(fieldNode?: CstNode[], methodNode?: CstNode[]) {
        if (this.#getStep() !== Step.SearchClassMembers) return
        this.#pushStep(Step.GetMemberInfo)
        this.visitAll(fieldNode)
        this.visitAll(methodNode)
        this.#popStep(Step.GetMemberInfo)
    }
    classMemberDeclaration(ctx: ClassMemberDeclarationCtx) {
        this.#memberDeclaration(ctx.fieldDeclaration, ctx.methodDeclaration)
    }
    interfaceMemberDeclaration(ctx: InterfaceMemberDeclarationCtx) {
        this.#memberDeclaration(ctx.constantDeclaration, ctx.interfaceMethodDeclaration)
    }

    #fieldDeclaration(ctx: FieldDeclarationCtx | ConstantDeclarationCtx) {
        if (this.#getStep() !== Step.GetMemberInfo) return
        this.#pushStep(Step.GetType)
        this.visitAll(ctx.unannType)
        this.#pushStep(Step.GetMemberName)
        this.visitAll(ctx.variableDeclaratorList)
    }
    fieldDeclaration(ctx: FieldDeclarationCtx) {
        this.#fieldDeclaration(ctx)
    }
    constantDeclaration(ctx: ConstantDeclarationCtx) {
        this.#fieldDeclaration(ctx)
    }

    variableDeclarator(ctx: VariableDeclaratorCtx) {
        if (this.#getStep() !== Step.GetMemberName) return
        this.fields.push([
            ctx.variableDeclaratorId[0].children.Identifier[0].image,
            this.#_savedType,
            ctx.variableInitializer === undefined ? undefined : ctx.variableInitializer[0].children,
        ])
        this.#popStep(Step.GetMemberName)
    }

    #methodDeclaration(ctx: MethodDeclarationCtx | InterfaceMethodDeclarationCtx) {
        if (this.#getStep() !== Step.GetMemberInfo) return
        this.#_shouldSaveMethod = false
        this.visitAll(ctx.methodHeader)

        if (this.#_shouldSaveMethod) {
            const block = ctx.methodBody[0].children.block
            this.nbtMethods.push([
                this.#_savedParameterName,
                block === undefined
                    ? []
                    : block[0].children.blockStatements === undefined
                    ? []
                    : block[0].children.blockStatements,
            ])
        }
    }
    methodDeclaration(ctx: MethodDeclarationCtx) {
        this.#methodDeclaration(ctx)
    }
    interfaceMethodDeclaration(ctx: InterfaceMethodDeclarationCtx) {
        this.#methodDeclaration(ctx)
    }

    methodHeader(ctx: MethodHeaderCtx) {
        if (this.#getStep() !== Step.GetMemberInfo) return
        this.#pushStep(Step.GetType)
        this.visitAll(ctx.result)
        this.#pushStep(Step.GetMemberName)
        this.visitAll(ctx.methodDeclarator)
    }

    methodDeclarator(ctx: MethodDeclaratorCtx) {
        if (this.#getStep() !== Step.GetMemberName) return
        const methodName = ctx.Identifier[0].image
        this.methods.push([methodName, this.#_savedType])
        this.#popStep(Step.GetMemberName)

        if (methodName.startsWith('write') || methodName.startsWith('save')) {
            this.#pushStep(Step.TestForNbtParameters)
            this.visitAll(ctx.formalParameterList)
            this.#popStep(Step.TestForNbtParameters)
        }
    }

    formalParameterList(ctx: FormalParameterListCtx) {
        if (this.#getStep() !== Step.TestForNbtParameters) return
        if (ctx.formalParameter.length !== 1) return
        this.visitAll(ctx.formalParameter)
    }

    variableParaRegularParameter(ctx: VariableParaRegularParameterCtx) {
        if (this.#getStep() !== Step.TestForNbtParameters) return
        this.#pushStep(Step.GetType)
        this.visitAll(ctx.unannType)
        if (this.#_savedType === 'NbtCompound') {
            this.#_shouldSaveMethod = true
            this.#_savedParameterName = ctx.variableDeclaratorId[0].children.Identifier[0].image
        }
    }

    result(ctx: ResultCtx) {
        if (this.#getStep() !== Step.GetType) return
        if (ctx.Void !== undefined) {
            this.#_savedType = ctx.Void[0].image
            this.#popStep(Step.GetType)
        }
        this.visitAll(ctx.unannType)
    }

    unannPrimitiveTypeWithOptionalDimsSuffix(ctx: UnannPrimitiveTypeWithOptionalDimsSuffixCtx) {
        if (this.#getStep() !== Step.GetType) return
        this.visitAll(ctx.unannPrimitiveType)
        if (ctx.dims !== undefined) this.#_savedType += '[]'
    }

    unannPrimitiveType(ctx: UnannPrimitiveTypeCtx) {
        if (this.#getStep() !== Step.GetType) return
        if (ctx.Boolean !== undefined) {
            this.#_savedType = ctx.Boolean[0].image
            this.#popStep(Step.GetType)
        }
        this.visitAll(ctx.numericType)
    }

    integralType(ctx: IntegralTypeCtx) {
        if (this.#getStep() !== Step.GetType) return
        if (ctx.Byte !== undefined) {
            this.#_savedType = ctx.Byte[0].image
        } else if (ctx.Short !== undefined) {
            this.#_savedType = ctx.Short[0].image
        } else if (ctx.Int !== undefined) {
            this.#_savedType = ctx.Int[0].image
        } else if (ctx.Long !== undefined) {
            this.#_savedType = ctx.Long[0].image
        } else if (ctx.Char !== undefined) {
            this.#_savedType = ctx.Char[0].image
        }
        this.#popStep(Step.GetType)
    }

    floatingPointType(ctx: FloatingPointTypeCtx) {
        if (this.#getStep() !== Step.GetType) return
        if (ctx.Float !== undefined) {
            this.#_savedType = ctx.Float[0].image
        } else if (ctx.Double !== undefined) {
            this.#_savedType = ctx.Double[0].image
        }
        this.#popStep(Step.GetType)
    }

    unannReferenceType(ctx: UnannReferenceTypeCtx) {
        if (this.#getStep() !== Step.GetType) return
        this.visitAll(ctx.unannClassOrInterfaceType)
        if (ctx.dims !== undefined) this.#_savedType += '[]'
    }

    unannClassType(ctx: UnannClassTypeCtx) {
        if (this.#getStep() !== Step.GetType) return
        this.#_savedType = ctx.Identifier.slice(-1)[0].image
        this.#popStep(Step.GetType)
    }
}
