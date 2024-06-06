/**
 * Encode the given string as a URI component, and set the request variable "expression" to the result.
 * @param {string} expression
 * @returns {void}
 */
export function expression(expression) {
    request.variables.set("expression", encodeURIComponent(expression))
}
