import "@typespec/http";
import "./models.tsp";
using TypeSpec.Http;
using Models;

@service({
    title: "Simplify Truth Expressions",
})
namespace Simplify {
    model SimplifyResponse {
    before: string;
    after: string;
    orderOfOperations?: string[] = [];
    expression: Expression;
}

model SimplifyOptions {
    lang: "en" | "nb" = "en";
    simplify: boolean = true;
    caseSensitive: boolean = false;
}

op simplify(
    @path exp: string,
    @query query?: SimplifyOptions
): SimplifyResponse;
}
