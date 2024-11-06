const townNameAdjectives: string[] = [
    "Dry", "Green", "Stone", "Bright", "New", "Old", "Red",
    "Dead", "Lost", "Hidden", "Sunny", "Dusty", "Golden",
    "Silver", "Rusty", "Iron"
];
const townNameNouns: string[] = [
    "Valley", "Hill", "Brook", "Point", "Creek", "Bend",
    "Saddle", "Spur", "Canyon", "Gulch", "Antler",
    "Prairie", "Mesa", "Bluff", "Grove", "Gulley",
    "Ridge", "Gorge", "Crest", "Meadow", "Flats",
    "Pass", "Hollow", "Butte", "Peak",
    "Dune", "Sands",
    "Iron", "Steel", "Sagebrush",
];
const townNameSuffixes: string[] = [
    "dale", "burg", "ton", "field", "wood", "town", "ford",
    "ville"
];
const townNamePrefixes: string[] = [
    "Bramble", "James", "Hazel", "Asp", "Rat", "Rattlers",
    "Miners", "Millet", "Daggers", "Devils", "Copper",
    "Lone", "Tumble", "Rust", "Sage",
    "Cotton", "Zephyr", "Cedar"
];

export function randomTownName() {
    if (rand() < 0.5) {
        return `${choose(townNameAdjectives)} ${choose(townNameNouns)}`;
    } else {
        return `${choose(townNamePrefixes)}${choose(townNameSuffixes)}`;
    }
}
