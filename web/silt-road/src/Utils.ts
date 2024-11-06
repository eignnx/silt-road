
export function camScaleUnaffected() {
    return {
        id: "camScaleUnaffected",
        requires: ["scale"],
        inspect() {
            return "camScaleUnaffected";
        },
        update() {
            this.scaleTo(1 / camScale().x);
        }
    };
}
