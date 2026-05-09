// Issue #637 followup / hono r2: closure-captured array IndexSet
// pre-fix used js_array_set_f64 (non-extending) which silently no-op'd
// when index >= length. v0.5.737 switches to js_array_set_f64_extend.
function go() {
    const arr: any[] = [];
    const fn = () => {
        arr[0] = "a";
        arr[2] = "c"; // sparse extend — gap at index 1
    };
    fn();
    console.log("len:", arr.length);
    console.log("arr[0]:", arr[0]);
    console.log("arr[1]:", arr[1]);
    console.log("arr[2]:", arr[2]);
    console.log("json:", JSON.stringify(arr));
}
go();

// Issue #637 / hono Trie pattern: arr[++captureIndex] = N inside replace cb
function build() {
    const indexReplacementMap: any[] = [];
    let captureIndex = 0;
    const input = "/users/([^/]+)@0#0";
    input.replace(/#(\d+)|@(\d+)/g, (_, h, p) => {
        if (h !== undefined) {
            indexReplacementMap[++captureIndex] = Number(h);
            return "$()";
        }
        if (p !== undefined) {
            ++captureIndex;
            return "";
        }
        return "";
    });
    console.log("irm len:", indexReplacementMap.length);
    console.log("irm:", JSON.stringify(indexReplacementMap));
}
build();

// Issue #637: arr[stringKey] = X coercion via direct closure call
// (forEach version is a known followup — captures-array-typeinfo gap)
function strKey() {
    const out: any[] = [];
    const fn = (k: string) => {
        out[k as any] = "h" + k;
    };
    fn("0");
    fn("1");
    fn("2");
    console.log("out len:", out.length);
    console.log("out:", JSON.stringify(out));
}
strKey();
