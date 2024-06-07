import "@typespec/openapi3";
using TypeSpec.OpenAPI;

namespace Models;

enum BinaryOperator {
    AND,
    OR,
    IMPLICATION
}

model ExpressionNot {
    not: Expression;
}

model ExpressionBinary {
    left: Expression;
    operator: BinaryOperator;
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