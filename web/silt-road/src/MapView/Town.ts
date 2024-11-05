import { GameObj, KAPLAYCtx } from 'kaplay';

export function plugin(k: KAPLAYCtx) {
    return {
        spawnTown(idx: number): GameObj<any> {
            return k.add([
                "town",
                townIdx(idx),
                k.pos(k.rand(k.width()), k.rand(k.height())),
                k.circle(16),
                k.area(),
                k.color(
                    k.randi(50, 200),
                    k.randi(50, 200),
                    k.randi(20, 200)
                ),
                toggleHighlight(),
            ]).add([
                k.text(`${idx}`),
                k.anchor("center"),
            ]);
        },

    };

    function toggleHighlight() {
        return {
            id: "toggleHighlight",
            inspect() {
                return this.is("outline") ? "highlighted" : "not highlighted";
            },
            toggleHighlight() {
                if (this.is("outline")) {
                    this.unuse("outline");
                } else {
                    this.use(k.outline(5, k.WHITE));
                }
            }
        };
    }
}

export function townIdx(idx: number) {
    return {
        id: "townIdx",
        idx,
        inspect() {
            return `townIdx(${idx})`;
        }
    };
}

