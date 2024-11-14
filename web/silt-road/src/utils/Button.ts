import { GameObj } from 'kaplay';
import { BROWN, DARK_BROWN } from '../Globals';

export function setupBtnHovers() {
    onHover("button", btn => {
        btn.color = BROWN;
    });
    onHoverEnd("button", btn => {
        btn.color = DARK_BROWN;
    });
}

export function addButton(txt: string, { tag, anchor: anch, pos: position }): GameObj<any> {
    const w = 24 * txt.length + 10;
    const btn = add([
        tag,
        "button",
        anchor(anch),
        area(),
        position,
        color(BLACK),
        rect(w, 60),
        outline(8, BLACK),
        layer("ui"),
        fixed(),
    ]).add([
        color(WHITE),
        text(txt, { width: w, align: "center" }),
        anchor(anch),
        fixed(),
    ]);

    return btn;
}
