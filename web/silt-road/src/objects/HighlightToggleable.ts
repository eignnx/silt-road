import { Comp } from 'kaplay';

export interface HighlightToggleableComp extends Comp {
    id: "highlightToggleable";
    addHighlight(): void;
    removeHighlight(): void;
}

export function highlightToggleable(): HighlightToggleableComp {
    return {
        id: "highlightToggleable",
        inspect() {
            const summary: string[] = [];
            if (this.is("highlighted")) summary.push("highlighted");
            if (this.is("highlightTweening")) summary.push("tweening");
            return `highlight: ${summary}`;
        },
        addHighlight() {
            if (this.is("highlightTweening")) return;
            this.use("highlightTweening");
            tween(1, 7, 1.0,
                val => this.outline.width = val,
                easings.easeOutElastic,
            ).then(() => {
                this.unuse("highlightTweening");
                this.use("highlighted");
            });
        },
        removeHighlight() {
            if (!this.is("highlighted") && !this.is("highlightTweening")) return;
            this.use("highlightTweening");
            tween(7, 1, 2.0,
                val => this.outline.width = val,
                easings.easeOutElastic
            ).then(() => {
                this.unuse("highlighted");
                this.unuse("highlightTweening");
            });
        },
    };
}
