import "@typespec/http";
import "./models.tsp";
using TypeSpec.Http;
using Models;

namespace Responses;

/**
 * Returns a response as HTML, with the content type set to "text/html".
 */
@summary("Returns a response as HTML")
model HTML {
  @header contentType: "text/html";
  @body _: string;
}

/**
 * Information about this API.
 */
@summary("Information")
model InfoResponse {
  message: string;
  docs: string;
  createdBy: string;
}

/**
 * If an expression is valid.
 */
@summary("If an expression is valid")
model IsValidResponse {
  isValid: boolean;
}

/**
 * Response after simplifying an expression.
 */
@summary("Simplify Response")
model SimplifyResponse {
  @summary("Before simplification")
  before: string;

  @summary("After simplification")
  after: string;

  @summary("Steps taken to simplify")
  operations: string[];

  @summary("The simplified expression")
  expression: Expression;
}

/**
 * Response after generating a truth table.
 */
@summary("Truth Table Response")
model TruthTableResponse {
  @summary("The truth table")
  truthTable: Models.TruthTable;
}

/**
 * Response after simplifying an expression and generating a truth table.
 */
@summary("Simplify and Table Response")
model SimplifyTableResponse {
  ...SimplifyResponse;
  ...TruthTableResponse;
}
