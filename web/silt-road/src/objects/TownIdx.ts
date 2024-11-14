export function townIdx(idx: number) {
    return {
        id: "townIdx",
        idx,
        inspect() {
            return `townIdx(${idx})`;
        }
    };
}
