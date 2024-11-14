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

export function addButton(txt: string, {
    tag,
    anchor: anch,
    pos: position,
    fontSize: fontSize = 30
}): GameObj<any> {
    const w = 0.5 * fontSize * txt.length + 10;
    const btn = add([
        tag,
        "button",
        anchor(anch),
        area(),
        position,
        color(DARK_BROWN),
        rect(w, fontSize * 2),
        outline(4, BLACK),
        layer("ui"),
        fixed(),
    ]).add([
        color(WHITE),
        text(txt, { width: w, align: "center", size: fontSize, font: "monospace" }),
        anchor(anch),
        fixed(),
    ]);

    return btn;
}
