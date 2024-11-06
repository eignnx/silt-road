import { GameObj, KAPLAYCtx, TweenController } from 'kaplay';

export function plugin(k: KAPLAYCtx) {
    return {
        spawnTown(idx: number): GameObj<any> {
            return k.add([
                "town",
                townIdx(idx),
                k.pos(k.rand(k.width()), k.rand(k.height())),
                k.circle(16),
                k.color(
                    k.randi(50, 200),
                    k.randi(50, 200),
                    k.randi(20, 200)
                ),
                k.area(),
                k.outline(0, k.WHITE),
                highlightToggleable(),
            ]).add([
                k.text(`${idx}`),
                k.anchor("center"),
            ]);
        },

    };

    function highlightToggleable() {
        return {
            id: "toggleHighlight",
            inspect() {
                const summary: string[] = [];
                if (this.is("highlighted")) summary.push("highlighted");
                if (this.is("highlightTweening")) summary.push("tweening");
                return `highlight: ${summary}`;
            },
            addHighlight() {
                if (this.is("highlightTweening")) return;
                this.use("highlightTweening");
                k.tween(1, 7, 1.0,
                    val => this.outline.width = val,
                    k.easings.easeOutElastic,
                ).then(() => {
                    this.unuse("highlightTweening");
                    this.use("highlighted");
                });
            },
            removeHighlight() {
                if (!this.is("highlighted") && !this.is("highlightTweening")) return;
                this.use("highlightTweening");
                k.tween(7, 1, 2.0,
                    val => this.outline.width = val,
                    k.easings.easeOutElastic
                ).then(() => {
                    this.unuse("highlighted");
                    this.unuse("highlightTweening");
                });
            },
            toggleHighlight() {
                if (this.is("highlighted")) {
                    this.unuse("highlighted");
                    k.tween(5, 0, 3,
                        val => this.outline.width = val,
                        k.easings.easeInOutExpo
                    );
                } else if (!this.is("highlightTweening")) {
                    this.use("highlightTweening");
                    k.tween(0, 5, 3.0,
                        val => this.outline.width = val,
                        k.easings.easeOutElastic,
                    ).then(() => {
                        this.unuse("highlightTweening");
                        this.use("highlighted");
                    });
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

