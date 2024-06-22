namespace Options;

@summary("Options for simplifying an expression")
model SimplifyOptions {
  @summary("Whether to simplify the expression")
  simplify?: boolean = true;

  @summary("Whether to ignore case when simplifying")
  ignoreCase?: boolean = false;
}

@summary("Sort order for a truth table")
enum Sort {
  DEFAULT,
  TRUE_FIRST,
  FALSE_FIRST,
}

@summary("Whether to hide specific rows in a truth table")
enum Hide {
  NONE,
  TRUE,
  FALSE,
}

@summary("Options for generating a truth table")
model TableOptions {
  @summary("Sort order for the truth table")
  sort?: Sort = Sort.DEFAULT;

  @summary("Hide specific rows in the truth table")
  hide?: Hide = Hide.NONE;

  /**
   * Hide intermediate steps when generating the truth table
   * For example in the expression "A and B or C", the intermediate step is: A and B
   */
  @summary("Hide intermediate steps when generating the truth table")
  hideIntermediateSteps?: boolean = false;
}

@summary("Options for simplifying an expression and generating a truth table")
model SimplifyTableOptions {
  ...SimplifyOptions;
  ...TableOptions;
}
