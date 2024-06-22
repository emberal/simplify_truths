import "@typespec/openapi3";
using TypeSpec.OpenAPI;

namespace Models;

@summary("A binary operator")
enum BinaryOperator {
  AND,
  OR,
  IMPLICATION,
}

@summary("The inverse of an expression")
model ExpressionNot {
  @summary("The expression to negate")
  not: Expression;
}

@summary("A binary expression")
model ExpressionBinary {
  @summary("The left expression")
  left: Expression;

  @summary("The binary operator")
  operator: BinaryOperator;

  @summary("The right expression")
  right: Expression;
}

@summary("An atomic expression")
model ExpressionAtomic {
  @summary("The atomic value")
  atomic: string;
}

@oneOf
@summary("A truth expression")
union Expression {
  ExpressionNot,
  ExpressionBinary,
  ExpressionAtomic,
}

@summary("A truth table")
model TruthTable {
  @summary("The header of the truth table")
  header: string[];

  @summary("The rows and columns of the truth table")
  truthMatrix: boolean[][];
}
