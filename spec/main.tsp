import "@typespec/http";
import "@typespec/versioning";
import "./models.tsp";
using TypeSpec.Http;
using TypeSpec.Versioning;
using Models;

enum Version {
    v2
}

@versioned(Version)
@service({
    title: "Simplify Truth Expressions",
    description: "Simplify truth expressions",
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

    @route("/simplify")
    interface Simplify {
        @get op simplify(
            @path exp: string,
            @query query?: SimplifyOptions
        ): SimplifyResponse;
    }
}
