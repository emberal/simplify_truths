import "@typespec/openapi3";
using TypeSpec.OpenAPI;

enum BinaryOperator {
    AND,
    OR,
    IMPLICATION
}

model ExpressionNot {
    not: Expression;
}

// TODO tuple type, possible in OpenAPI 3.1, but unsupported in typespec
model ExpressionBinary {
    operator: BinaryOperator;
    left: Expression;
    right: Expression;
}

model ExpressionAtomic {
    atomic: string;
}

@oneOf
union Expression {
    ExpressionNot;
    ExpressionBinary;
    ExpressionAtomic;
}

model SimplifyResponse {
    before: string;
    after: string;
    orderOfOperations: string[];
    expression: Expression;
}

op simplify(): SimplifyResponse;
