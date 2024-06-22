import "@typespec/http";
import "@typespec/versioning";
import "./models.tsp";
import "./response.tsp";
import "./options.tsp";
using TypeSpec.Http;
using TypeSpec.Versioning;
using Models;
using Responses;
using Options;

/**
 * A service to simplify truth expressions, and generate truth tables.
 */
@service({
  title: "Simplify Truth API",
})
@versioned(Version)
namespace SimplifyTruths;

enum Version {
  v2,
}

@tag("Common")
interface Index {
  /**
   * Information about this API.
   */
  @get
  @summary("Information")
  index(): InfoResponse;

  /**
   * The OpenAPI specification for this API.
   */
  @get
  @route("/openapi")
  @summary("The OpenAPI specification")
  openAPI(): HTML;

  /**
   * Check if an expression is valid.
   */
  @get
  @tag("Expression")
  @route("/is-valid")
  @summary("Check if an expression is valid")
  isValid(@path exp: string): IsValidResponse;
}

@tag("Expression")
@route("/simplify")
interface Simplify {
  @get
  @summary("Simplify a truth expression")
  simplify(@path exp: string, @query query?: SimplifyOptions): SimplifyResponse;

  @get
  @tag("Table")
  @route("/table")
  @summary("Simplify and generate a truth table")
  simplifyTable(
    @path exp: string,
    @query query?: SimplifyTableOptions,
  ): SimplifyTableResponse;
}

@tag("Table")
@route("/table")
interface TruthTable {
  @get
  @summary("Generate a truth table")
  simplify(@path exp: string, @query query?: TableOptions): TruthTableResponse;
}
