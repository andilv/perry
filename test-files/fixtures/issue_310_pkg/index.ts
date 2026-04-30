// Re-exporter — mirrors `effect/src/index.ts:229`'s `export * as Effect
// from "./Effect.js"` shape that drove issue #310. Pre-fix Perry's
// ExportNamed lowering only matched `ExportSpecifier::Named` and
// silently dropped Namespace specifiers; the consumer's `import { Foo }`
// then resolved to nothing and every `Foo.<member>` lowered to 0.

export * as Foo from "./Foo.ts";
export * as Bar from "./Bar.ts";

// Mix in a regular re-export to confirm we don't regress that path while
// adding the namespace-re-export branch.
export { tag as fooTag } from "./Foo.ts";
