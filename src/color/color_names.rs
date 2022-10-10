use std::collections::HashMap;

use crate::color::color::Color;

use super::color::AlphaPosition;

/// Get the name of the color.
///
/// The name will be name, if none of the datasets contain it.
/// It will return the first found name, searching the w3c basic set, then the w3c extend set
/// and lastly the xkcd set.
pub fn name(color: Color, basic: bool, extended: bool, xkcd: bool) -> Option<String> {
    //this is horrible syntax
    //it would be easier to do if basic && elt Some...,
    //but thats not implemented yet
    if basic {
        if let Some(name) = w3c_basic_names().get(&color) {
            return Some(name.to_string());
        } else {
            return name(color, false, extended, xkcd);
        }
    } else if extended {
        if let Some(name) = w3c_extended_names().get(&color) {
            return Some(name.to_string());
        } else {
            return name(color, basic, false, xkcd);
        }
    } else if xkcd {
        if let Some(name) = xkcd_names().get(&color) {
            return Some(name.to_string());
        } else {
            return name(color, basic, extended, false);
        }
    } else {
        None
    }
}

/// Returns the [w3c basic color keywords](https://www.w3.org/TR/css-color-3/#html4).
///
/// The names are mapped to their corresponding color.
/// To get the name of a color the [name] function is preferred.
pub fn w3c_basic_names() -> HashMap<Color, &'static str> {
    HashMap::from([
        (Color::rgb(0, 0, 0), "black"),
        (Color::rgb(192, 192, 192), "silver"),
        (Color::rgb(128, 128, 128), "gray"),
        (Color::rgb(255, 255, 255), "white"),
        (Color::rgb(128, 0, 0), "maroon"),
        (Color::rgb(255, 0, 0), "red"),
        (Color::rgb(128, 0, 128), "purple"),
        (Color::rgb(255, 0, 255), "fuchsia"),
        (Color::rgb(0, 128, 0), "green"),
        (Color::rgb(0, 255, 0), "lime"),
        (Color::rgb(128, 128, 0), "olive"),
        (Color::rgb(255, 255, 0), "yellow"),
        (Color::rgb(0, 0, 128), "navy"),
        (Color::rgb(0, 0, 255), "blue"),
        (Color::rgb(0, 128, 128), "teal"),
        (Color::rgb(0, 255, 255), "aqua"),
    ])
}

/// Returns the [w3c extended color keywords](https://www.w3.org/TR/css-color-3/#svg-color).
///
/// The names are mapped to their corresponding color.
/// To get the name of a color the [name] function is preferred.
pub fn w3c_extended_names() -> HashMap<Color, &'static str> {
    HashMap::from([
        (Color::rgb(240, 248, 255), "aliceblue"),
        (Color::rgb(250, 235, 215), "antiquewhite"),
        (Color::rgb(0, 255, 255), "aqua"),
        (Color::rgb(127, 255, 212), "aquamarine"),
        (Color::rgb(240, 255, 255), "azure"),
        (Color::rgb(245, 245, 220), "beige"),
        (Color::rgb(255, 228, 196), "bisque"),
        (Color::rgb(0, 0, 0), "black"),
        (Color::rgb(255, 235, 205), "blanchedalmond"),
        (Color::rgb(0, 0, 255), "blue"),
        (Color::rgb(138, 43, 226), "blueviolet"),
        (Color::rgb(165, 42, 42), "brown"),
        (Color::rgb(222, 184, 135), "burlywood"),
        (Color::rgb(95, 158, 160), "cadetblue"),
        (Color::rgb(127, 255, 0), "chartreuse"),
        (Color::rgb(210, 105, 30), "chocolate"),
        (Color::rgb(255, 127, 80), "coral"),
        (Color::rgb(100, 149, 237), "cornflowerblue"),
        (Color::rgb(255, 248, 220), "cornsilk"),
        (Color::rgb(220, 20, 60), "crimson"),
        (Color::rgb(0, 255, 255), "cyan"),
        (Color::rgb(0, 0, 139), "darkblue"),
        (Color::rgb(0, 139, 139), "darkcyan"),
        (Color::rgb(184, 134, 11), "darkgoldenrod"),
        (Color::rgb(169, 169, 169), "darkgray"),
        (Color::rgb(0, 100, 0), "darkgreen"),
        (Color::rgb(169, 169, 169), "darkgrey"),
        (Color::rgb(189, 183, 107), "darkkhaki"),
        (Color::rgb(139, 0, 139), "darkmagenta"),
        (Color::rgb(85, 107, 47), "darkolivegreen"),
        (Color::rgb(255, 140, 0), "darkorange"),
        (Color::rgb(153, 50, 204), "darkorchid"),
        (Color::rgb(139, 0, 0), "darkred"),
        (Color::rgb(233, 150, 122), "darksalmon"),
        (Color::rgb(143, 188, 143), "darkseagreen"),
        (Color::rgb(72, 61, 139), "darkslateblue"),
        (Color::rgb(47, 79, 79), "darkslategray"),
        (Color::rgb(47, 79, 79), "darkslategrey"),
        (Color::rgb(0, 206, 209), "darkturquoise"),
        (Color::rgb(148, 0, 211), "darkviolet"),
        (Color::rgb(255, 20, 147), "deeppink"),
        (Color::rgb(0, 191, 255), "deepskyblue"),
        (Color::rgb(105, 105, 105), "dimgray"),
        (Color::rgb(105, 105, 105), "dimgrey"),
        (Color::rgb(30, 144, 255), "dodgerblue"),
        (Color::rgb(178, 34, 34), "firebrick"),
        (Color::rgb(255, 250, 240), "floralwhite"),
        (Color::rgb(34, 139, 34), "forestgreen"),
        (Color::rgb(255, 0, 255), "fuchsia"),
        (Color::rgb(220, 220, 220), "gainsboro"),
        (Color::rgb(248, 248, 255), "ghostwhite"),
        (Color::rgb(255, 215, 0), "gold"),
        (Color::rgb(218, 165, 32), "goldenrod"),
        (Color::rgb(128, 128, 128), "gray"),
        (Color::rgb(0, 128, 0), "green"),
        (Color::rgb(173, 255, 47), "greenyellow"),
        (Color::rgb(128, 128, 128), "grey"),
        (Color::rgb(240, 255, 240), "honeydew"),
        (Color::rgb(255, 105, 180), "hotpink"),
        (Color::rgb(205, 92, 92), "indianred"),
        (Color::rgb(75, 0, 130), "indigo"),
        (Color::rgb(255, 255, 240), "ivory"),
        (Color::rgb(240, 230, 140), "khaki"),
        (Color::rgb(230, 230, 250), "lavender"),
        (Color::rgb(255, 240, 245), "lavenderblush"),
        (Color::rgb(124, 252, 0), "lawngreen"),
        (Color::rgb(255, 250, 205), "lemonchiffon"),
        (Color::rgb(173, 216, 230), "lightblue"),
        (Color::rgb(240, 128, 128), "lightcoral"),
        (Color::rgb(224, 255, 255), "lightcyan"),
        (Color::rgb(250, 250, 210), "lightgoldenrodyellow"),
        (Color::rgb(211, 211, 211), "lightgray"),
        (Color::rgb(144, 238, 144), "lightgreen"),
        (Color::rgb(211, 211, 211), "lightgrey"),
        (Color::rgb(255, 182, 193), "lightpink"),
        (Color::rgb(255, 160, 122), "lightsalmon"),
        (Color::rgb(32, 178, 170), "lightseagreen"),
        (Color::rgb(135, 206, 250), "lightskyblue"),
        (Color::rgb(119, 136, 153), "lightslategray"),
        (Color::rgb(119, 136, 153), "lightslategrey"),
        (Color::rgb(176, 196, 222), "lightsteelblue"),
        (Color::rgb(255, 255, 224), "lightyellow"),
        (Color::rgb(0, 255, 0), "lime"),
        (Color::rgb(50, 205, 50), "limegreen"),
        (Color::rgb(250, 240, 230), "linen"),
        (Color::rgb(255, 0, 255), "magenta"),
        (Color::rgb(128, 0, 0), "maroon"),
        (Color::rgb(102, 205, 170), "mediumaquamarine"),
        (Color::rgb(0, 0, 205), "mediumblue"),
        (Color::rgb(186, 85, 211), "mediumorchid"),
        (Color::rgb(147, 112, 219), "mediumpurple"),
        (Color::rgb(60, 179, 113), "mediumseagreen"),
        (Color::rgb(123, 104, 238), "mediumslateblue"),
        (Color::rgb(0, 250, 154), "mediumspringgreen"),
        (Color::rgb(72, 209, 204), "mediumturquoise"),
        (Color::rgb(199, 21, 133), "mediumvioletred"),
        (Color::rgb(25, 25, 112), "midnightblue"),
        (Color::rgb(245, 255, 250), "mintcream"),
        (Color::rgb(255, 228, 225), "mistyrose"),
        (Color::rgb(255, 228, 181), "moccasin"),
        (Color::rgb(255, 222, 173), "navajowhite"),
        (Color::rgb(0, 0, 128), "navy"),
        (Color::rgb(253, 245, 230), "oldlace"),
        (Color::rgb(128, 128, 0), "olive"),
        (Color::rgb(107, 142, 35), "olivedrab"),
        (Color::rgb(255, 165, 0), "orange"),
        (Color::rgb(255, 69, 0), "orangered"),
        (Color::rgb(218, 112, 214), "orchid"),
        (Color::rgb(238, 232, 170), "palegoldenrod"),
        (Color::rgb(152, 251, 152), "palegreen"),
        (Color::rgb(175, 238, 238), "paleturquoise"),
        (Color::rgb(219, 112, 147), "palevioletred"),
        (Color::rgb(255, 239, 213), "papayawhip"),
        (Color::rgb(255, 218, 185), "peachpuff"),
        (Color::rgb(205, 133, 63), "peru"),
        (Color::rgb(255, 192, 203), "pink"),
        (Color::rgb(221, 160, 221), "plum"),
        (Color::rgb(176, 224, 230), "powderblue"),
        (Color::rgb(128, 0, 128), "purple"),
        (Color::rgb(255, 0, 0), "red"),
        (Color::rgb(188, 143, 143), "rosybrown"),
        (Color::rgb(65, 105, 225), "royalblue"),
        (Color::rgb(139, 69, 19), "saddlebrown"),
        (Color::rgb(250, 128, 114), "salmon"),
        (Color::rgb(244, 164, 96), "sandybrown"),
        (Color::rgb(46, 139, 87), "seagreen"),
        (Color::rgb(255, 245, 238), "seashell"),
        (Color::rgb(160, 82, 45), "sienna"),
        (Color::rgb(192, 192, 192), "silver"),
        (Color::rgb(135, 206, 235), "skyblue"),
        (Color::rgb(106, 90, 205), "slateblue"),
        (Color::rgb(112, 128, 144), "slategray"),
        (Color::rgb(112, 128, 144), "slategrey"),
        (Color::rgb(255, 250, 250), "snow"),
        (Color::rgb(0, 255, 127), "springgreen"),
        (Color::rgb(70, 130, 180), "steelblue"),
        (Color::rgb(210, 180, 140), "tan"),
        (Color::rgb(0, 128, 128), "teal"),
        (Color::rgb(216, 191, 216), "thistle"),
        (Color::rgb(255, 99, 71), "tomato"),
        (Color::rgb(64, 224, 208), "turquoise"),
        (Color::rgb(238, 130, 238), "violet"),
        (Color::rgb(245, 222, 179), "wheat"),
        (Color::rgb(255, 255, 255), "white"),
        (Color::rgb(245, 245, 245), "whitesmoke"),
        (Color::rgb(255, 255, 0), "yellow"),
        (Color::rgb(154, 205, 50), "yellowgreen"),
    ])
}

/// Returns the [xkcd color names](https://xkcd.com/color/rgb.txt).
///
/// The names are mapped to their corresponding color.
/// To get the name of a color the [name] function is preferred.
pub fn xkcd_names() -> HashMap<Color, &'static str> {
    HashMap::from([
        (
            Color::from_hex("#acc2d9", AlphaPosition::None).unwrap(),
            "cloudy blue",
        ),
        (
            Color::from_hex("#56ae57", AlphaPosition::None).unwrap(),
            "dark pastel green",
        ),
        (
            Color::from_hex("#b2996e", AlphaPosition::None).unwrap(),
            "dust",
        ),
        (
            Color::from_hex("#a8ff04", AlphaPosition::None).unwrap(),
            "electric lime",
        ),
        (
            Color::from_hex("#69d84f", AlphaPosition::None).unwrap(),
            "fresh green",
        ),
        (
            Color::from_hex("#894585", AlphaPosition::None).unwrap(),
            "light eggplant",
        ),
        (
            Color::from_hex("#70b23f", AlphaPosition::None).unwrap(),
            "nasty green",
        ),
        (
            Color::from_hex("#d4ffff", AlphaPosition::None).unwrap(),
            "really light blue",
        ),
        (
            Color::from_hex("#65ab7c", AlphaPosition::None).unwrap(),
            "tea",
        ),
        (
            Color::from_hex("#952e8f", AlphaPosition::None).unwrap(),
            "warm purple",
        ),
        (
            Color::from_hex("#fcfc81", AlphaPosition::None).unwrap(),
            "yellowish tan",
        ),
        (
            Color::from_hex("#a5a391", AlphaPosition::None).unwrap(),
            "cement",
        ),
        (
            Color::from_hex("#388004", AlphaPosition::None).unwrap(),
            "dark grass green",
        ),
        (
            Color::from_hex("#4c9085", AlphaPosition::None).unwrap(),
            "dusty teal",
        ),
        (
            Color::from_hex("#5e9b8a", AlphaPosition::None).unwrap(),
            "grey teal",
        ),
        (
            Color::from_hex("#efb435", AlphaPosition::None).unwrap(),
            "macaroni and cheese",
        ),
        (
            Color::from_hex("#d99b82", AlphaPosition::None).unwrap(),
            "pinkish tan",
        ),
        (
            Color::from_hex("#0a5f38", AlphaPosition::None).unwrap(),
            "spruce",
        ),
        (
            Color::from_hex("#0c06f7", AlphaPosition::None).unwrap(),
            "strong blue",
        ),
        (
            Color::from_hex("#61de2a", AlphaPosition::None).unwrap(),
            "toxic green",
        ),
        (
            Color::from_hex("#3778bf", AlphaPosition::None).unwrap(),
            "windows blue",
        ),
        (
            Color::from_hex("#2242c7", AlphaPosition::None).unwrap(),
            "blue blue",
        ),
        (
            Color::from_hex("#533cc6", AlphaPosition::None).unwrap(),
            "blue with a hint of purple",
        ),
        (
            Color::from_hex("#9bb53c", AlphaPosition::None).unwrap(),
            "booger",
        ),
        (
            Color::from_hex("#05ffa6", AlphaPosition::None).unwrap(),
            "bright sea green",
        ),
        (
            Color::from_hex("#1f6357", AlphaPosition::None).unwrap(),
            "dark green blue",
        ),
        (
            Color::from_hex("#017374", AlphaPosition::None).unwrap(),
            "deep turquoise",
        ),
        (
            Color::from_hex("#0cb577", AlphaPosition::None).unwrap(),
            "green teal",
        ),
        (
            Color::from_hex("#ff0789", AlphaPosition::None).unwrap(),
            "strong pink",
        ),
        (
            Color::from_hex("#afa88b", AlphaPosition::None).unwrap(),
            "bland",
        ),
        (
            Color::from_hex("#08787f", AlphaPosition::None).unwrap(),
            "deep aqua",
        ),
        (
            Color::from_hex("#dd85d7", AlphaPosition::None).unwrap(),
            "lavender pink",
        ),
        (
            Color::from_hex("#a6c875", AlphaPosition::None).unwrap(),
            "light moss green",
        ),
        (
            Color::from_hex("#a7ffb5", AlphaPosition::None).unwrap(),
            "light seafoam green",
        ),
        (
            Color::from_hex("#c2b709", AlphaPosition::None).unwrap(),
            "olive yellow",
        ),
        (
            Color::from_hex("#e78ea5", AlphaPosition::None).unwrap(),
            "pig pink",
        ),
        (
            Color::from_hex("#966ebd", AlphaPosition::None).unwrap(),
            "deep lilac",
        ),
        (
            Color::from_hex("#ccad60", AlphaPosition::None).unwrap(),
            "desert",
        ),
        (
            Color::from_hex("#ac86a8", AlphaPosition::None).unwrap(),
            "dusty lavender",
        ),
        (
            Color::from_hex("#947e94", AlphaPosition::None).unwrap(),
            "purpley grey",
        ),
        (
            Color::from_hex("#983fb2", AlphaPosition::None).unwrap(),
            "purply",
        ),
        (
            Color::from_hex("#ff63e9", AlphaPosition::None).unwrap(),
            "candy pink",
        ),
        (
            Color::from_hex("#b2fba5", AlphaPosition::None).unwrap(),
            "light pastel green",
        ),
        (
            Color::from_hex("#63b365", AlphaPosition::None).unwrap(),
            "boring green",
        ),
        (
            Color::from_hex("#8ee53f", AlphaPosition::None).unwrap(),
            "kiwi green",
        ),
        (
            Color::from_hex("#b7e1a1", AlphaPosition::None).unwrap(),
            "light grey green",
        ),
        (
            Color::from_hex("#ff6f52", AlphaPosition::None).unwrap(),
            "orange pink",
        ),
        (
            Color::from_hex("#bdf8a3", AlphaPosition::None).unwrap(),
            "tea green",
        ),
        (
            Color::from_hex("#d3b683", AlphaPosition::None).unwrap(),
            "very light brown",
        ),
        (
            Color::from_hex("#fffcc4", AlphaPosition::None).unwrap(),
            "egg shell",
        ),
        (
            Color::from_hex("#430541", AlphaPosition::None).unwrap(),
            "eggplant purple",
        ),
        (
            Color::from_hex("#ffb2d0", AlphaPosition::None).unwrap(),
            "powder pink",
        ),
        (
            Color::from_hex("#997570", AlphaPosition::None).unwrap(),
            "reddish grey",
        ),
        (
            Color::from_hex("#ad900d", AlphaPosition::None).unwrap(),
            "baby shit brown",
        ),
        (
            Color::from_hex("#c48efd", AlphaPosition::None).unwrap(),
            "liliac",
        ),
        (
            Color::from_hex("#507b9c", AlphaPosition::None).unwrap(),
            "stormy blue",
        ),
        (
            Color::from_hex("#7d7103", AlphaPosition::None).unwrap(),
            "ugly brown",
        ),
        (
            Color::from_hex("#fffd78", AlphaPosition::None).unwrap(),
            "custard",
        ),
        (
            Color::from_hex("#da467d", AlphaPosition::None).unwrap(),
            "darkish pink",
        ),
        (
            Color::from_hex("#410200", AlphaPosition::None).unwrap(),
            "deep brown",
        ),
        (
            Color::from_hex("#c9d179", AlphaPosition::None).unwrap(),
            "greenish beige",
        ),
        (
            Color::from_hex("#fffa86", AlphaPosition::None).unwrap(),
            "manilla",
        ),
        (
            Color::from_hex("#5684ae", AlphaPosition::None).unwrap(),
            "off blue",
        ),
        (
            Color::from_hex("#6b7c85", AlphaPosition::None).unwrap(),
            "battleship grey",
        ),
        (
            Color::from_hex("#6f6c0a", AlphaPosition::None).unwrap(),
            "browny green",
        ),
        (
            Color::from_hex("#7e4071", AlphaPosition::None).unwrap(),
            "bruise",
        ),
        (
            Color::from_hex("#009337", AlphaPosition::None).unwrap(),
            "kelley green",
        ),
        (
            Color::from_hex("#d0e429", AlphaPosition::None).unwrap(),
            "sickly yellow",
        ),
        (
            Color::from_hex("#fff917", AlphaPosition::None).unwrap(),
            "sunny yellow",
        ),
        (
            Color::from_hex("#1d5dec", AlphaPosition::None).unwrap(),
            "azul",
        ),
        (
            Color::from_hex("#054907", AlphaPosition::None).unwrap(),
            "darkgreen",
        ),
        (
            Color::from_hex("#b5ce08", AlphaPosition::None).unwrap(),
            "green/yellow",
        ),
        (
            Color::from_hex("#8fb67b", AlphaPosition::None).unwrap(),
            "lichen",
        ),
        (
            Color::from_hex("#c8ffb0", AlphaPosition::None).unwrap(),
            "light light green",
        ),
        (
            Color::from_hex("#fdde6c", AlphaPosition::None).unwrap(),
            "pale gold",
        ),
        (
            Color::from_hex("#ffdf22", AlphaPosition::None).unwrap(),
            "sun yellow",
        ),
        (
            Color::from_hex("#a9be70", AlphaPosition::None).unwrap(),
            "tan green",
        ),
        (
            Color::from_hex("#6832e3", AlphaPosition::None).unwrap(),
            "burple",
        ),
        (
            Color::from_hex("#fdb147", AlphaPosition::None).unwrap(),
            "butterscotch",
        ),
        (
            Color::from_hex("#c7ac7d", AlphaPosition::None).unwrap(),
            "toupe",
        ),
        (
            Color::from_hex("#fff39a", AlphaPosition::None).unwrap(),
            "dark cream",
        ),
        (
            Color::from_hex("#850e04", AlphaPosition::None).unwrap(),
            "indian red",
        ),
        (
            Color::from_hex("#efc0fe", AlphaPosition::None).unwrap(),
            "light lavendar",
        ),
        (
            Color::from_hex("#40fd14", AlphaPosition::None).unwrap(),
            "poison green",
        ),
        (
            Color::from_hex("#b6c406", AlphaPosition::None).unwrap(),
            "baby puke green",
        ),
        (
            Color::from_hex("#9dff00", AlphaPosition::None).unwrap(),
            "bright yellow green",
        ),
        (
            Color::from_hex("#3c4142", AlphaPosition::None).unwrap(),
            "charcoal grey",
        ),
        (
            Color::from_hex("#f2ab15", AlphaPosition::None).unwrap(),
            "squash",
        ),
        (
            Color::from_hex("#ac4f06", AlphaPosition::None).unwrap(),
            "cinnamon",
        ),
        (
            Color::from_hex("#c4fe82", AlphaPosition::None).unwrap(),
            "light pea green",
        ),
        (
            Color::from_hex("#2cfa1f", AlphaPosition::None).unwrap(),
            "radioactive green",
        ),
        (
            Color::from_hex("#9a6200", AlphaPosition::None).unwrap(),
            "raw sienna",
        ),
        (
            Color::from_hex("#ca9bf7", AlphaPosition::None).unwrap(),
            "baby purple",
        ),
        (
            Color::from_hex("#875f42", AlphaPosition::None).unwrap(),
            "cocoa",
        ),
        (
            Color::from_hex("#3a2efe", AlphaPosition::None).unwrap(),
            "light royal blue",
        ),
        (
            Color::from_hex("#fd8d49", AlphaPosition::None).unwrap(),
            "orangeish",
        ),
        (
            Color::from_hex("#8b3103", AlphaPosition::None).unwrap(),
            "rust brown",
        ),
        (
            Color::from_hex("#cba560", AlphaPosition::None).unwrap(),
            "sand brown",
        ),
        (
            Color::from_hex("#698339", AlphaPosition::None).unwrap(),
            "swamp",
        ),
        (
            Color::from_hex("#0cdc73", AlphaPosition::None).unwrap(),
            "tealish green",
        ),
        (
            Color::from_hex("#b75203", AlphaPosition::None).unwrap(),
            "burnt siena",
        ),
        (
            Color::from_hex("#7f8f4e", AlphaPosition::None).unwrap(),
            "camo",
        ),
        (
            Color::from_hex("#26538d", AlphaPosition::None).unwrap(),
            "dusk blue",
        ),
        (
            Color::from_hex("#63a950", AlphaPosition::None).unwrap(),
            "fern",
        ),
        (
            Color::from_hex("#c87f89", AlphaPosition::None).unwrap(),
            "old rose",
        ),
        (
            Color::from_hex("#b1fc99", AlphaPosition::None).unwrap(),
            "pale light green",
        ),
        (
            Color::from_hex("#ff9a8a", AlphaPosition::None).unwrap(),
            "peachy pink",
        ),
        (
            Color::from_hex("#f6688e", AlphaPosition::None).unwrap(),
            "rosy pink",
        ),
        (
            Color::from_hex("#76fda8", AlphaPosition::None).unwrap(),
            "light bluish green",
        ),
        (
            Color::from_hex("#53fe5c", AlphaPosition::None).unwrap(),
            "light bright green",
        ),
        (
            Color::from_hex("#4efd54", AlphaPosition::None).unwrap(),
            "light neon green",
        ),
        (
            Color::from_hex("#a0febf", AlphaPosition::None).unwrap(),
            "light seafoam",
        ),
        (
            Color::from_hex("#7bf2da", AlphaPosition::None).unwrap(),
            "tiffany blue",
        ),
        (
            Color::from_hex("#bcf5a6", AlphaPosition::None).unwrap(),
            "washed out green",
        ),
        (
            Color::from_hex("#ca6b02", AlphaPosition::None).unwrap(),
            "browny orange",
        ),
        (
            Color::from_hex("#107ab0", AlphaPosition::None).unwrap(),
            "nice blue",
        ),
        (
            Color::from_hex("#2138ab", AlphaPosition::None).unwrap(),
            "sapphire",
        ),
        (
            Color::from_hex("#719f91", AlphaPosition::None).unwrap(),
            "greyish teal",
        ),
        (
            Color::from_hex("#fdb915", AlphaPosition::None).unwrap(),
            "orangey yellow",
        ),
        (
            Color::from_hex("#fefcaf", AlphaPosition::None).unwrap(),
            "parchment",
        ),
        (
            Color::from_hex("#fcf679", AlphaPosition::None).unwrap(),
            "straw",
        ),
        (
            Color::from_hex("#1d0200", AlphaPosition::None).unwrap(),
            "very dark brown",
        ),
        (
            Color::from_hex("#cb6843", AlphaPosition::None).unwrap(),
            "terracota",
        ),
        (
            Color::from_hex("#31668a", AlphaPosition::None).unwrap(),
            "ugly blue",
        ),
        (
            Color::from_hex("#247afd", AlphaPosition::None).unwrap(),
            "clear blue",
        ),
        (
            Color::from_hex("#ffffb6", AlphaPosition::None).unwrap(),
            "creme",
        ),
        (
            Color::from_hex("#90fda9", AlphaPosition::None).unwrap(),
            "foam green",
        ),
        (
            Color::from_hex("#86a17d", AlphaPosition::None).unwrap(),
            "grey/green",
        ),
        (
            Color::from_hex("#fddc5c", AlphaPosition::None).unwrap(),
            "light gold",
        ),
        (
            Color::from_hex("#78d1b6", AlphaPosition::None).unwrap(),
            "seafoam blue",
        ),
        (
            Color::from_hex("#13bbaf", AlphaPosition::None).unwrap(),
            "topaz",
        ),
        (
            Color::from_hex("#fb5ffc", AlphaPosition::None).unwrap(),
            "violet pink",
        ),
        (
            Color::from_hex("#20f986", AlphaPosition::None).unwrap(),
            "wintergreen",
        ),
        (
            Color::from_hex("#ffe36e", AlphaPosition::None).unwrap(),
            "yellow tan",
        ),
        (
            Color::from_hex("#9d0759", AlphaPosition::None).unwrap(),
            "dark fuchsia",
        ),
        (
            Color::from_hex("#3a18b1", AlphaPosition::None).unwrap(),
            "indigo blue",
        ),
        (
            Color::from_hex("#c2ff89", AlphaPosition::None).unwrap(),
            "light yellowish green",
        ),
        (
            Color::from_hex("#d767ad", AlphaPosition::None).unwrap(),
            "pale magenta",
        ),
        (
            Color::from_hex("#720058", AlphaPosition::None).unwrap(),
            "rich purple",
        ),
        (
            Color::from_hex("#ffda03", AlphaPosition::None).unwrap(),
            "sunflower yellow",
        ),
        (
            Color::from_hex("#01c08d", AlphaPosition::None).unwrap(),
            "green/blue",
        ),
        (
            Color::from_hex("#ac7434", AlphaPosition::None).unwrap(),
            "leather",
        ),
        (
            Color::from_hex("#014600", AlphaPosition::None).unwrap(),
            "racing green",
        ),
        (
            Color::from_hex("#9900fa", AlphaPosition::None).unwrap(),
            "vivid purple",
        ),
        (
            Color::from_hex("#02066f", AlphaPosition::None).unwrap(),
            "dark royal blue",
        ),
        (
            Color::from_hex("#8e7618", AlphaPosition::None).unwrap(),
            "hazel",
        ),
        (
            Color::from_hex("#d1768f", AlphaPosition::None).unwrap(),
            "muted pink",
        ),
        (
            Color::from_hex("#96b403", AlphaPosition::None).unwrap(),
            "booger green",
        ),
        (
            Color::from_hex("#fdff63", AlphaPosition::None).unwrap(),
            "canary",
        ),
        (
            Color::from_hex("#95a3a6", AlphaPosition::None).unwrap(),
            "cool grey",
        ),
        (
            Color::from_hex("#7f684e", AlphaPosition::None).unwrap(),
            "dark taupe",
        ),
        (
            Color::from_hex("#751973", AlphaPosition::None).unwrap(),
            "darkish purple",
        ),
        (
            Color::from_hex("#089404", AlphaPosition::None).unwrap(),
            "true green",
        ),
        (
            Color::from_hex("#ff6163", AlphaPosition::None).unwrap(),
            "coral pink",
        ),
        (
            Color::from_hex("#598556", AlphaPosition::None).unwrap(),
            "dark sage",
        ),
        (
            Color::from_hex("#214761", AlphaPosition::None).unwrap(),
            "dark slate blue",
        ),
        (
            Color::from_hex("#3c73a8", AlphaPosition::None).unwrap(),
            "flat blue",
        ),
        (
            Color::from_hex("#ba9e88", AlphaPosition::None).unwrap(),
            "mushroom",
        ),
        (
            Color::from_hex("#021bf9", AlphaPosition::None).unwrap(),
            "rich blue",
        ),
        (
            Color::from_hex("#734a65", AlphaPosition::None).unwrap(),
            "dirty purple",
        ),
        (
            Color::from_hex("#23c48b", AlphaPosition::None).unwrap(),
            "greenblue",
        ),
        (
            Color::from_hex("#8fae22", AlphaPosition::None).unwrap(),
            "icky green",
        ),
        (
            Color::from_hex("#e6f2a2", AlphaPosition::None).unwrap(),
            "light khaki",
        ),
        (
            Color::from_hex("#4b57db", AlphaPosition::None).unwrap(),
            "warm blue",
        ),
        (
            Color::from_hex("#d90166", AlphaPosition::None).unwrap(),
            "dark hot pink",
        ),
        (
            Color::from_hex("#015482", AlphaPosition::None).unwrap(),
            "deep sea blue",
        ),
        (
            Color::from_hex("#9d0216", AlphaPosition::None).unwrap(),
            "carmine",
        ),
        (
            Color::from_hex("#728f02", AlphaPosition::None).unwrap(),
            "dark yellow green",
        ),
        (
            Color::from_hex("#ffe5ad", AlphaPosition::None).unwrap(),
            "pale peach",
        ),
        (
            Color::from_hex("#4e0550", AlphaPosition::None).unwrap(),
            "plum purple",
        ),
        (
            Color::from_hex("#f9bc08", AlphaPosition::None).unwrap(),
            "golden rod",
        ),
        (
            Color::from_hex("#ff073a", AlphaPosition::None).unwrap(),
            "neon red",
        ),
        (
            Color::from_hex("#c77986", AlphaPosition::None).unwrap(),
            "old pink",
        ),
        (
            Color::from_hex("#d6fffe", AlphaPosition::None).unwrap(),
            "very pale blue",
        ),
        (
            Color::from_hex("#fe4b03", AlphaPosition::None).unwrap(),
            "blood orange",
        ),
        (
            Color::from_hex("#fd5956", AlphaPosition::None).unwrap(),
            "grapefruit",
        ),
        (
            Color::from_hex("#fce166", AlphaPosition::None).unwrap(),
            "sand yellow",
        ),
        (
            Color::from_hex("#b2713d", AlphaPosition::None).unwrap(),
            "clay brown",
        ),
        (
            Color::from_hex("#1f3b4d", AlphaPosition::None).unwrap(),
            "dark blue grey",
        ),
        (
            Color::from_hex("#699d4c", AlphaPosition::None).unwrap(),
            "flat green",
        ),
        (
            Color::from_hex("#56fca2", AlphaPosition::None).unwrap(),
            "light green blue",
        ),
        (
            Color::from_hex("#fb5581", AlphaPosition::None).unwrap(),
            "warm pink",
        ),
        (
            Color::from_hex("#3e82fc", AlphaPosition::None).unwrap(),
            "dodger blue",
        ),
        (
            Color::from_hex("#a0bf16", AlphaPosition::None).unwrap(),
            "gross green",
        ),
        (
            Color::from_hex("#d6fffa", AlphaPosition::None).unwrap(),
            "ice",
        ),
        (
            Color::from_hex("#4f738e", AlphaPosition::None).unwrap(),
            "metallic blue",
        ),
        (
            Color::from_hex("#ffb19a", AlphaPosition::None).unwrap(),
            "pale salmon",
        ),
        (
            Color::from_hex("#5c8b15", AlphaPosition::None).unwrap(),
            "sap green",
        ),
        (
            Color::from_hex("#54ac68", AlphaPosition::None).unwrap(),
            "algae",
        ),
        (
            Color::from_hex("#89a0b0", AlphaPosition::None).unwrap(),
            "bluey grey",
        ),
        (
            Color::from_hex("#7ea07a", AlphaPosition::None).unwrap(),
            "greeny grey",
        ),
        (
            Color::from_hex("#1bfc06", AlphaPosition::None).unwrap(),
            "highlighter green",
        ),
        (
            Color::from_hex("#cafffb", AlphaPosition::None).unwrap(),
            "light light blue",
        ),
        (
            Color::from_hex("#b6ffbb", AlphaPosition::None).unwrap(),
            "light mint",
        ),
        (
            Color::from_hex("#a75e09", AlphaPosition::None).unwrap(),
            "raw umber",
        ),
        (
            Color::from_hex("#152eff", AlphaPosition::None).unwrap(),
            "vivid blue",
        ),
        (
            Color::from_hex("#8d5eb7", AlphaPosition::None).unwrap(),
            "deep lavender",
        ),
        (
            Color::from_hex("#5f9e8f", AlphaPosition::None).unwrap(),
            "dull teal",
        ),
        (
            Color::from_hex("#63f7b4", AlphaPosition::None).unwrap(),
            "light greenish blue",
        ),
        (
            Color::from_hex("#606602", AlphaPosition::None).unwrap(),
            "mud green",
        ),
        (
            Color::from_hex("#fc86aa", AlphaPosition::None).unwrap(),
            "pinky",
        ),
        (
            Color::from_hex("#8c0034", AlphaPosition::None).unwrap(),
            "red wine",
        ),
        (
            Color::from_hex("#758000", AlphaPosition::None).unwrap(),
            "shit green",
        ),
        (
            Color::from_hex("#ab7e4c", AlphaPosition::None).unwrap(),
            "tan brown",
        ),
        (
            Color::from_hex("#030764", AlphaPosition::None).unwrap(),
            "darkblue",
        ),
        (
            Color::from_hex("#fe86a4", AlphaPosition::None).unwrap(),
            "rosa",
        ),
        (
            Color::from_hex("#d5174e", AlphaPosition::None).unwrap(),
            "lipstick",
        ),
        (
            Color::from_hex("#fed0fc", AlphaPosition::None).unwrap(),
            "pale mauve",
        ),
        (
            Color::from_hex("#680018", AlphaPosition::None).unwrap(),
            "claret",
        ),
        (
            Color::from_hex("#fedf08", AlphaPosition::None).unwrap(),
            "dandelion",
        ),
        (
            Color::from_hex("#fe420f", AlphaPosition::None).unwrap(),
            "orangered",
        ),
        (
            Color::from_hex("#6f7c00", AlphaPosition::None).unwrap(),
            "poop green",
        ),
        (
            Color::from_hex("#ca0147", AlphaPosition::None).unwrap(),
            "ruby",
        ),
        (
            Color::from_hex("#1b2431", AlphaPosition::None).unwrap(),
            "dark",
        ),
        (
            Color::from_hex("#00fbb0", AlphaPosition::None).unwrap(),
            "greenish turquoise",
        ),
        (
            Color::from_hex("#db5856", AlphaPosition::None).unwrap(),
            "pastel red",
        ),
        (
            Color::from_hex("#ddd618", AlphaPosition::None).unwrap(),
            "piss yellow",
        ),
        (
            Color::from_hex("#41fdfe", AlphaPosition::None).unwrap(),
            "bright cyan",
        ),
        (
            Color::from_hex("#cf524e", AlphaPosition::None).unwrap(),
            "dark coral",
        ),
        (
            Color::from_hex("#21c36f", AlphaPosition::None).unwrap(),
            "algae green",
        ),
        (
            Color::from_hex("#a90308", AlphaPosition::None).unwrap(),
            "darkish red",
        ),
        (
            Color::from_hex("#6e1005", AlphaPosition::None).unwrap(),
            "reddy brown",
        ),
        (
            Color::from_hex("#fe828c", AlphaPosition::None).unwrap(),
            "blush pink",
        ),
        (
            Color::from_hex("#4b6113", AlphaPosition::None).unwrap(),
            "camouflage green",
        ),
        (
            Color::from_hex("#4da409", AlphaPosition::None).unwrap(),
            "lawn green",
        ),
        (
            Color::from_hex("#beae8a", AlphaPosition::None).unwrap(),
            "putty",
        ),
        (
            Color::from_hex("#0339f8", AlphaPosition::None).unwrap(),
            "vibrant blue",
        ),
        (
            Color::from_hex("#a88f59", AlphaPosition::None).unwrap(),
            "dark sand",
        ),
        (
            Color::from_hex("#5d21d0", AlphaPosition::None).unwrap(),
            "purple/blue",
        ),
        (
            Color::from_hex("#feb209", AlphaPosition::None).unwrap(),
            "saffron",
        ),
        (
            Color::from_hex("#4e518b", AlphaPosition::None).unwrap(),
            "twilight",
        ),
        (
            Color::from_hex("#964e02", AlphaPosition::None).unwrap(),
            "warm brown",
        ),
        (
            Color::from_hex("#85a3b2", AlphaPosition::None).unwrap(),
            "bluegrey",
        ),
        (
            Color::from_hex("#ff69af", AlphaPosition::None).unwrap(),
            "bubble gum pink",
        ),
        (
            Color::from_hex("#c3fbf4", AlphaPosition::None).unwrap(),
            "duck egg blue",
        ),
        (
            Color::from_hex("#2afeb7", AlphaPosition::None).unwrap(),
            "greenish cyan",
        ),
        (
            Color::from_hex("#005f6a", AlphaPosition::None).unwrap(),
            "petrol",
        ),
        (
            Color::from_hex("#0c1793", AlphaPosition::None).unwrap(),
            "royal",
        ),
        (
            Color::from_hex("#ffff81", AlphaPosition::None).unwrap(),
            "butter",
        ),
        (
            Color::from_hex("#f0833a", AlphaPosition::None).unwrap(),
            "dusty orange",
        ),
        (
            Color::from_hex("#f1f33f", AlphaPosition::None).unwrap(),
            "off yellow",
        ),
        (
            Color::from_hex("#b1d27b", AlphaPosition::None).unwrap(),
            "pale olive green",
        ),
        (
            Color::from_hex("#fc824a", AlphaPosition::None).unwrap(),
            "orangish",
        ),
        (
            Color::from_hex("#71aa34", AlphaPosition::None).unwrap(),
            "leaf",
        ),
        (
            Color::from_hex("#b7c9e2", AlphaPosition::None).unwrap(),
            "light blue grey",
        ),
        (
            Color::from_hex("#4b0101", AlphaPosition::None).unwrap(),
            "dried blood",
        ),
        (
            Color::from_hex("#a552e6", AlphaPosition::None).unwrap(),
            "lightish purple",
        ),
        (
            Color::from_hex("#af2f0d", AlphaPosition::None).unwrap(),
            "rusty red",
        ),
        (
            Color::from_hex("#8b88f8", AlphaPosition::None).unwrap(),
            "lavender blue",
        ),
        (
            Color::from_hex("#9af764", AlphaPosition::None).unwrap(),
            "light grass green",
        ),
        (
            Color::from_hex("#a6fbb2", AlphaPosition::None).unwrap(),
            "light mint green",
        ),
        (
            Color::from_hex("#ffc512", AlphaPosition::None).unwrap(),
            "sunflower",
        ),
        (
            Color::from_hex("#750851", AlphaPosition::None).unwrap(),
            "velvet",
        ),
        (
            Color::from_hex("#c14a09", AlphaPosition::None).unwrap(),
            "brick orange",
        ),
        (
            Color::from_hex("#fe2f4a", AlphaPosition::None).unwrap(),
            "lightish red",
        ),
        (
            Color::from_hex("#0203e2", AlphaPosition::None).unwrap(),
            "pure blue",
        ),
        (
            Color::from_hex("#0a437a", AlphaPosition::None).unwrap(),
            "twilight blue",
        ),
        (
            Color::from_hex("#a50055", AlphaPosition::None).unwrap(),
            "violet red",
        ),
        (
            Color::from_hex("#ae8b0c", AlphaPosition::None).unwrap(),
            "yellowy brown",
        ),
        (
            Color::from_hex("#fd798f", AlphaPosition::None).unwrap(),
            "carnation",
        ),
        (
            Color::from_hex("#bfac05", AlphaPosition::None).unwrap(),
            "muddy yellow",
        ),
        (
            Color::from_hex("#3eaf76", AlphaPosition::None).unwrap(),
            "dark seafoam green",
        ),
        (
            Color::from_hex("#c74767", AlphaPosition::None).unwrap(),
            "deep rose",
        ),
        (
            Color::from_hex("#b9484e", AlphaPosition::None).unwrap(),
            "dusty red",
        ),
        (
            Color::from_hex("#647d8e", AlphaPosition::None).unwrap(),
            "grey/blue",
        ),
        (
            Color::from_hex("#bffe28", AlphaPosition::None).unwrap(),
            "lemon lime",
        ),
        (
            Color::from_hex("#d725de", AlphaPosition::None).unwrap(),
            "purple/pink",
        ),
        (
            Color::from_hex("#b29705", AlphaPosition::None).unwrap(),
            "brown yellow",
        ),
        (
            Color::from_hex("#673a3f", AlphaPosition::None).unwrap(),
            "purple brown",
        ),
        (
            Color::from_hex("#a87dc2", AlphaPosition::None).unwrap(),
            "wisteria",
        ),
        (
            Color::from_hex("#fafe4b", AlphaPosition::None).unwrap(),
            "banana yellow",
        ),
        (
            Color::from_hex("#c0022f", AlphaPosition::None).unwrap(),
            "lipstick red",
        ),
        (
            Color::from_hex("#0e87cc", AlphaPosition::None).unwrap(),
            "water blue",
        ),
        (
            Color::from_hex("#8d8468", AlphaPosition::None).unwrap(),
            "brown grey",
        ),
        (
            Color::from_hex("#ad03de", AlphaPosition::None).unwrap(),
            "vibrant purple",
        ),
        (
            Color::from_hex("#8cff9e", AlphaPosition::None).unwrap(),
            "baby green",
        ),
        (
            Color::from_hex("#94ac02", AlphaPosition::None).unwrap(),
            "barf green",
        ),
        (
            Color::from_hex("#c4fff7", AlphaPosition::None).unwrap(),
            "eggshell blue",
        ),
        (
            Color::from_hex("#fdee73", AlphaPosition::None).unwrap(),
            "sandy yellow",
        ),
        (
            Color::from_hex("#33b864", AlphaPosition::None).unwrap(),
            "cool green",
        ),
        (
            Color::from_hex("#fff9d0", AlphaPosition::None).unwrap(),
            "pale",
        ),
        (
            Color::from_hex("#758da3", AlphaPosition::None).unwrap(),
            "blue/grey",
        ),
        (
            Color::from_hex("#f504c9", AlphaPosition::None).unwrap(),
            "hot magenta",
        ),
        (
            Color::from_hex("#77a1b5", AlphaPosition::None).unwrap(),
            "greyblue",
        ),
        (
            Color::from_hex("#8756e4", AlphaPosition::None).unwrap(),
            "purpley",
        ),
        (
            Color::from_hex("#889717", AlphaPosition::None).unwrap(),
            "baby shit green",
        ),
        (
            Color::from_hex("#c27e79", AlphaPosition::None).unwrap(),
            "brownish pink",
        ),
        (
            Color::from_hex("#017371", AlphaPosition::None).unwrap(),
            "dark aquamarine",
        ),
        (
            Color::from_hex("#9f8303", AlphaPosition::None).unwrap(),
            "diarrhea",
        ),
        (
            Color::from_hex("#f7d560", AlphaPosition::None).unwrap(),
            "light mustard",
        ),
        (
            Color::from_hex("#bdf6fe", AlphaPosition::None).unwrap(),
            "pale sky blue",
        ),
        (
            Color::from_hex("#75b84f", AlphaPosition::None).unwrap(),
            "turtle green",
        ),
        (
            Color::from_hex("#9cbb04", AlphaPosition::None).unwrap(),
            "bright olive",
        ),
        (
            Color::from_hex("#29465b", AlphaPosition::None).unwrap(),
            "dark grey blue",
        ),
        (
            Color::from_hex("#696006", AlphaPosition::None).unwrap(),
            "greeny brown",
        ),
        (
            Color::from_hex("#adf802", AlphaPosition::None).unwrap(),
            "lemon green",
        ),
        (
            Color::from_hex("#c1c6fc", AlphaPosition::None).unwrap(),
            "light periwinkle",
        ),
        (
            Color::from_hex("#35ad6b", AlphaPosition::None).unwrap(),
            "seaweed green",
        ),
        (
            Color::from_hex("#fffd37", AlphaPosition::None).unwrap(),
            "sunshine yellow",
        ),
        (
            Color::from_hex("#a442a0", AlphaPosition::None).unwrap(),
            "ugly purple",
        ),
        (
            Color::from_hex("#f36196", AlphaPosition::None).unwrap(),
            "medium pink",
        ),
        (
            Color::from_hex("#947706", AlphaPosition::None).unwrap(),
            "puke brown",
        ),
        (
            Color::from_hex("#fff4f2", AlphaPosition::None).unwrap(),
            "very light pink",
        ),
        (
            Color::from_hex("#1e9167", AlphaPosition::None).unwrap(),
            "viridian",
        ),
        (
            Color::from_hex("#b5c306", AlphaPosition::None).unwrap(),
            "bile",
        ),
        (
            Color::from_hex("#feff7f", AlphaPosition::None).unwrap(),
            "faded yellow",
        ),
        (
            Color::from_hex("#cffdbc", AlphaPosition::None).unwrap(),
            "very pale green",
        ),
        (
            Color::from_hex("#0add08", AlphaPosition::None).unwrap(),
            "vibrant green",
        ),
        (
            Color::from_hex("#87fd05", AlphaPosition::None).unwrap(),
            "bright lime",
        ),
        (
            Color::from_hex("#1ef876", AlphaPosition::None).unwrap(),
            "spearmint",
        ),
        (
            Color::from_hex("#7bfdc7", AlphaPosition::None).unwrap(),
            "light aquamarine",
        ),
        (
            Color::from_hex("#bcecac", AlphaPosition::None).unwrap(),
            "light sage",
        ),
        (
            Color::from_hex("#bbf90f", AlphaPosition::None).unwrap(),
            "yellowgreen",
        ),
        (
            Color::from_hex("#ab9004", AlphaPosition::None).unwrap(),
            "baby poo",
        ),
        (
            Color::from_hex("#1fb57a", AlphaPosition::None).unwrap(),
            "dark seafoam",
        ),
        (
            Color::from_hex("#00555a", AlphaPosition::None).unwrap(),
            "deep teal",
        ),
        (
            Color::from_hex("#a484ac", AlphaPosition::None).unwrap(),
            "heather",
        ),
        (
            Color::from_hex("#c45508", AlphaPosition::None).unwrap(),
            "rust orange",
        ),
        (
            Color::from_hex("#3f829d", AlphaPosition::None).unwrap(),
            "dirty blue",
        ),
        (
            Color::from_hex("#548d44", AlphaPosition::None).unwrap(),
            "fern green",
        ),
        (
            Color::from_hex("#c95efb", AlphaPosition::None).unwrap(),
            "bright lilac",
        ),
        (
            Color::from_hex("#3ae57f", AlphaPosition::None).unwrap(),
            "weird green",
        ),
        (
            Color::from_hex("#016795", AlphaPosition::None).unwrap(),
            "peacock blue",
        ),
        (
            Color::from_hex("#87a922", AlphaPosition::None).unwrap(),
            "avocado green",
        ),
        (
            Color::from_hex("#f0944d", AlphaPosition::None).unwrap(),
            "faded orange",
        ),
        (
            Color::from_hex("#5d1451", AlphaPosition::None).unwrap(),
            "grape purple",
        ),
        (
            Color::from_hex("#25ff29", AlphaPosition::None).unwrap(),
            "hot green",
        ),
        (
            Color::from_hex("#d0fe1d", AlphaPosition::None).unwrap(),
            "lime yellow",
        ),
        (
            Color::from_hex("#ffa62b", AlphaPosition::None).unwrap(),
            "mango",
        ),
        (
            Color::from_hex("#01b44c", AlphaPosition::None).unwrap(),
            "shamrock",
        ),
        (
            Color::from_hex("#ff6cb5", AlphaPosition::None).unwrap(),
            "bubblegum",
        ),
        (
            Color::from_hex("#6b4247", AlphaPosition::None).unwrap(),
            "purplish brown",
        ),
        (
            Color::from_hex("#c7c10c", AlphaPosition::None).unwrap(),
            "vomit yellow",
        ),
        (
            Color::from_hex("#b7fffa", AlphaPosition::None).unwrap(),
            "pale cyan",
        ),
        (
            Color::from_hex("#aeff6e", AlphaPosition::None).unwrap(),
            "key lime",
        ),
        (
            Color::from_hex("#ec2d01", AlphaPosition::None).unwrap(),
            "tomato red",
        ),
        (
            Color::from_hex("#76ff7b", AlphaPosition::None).unwrap(),
            "lightgreen",
        ),
        (
            Color::from_hex("#730039", AlphaPosition::None).unwrap(),
            "merlot",
        ),
        (
            Color::from_hex("#040348", AlphaPosition::None).unwrap(),
            "night blue",
        ),
        (
            Color::from_hex("#df4ec8", AlphaPosition::None).unwrap(),
            "purpleish pink",
        ),
        (
            Color::from_hex("#6ecb3c", AlphaPosition::None).unwrap(),
            "apple",
        ),
        (
            Color::from_hex("#8f9805", AlphaPosition::None).unwrap(),
            "baby poop green",
        ),
        (
            Color::from_hex("#5edc1f", AlphaPosition::None).unwrap(),
            "green apple",
        ),
        (
            Color::from_hex("#d94ff5", AlphaPosition::None).unwrap(),
            "heliotrope",
        ),
        (
            Color::from_hex("#c8fd3d", AlphaPosition::None).unwrap(),
            "yellow/green",
        ),
        (
            Color::from_hex("#070d0d", AlphaPosition::None).unwrap(),
            "almost black",
        ),
        (
            Color::from_hex("#4984b8", AlphaPosition::None).unwrap(),
            "cool blue",
        ),
        (
            Color::from_hex("#51b73b", AlphaPosition::None).unwrap(),
            "leafy green",
        ),
        (
            Color::from_hex("#ac7e04", AlphaPosition::None).unwrap(),
            "mustard brown",
        ),
        (
            Color::from_hex("#4e5481", AlphaPosition::None).unwrap(),
            "dusk",
        ),
        (
            Color::from_hex("#876e4b", AlphaPosition::None).unwrap(),
            "dull brown",
        ),
        (
            Color::from_hex("#58bc08", AlphaPosition::None).unwrap(),
            "frog green",
        ),
        (
            Color::from_hex("#2fef10", AlphaPosition::None).unwrap(),
            "vivid green",
        ),
        (
            Color::from_hex("#2dfe54", AlphaPosition::None).unwrap(),
            "bright light green",
        ),
        (
            Color::from_hex("#0aff02", AlphaPosition::None).unwrap(),
            "fluro green",
        ),
        (
            Color::from_hex("#9cef43", AlphaPosition::None).unwrap(),
            "kiwi",
        ),
        (
            Color::from_hex("#18d17b", AlphaPosition::None).unwrap(),
            "seaweed",
        ),
        (
            Color::from_hex("#35530a", AlphaPosition::None).unwrap(),
            "navy green",
        ),
        (
            Color::from_hex("#1805db", AlphaPosition::None).unwrap(),
            "ultramarine blue",
        ),
        (
            Color::from_hex("#6258c4", AlphaPosition::None).unwrap(),
            "iris",
        ),
        (
            Color::from_hex("#ff964f", AlphaPosition::None).unwrap(),
            "pastel orange",
        ),
        (
            Color::from_hex("#ffab0f", AlphaPosition::None).unwrap(),
            "yellowish orange",
        ),
        (
            Color::from_hex("#8f8ce7", AlphaPosition::None).unwrap(),
            "perrywinkle",
        ),
        (
            Color::from_hex("#24bca8", AlphaPosition::None).unwrap(),
            "tealish",
        ),
        (
            Color::from_hex("#3f012c", AlphaPosition::None).unwrap(),
            "dark plum",
        ),
        (
            Color::from_hex("#cbf85f", AlphaPosition::None).unwrap(),
            "pear",
        ),
        (
            Color::from_hex("#ff724c", AlphaPosition::None).unwrap(),
            "pinkish orange",
        ),
        (
            Color::from_hex("#280137", AlphaPosition::None).unwrap(),
            "midnight purple",
        ),
        (
            Color::from_hex("#b36ff6", AlphaPosition::None).unwrap(),
            "light urple",
        ),
        (
            Color::from_hex("#48c072", AlphaPosition::None).unwrap(),
            "dark mint",
        ),
        (
            Color::from_hex("#bccb7a", AlphaPosition::None).unwrap(),
            "greenish tan",
        ),
        (
            Color::from_hex("#a8415b", AlphaPosition::None).unwrap(),
            "light burgundy",
        ),
        (
            Color::from_hex("#06b1c4", AlphaPosition::None).unwrap(),
            "turquoise blue",
        ),
        (
            Color::from_hex("#cd7584", AlphaPosition::None).unwrap(),
            "ugly pink",
        ),
        (
            Color::from_hex("#f1da7a", AlphaPosition::None).unwrap(),
            "sandy",
        ),
        (
            Color::from_hex("#ff0490", AlphaPosition::None).unwrap(),
            "electric pink",
        ),
        (
            Color::from_hex("#805b87", AlphaPosition::None).unwrap(),
            "muted purple",
        ),
        (
            Color::from_hex("#50a747", AlphaPosition::None).unwrap(),
            "mid green",
        ),
        (
            Color::from_hex("#a8a495", AlphaPosition::None).unwrap(),
            "greyish",
        ),
        (
            Color::from_hex("#cfff04", AlphaPosition::None).unwrap(),
            "neon yellow",
        ),
        (
            Color::from_hex("#ffff7e", AlphaPosition::None).unwrap(),
            "banana",
        ),
        (
            Color::from_hex("#ff7fa7", AlphaPosition::None).unwrap(),
            "carnation pink",
        ),
        (
            Color::from_hex("#ef4026", AlphaPosition::None).unwrap(),
            "tomato",
        ),
        (
            Color::from_hex("#3c9992", AlphaPosition::None).unwrap(),
            "sea",
        ),
        (
            Color::from_hex("#886806", AlphaPosition::None).unwrap(),
            "muddy brown",
        ),
        (
            Color::from_hex("#04f489", AlphaPosition::None).unwrap(),
            "turquoise green",
        ),
        (
            Color::from_hex("#fef69e", AlphaPosition::None).unwrap(),
            "buff",
        ),
        (
            Color::from_hex("#cfaf7b", AlphaPosition::None).unwrap(),
            "fawn",
        ),
        (
            Color::from_hex("#3b719f", AlphaPosition::None).unwrap(),
            "muted blue",
        ),
        (
            Color::from_hex("#fdc1c5", AlphaPosition::None).unwrap(),
            "pale rose",
        ),
        (
            Color::from_hex("#20c073", AlphaPosition::None).unwrap(),
            "dark mint green",
        ),
        (
            Color::from_hex("#9b5fc0", AlphaPosition::None).unwrap(),
            "amethyst",
        ),
        (
            Color::from_hex("#0f9b8e", AlphaPosition::None).unwrap(),
            "blue/green",
        ),
        (
            Color::from_hex("#742802", AlphaPosition::None).unwrap(),
            "chestnut",
        ),
        (
            Color::from_hex("#9db92c", AlphaPosition::None).unwrap(),
            "sick green",
        ),
        (
            Color::from_hex("#a4bf20", AlphaPosition::None).unwrap(),
            "pea",
        ),
        (
            Color::from_hex("#cd5909", AlphaPosition::None).unwrap(),
            "rusty orange",
        ),
        (
            Color::from_hex("#ada587", AlphaPosition::None).unwrap(),
            "stone",
        ),
        (
            Color::from_hex("#be013c", AlphaPosition::None).unwrap(),
            "rose red",
        ),
        (
            Color::from_hex("#b8ffeb", AlphaPosition::None).unwrap(),
            "pale aqua",
        ),
        (
            Color::from_hex("#dc4d01", AlphaPosition::None).unwrap(),
            "deep orange",
        ),
        (
            Color::from_hex("#a2653e", AlphaPosition::None).unwrap(),
            "earth",
        ),
        (
            Color::from_hex("#638b27", AlphaPosition::None).unwrap(),
            "mossy green",
        ),
        (
            Color::from_hex("#419c03", AlphaPosition::None).unwrap(),
            "grassy green",
        ),
        (
            Color::from_hex("#b1ff65", AlphaPosition::None).unwrap(),
            "pale lime green",
        ),
        (
            Color::from_hex("#9dbcd4", AlphaPosition::None).unwrap(),
            "light grey blue",
        ),
        (
            Color::from_hex("#fdfdfe", AlphaPosition::None).unwrap(),
            "pale grey",
        ),
        (
            Color::from_hex("#77ab56", AlphaPosition::None).unwrap(),
            "asparagus",
        ),
        (
            Color::from_hex("#464196", AlphaPosition::None).unwrap(),
            "blueberry",
        ),
        (
            Color::from_hex("#990147", AlphaPosition::None).unwrap(),
            "purple red",
        ),
        (
            Color::from_hex("#befd73", AlphaPosition::None).unwrap(),
            "pale lime",
        ),
        (
            Color::from_hex("#32bf84", AlphaPosition::None).unwrap(),
            "greenish teal",
        ),
        (
            Color::from_hex("#af6f09", AlphaPosition::None).unwrap(),
            "caramel",
        ),
        (
            Color::from_hex("#a0025c", AlphaPosition::None).unwrap(),
            "deep magenta",
        ),
        (
            Color::from_hex("#ffd8b1", AlphaPosition::None).unwrap(),
            "light peach",
        ),
        (
            Color::from_hex("#7f4e1e", AlphaPosition::None).unwrap(),
            "milk chocolate",
        ),
        (
            Color::from_hex("#bf9b0c", AlphaPosition::None).unwrap(),
            "ocher",
        ),
        (
            Color::from_hex("#6ba353", AlphaPosition::None).unwrap(),
            "off green",
        ),
        (
            Color::from_hex("#f075e6", AlphaPosition::None).unwrap(),
            "purply pink",
        ),
        (
            Color::from_hex("#7bc8f6", AlphaPosition::None).unwrap(),
            "lightblue",
        ),
        (
            Color::from_hex("#475f94", AlphaPosition::None).unwrap(),
            "dusky blue",
        ),
        (
            Color::from_hex("#f5bf03", AlphaPosition::None).unwrap(),
            "golden",
        ),
        (
            Color::from_hex("#fffeb6", AlphaPosition::None).unwrap(),
            "light beige",
        ),
        (
            Color::from_hex("#fffd74", AlphaPosition::None).unwrap(),
            "butter yellow",
        ),
        (
            Color::from_hex("#895b7b", AlphaPosition::None).unwrap(),
            "dusky purple",
        ),
        (
            Color::from_hex("#436bad", AlphaPosition::None).unwrap(),
            "french blue",
        ),
        (
            Color::from_hex("#d0c101", AlphaPosition::None).unwrap(),
            "ugly yellow",
        ),
        (
            Color::from_hex("#c6f808", AlphaPosition::None).unwrap(),
            "greeny yellow",
        ),
        (
            Color::from_hex("#f43605", AlphaPosition::None).unwrap(),
            "orangish red",
        ),
        (
            Color::from_hex("#02c14d", AlphaPosition::None).unwrap(),
            "shamrock green",
        ),
        (
            Color::from_hex("#b25f03", AlphaPosition::None).unwrap(),
            "orangish brown",
        ),
        (
            Color::from_hex("#2a7e19", AlphaPosition::None).unwrap(),
            "tree green",
        ),
        (
            Color::from_hex("#490648", AlphaPosition::None).unwrap(),
            "deep violet",
        ),
        (
            Color::from_hex("#536267", AlphaPosition::None).unwrap(),
            "gunmetal",
        ),
        (
            Color::from_hex("#5a06ef", AlphaPosition::None).unwrap(),
            "blue/purple",
        ),
        (
            Color::from_hex("#cf0234", AlphaPosition::None).unwrap(),
            "cherry",
        ),
        (
            Color::from_hex("#c4a661", AlphaPosition::None).unwrap(),
            "sandy brown",
        ),
        (
            Color::from_hex("#978a84", AlphaPosition::None).unwrap(),
            "warm grey",
        ),
        (
            Color::from_hex("#1f0954", AlphaPosition::None).unwrap(),
            "dark indigo",
        ),
        (
            Color::from_hex("#03012d", AlphaPosition::None).unwrap(),
            "midnight",
        ),
        (
            Color::from_hex("#2bb179", AlphaPosition::None).unwrap(),
            "bluey green",
        ),
        (
            Color::from_hex("#c3909b", AlphaPosition::None).unwrap(),
            "grey pink",
        ),
        (
            Color::from_hex("#a66fb5", AlphaPosition::None).unwrap(),
            "soft purple",
        ),
        (
            Color::from_hex("#770001", AlphaPosition::None).unwrap(),
            "blood",
        ),
        (
            Color::from_hex("#922b05", AlphaPosition::None).unwrap(),
            "brown red",
        ),
        (
            Color::from_hex("#7d7f7c", AlphaPosition::None).unwrap(),
            "medium grey",
        ),
        (
            Color::from_hex("#990f4b", AlphaPosition::None).unwrap(),
            "berry",
        ),
        (
            Color::from_hex("#8f7303", AlphaPosition::None).unwrap(),
            "poo",
        ),
        (
            Color::from_hex("#c83cb9", AlphaPosition::None).unwrap(),
            "purpley pink",
        ),
        (
            Color::from_hex("#fea993", AlphaPosition::None).unwrap(),
            "light salmon",
        ),
        (
            Color::from_hex("#acbb0d", AlphaPosition::None).unwrap(),
            "snot",
        ),
        (
            Color::from_hex("#c071fe", AlphaPosition::None).unwrap(),
            "easter purple",
        ),
        (
            Color::from_hex("#ccfd7f", AlphaPosition::None).unwrap(),
            "light yellow green",
        ),
        (
            Color::from_hex("#00022e", AlphaPosition::None).unwrap(),
            "dark navy blue",
        ),
        (
            Color::from_hex("#828344", AlphaPosition::None).unwrap(),
            "drab",
        ),
        (
            Color::from_hex("#ffc5cb", AlphaPosition::None).unwrap(),
            "light rose",
        ),
        (
            Color::from_hex("#ab1239", AlphaPosition::None).unwrap(),
            "rouge",
        ),
        (
            Color::from_hex("#b0054b", AlphaPosition::None).unwrap(),
            "purplish red",
        ),
        (
            Color::from_hex("#99cc04", AlphaPosition::None).unwrap(),
            "slime green",
        ),
        (
            Color::from_hex("#937c00", AlphaPosition::None).unwrap(),
            "baby poop",
        ),
        (
            Color::from_hex("#019529", AlphaPosition::None).unwrap(),
            "irish green",
        ),
        (
            Color::from_hex("#ef1de7", AlphaPosition::None).unwrap(),
            "pink/purple",
        ),
        (
            Color::from_hex("#000435", AlphaPosition::None).unwrap(),
            "dark navy",
        ),
        (
            Color::from_hex("#42b395", AlphaPosition::None).unwrap(),
            "greeny blue",
        ),
        (
            Color::from_hex("#9d5783", AlphaPosition::None).unwrap(),
            "light plum",
        ),
        (
            Color::from_hex("#c8aca9", AlphaPosition::None).unwrap(),
            "pinkish grey",
        ),
        (
            Color::from_hex("#c87606", AlphaPosition::None).unwrap(),
            "dirty orange",
        ),
        (
            Color::from_hex("#aa2704", AlphaPosition::None).unwrap(),
            "rust red",
        ),
        (
            Color::from_hex("#e4cbff", AlphaPosition::None).unwrap(),
            "pale lilac",
        ),
        (
            Color::from_hex("#fa4224", AlphaPosition::None).unwrap(),
            "orangey red",
        ),
        (
            Color::from_hex("#0804f9", AlphaPosition::None).unwrap(),
            "primary blue",
        ),
        (
            Color::from_hex("#5cb200", AlphaPosition::None).unwrap(),
            "kermit green",
        ),
        (
            Color::from_hex("#76424e", AlphaPosition::None).unwrap(),
            "brownish purple",
        ),
        (
            Color::from_hex("#6c7a0e", AlphaPosition::None).unwrap(),
            "murky green",
        ),
        (
            Color::from_hex("#fbdd7e", AlphaPosition::None).unwrap(),
            "wheat",
        ),
        (
            Color::from_hex("#2a0134", AlphaPosition::None).unwrap(),
            "very dark purple",
        ),
        (
            Color::from_hex("#044a05", AlphaPosition::None).unwrap(),
            "bottle green",
        ),
        (
            Color::from_hex("#fd4659", AlphaPosition::None).unwrap(),
            "watermelon",
        ),
        (
            Color::from_hex("#0d75f8", AlphaPosition::None).unwrap(),
            "deep sky blue",
        ),
        (
            Color::from_hex("#fe0002", AlphaPosition::None).unwrap(),
            "fire engine red",
        ),
        (
            Color::from_hex("#cb9d06", AlphaPosition::None).unwrap(),
            "yellow ochre",
        ),
        (
            Color::from_hex("#fb7d07", AlphaPosition::None).unwrap(),
            "pumpkin orange",
        ),
        (
            Color::from_hex("#b9cc81", AlphaPosition::None).unwrap(),
            "pale olive",
        ),
        (
            Color::from_hex("#edc8ff", AlphaPosition::None).unwrap(),
            "light lilac",
        ),
        (
            Color::from_hex("#61e160", AlphaPosition::None).unwrap(),
            "lightish green",
        ),
        (
            Color::from_hex("#8ab8fe", AlphaPosition::None).unwrap(),
            "carolina blue",
        ),
        (
            Color::from_hex("#920a4e", AlphaPosition::None).unwrap(),
            "mulberry",
        ),
        (
            Color::from_hex("#fe02a2", AlphaPosition::None).unwrap(),
            "shocking pink",
        ),
        (
            Color::from_hex("#9a3001", AlphaPosition::None).unwrap(),
            "auburn",
        ),
        (
            Color::from_hex("#65fe08", AlphaPosition::None).unwrap(),
            "bright lime green",
        ),
        (
            Color::from_hex("#befdb7", AlphaPosition::None).unwrap(),
            "celadon",
        ),
        (
            Color::from_hex("#b17261", AlphaPosition::None).unwrap(),
            "pinkish brown",
        ),
        (
            Color::from_hex("#885f01", AlphaPosition::None).unwrap(),
            "poo brown",
        ),
        (
            Color::from_hex("#02ccfe", AlphaPosition::None).unwrap(),
            "bright sky blue",
        ),
        (
            Color::from_hex("#c1fd95", AlphaPosition::None).unwrap(),
            "celery",
        ),
        (
            Color::from_hex("#836539", AlphaPosition::None).unwrap(),
            "dirt brown",
        ),
        (
            Color::from_hex("#fb2943", AlphaPosition::None).unwrap(),
            "strawberry",
        ),
        (
            Color::from_hex("#84b701", AlphaPosition::None).unwrap(),
            "dark lime",
        ),
        (
            Color::from_hex("#b66325", AlphaPosition::None).unwrap(),
            "copper",
        ),
        (
            Color::from_hex("#7f5112", AlphaPosition::None).unwrap(),
            "medium brown",
        ),
        (
            Color::from_hex("#5fa052", AlphaPosition::None).unwrap(),
            "muted green",
        ),
        (
            Color::from_hex("#6dedfd", AlphaPosition::None).unwrap(),
            "robin's egg",
        ),
        (
            Color::from_hex("#0bf9ea", AlphaPosition::None).unwrap(),
            "bright aqua",
        ),
        (
            Color::from_hex("#c760ff", AlphaPosition::None).unwrap(),
            "bright lavender",
        ),
        (
            Color::from_hex("#ffffcb", AlphaPosition::None).unwrap(),
            "ivory",
        ),
        (
            Color::from_hex("#f6cefc", AlphaPosition::None).unwrap(),
            "very light purple",
        ),
        (
            Color::from_hex("#155084", AlphaPosition::None).unwrap(),
            "light navy",
        ),
        (
            Color::from_hex("#f5054f", AlphaPosition::None).unwrap(),
            "pink red",
        ),
        (
            Color::from_hex("#645403", AlphaPosition::None).unwrap(),
            "olive brown",
        ),
        (
            Color::from_hex("#7a5901", AlphaPosition::None).unwrap(),
            "poop brown",
        ),
        (
            Color::from_hex("#a8b504", AlphaPosition::None).unwrap(),
            "mustard green",
        ),
        (
            Color::from_hex("#3d9973", AlphaPosition::None).unwrap(),
            "ocean green",
        ),
        (
            Color::from_hex("#000133", AlphaPosition::None).unwrap(),
            "very dark blue",
        ),
        (
            Color::from_hex("#76a973", AlphaPosition::None).unwrap(),
            "dusty green",
        ),
        (
            Color::from_hex("#2e5a88", AlphaPosition::None).unwrap(),
            "light navy blue",
        ),
        (
            Color::from_hex("#0bf77d", AlphaPosition::None).unwrap(),
            "minty green",
        ),
        (
            Color::from_hex("#bd6c48", AlphaPosition::None).unwrap(),
            "adobe",
        ),
        (
            Color::from_hex("#ac1db8", AlphaPosition::None).unwrap(),
            "barney",
        ),
        (
            Color::from_hex("#2baf6a", AlphaPosition::None).unwrap(),
            "jade green",
        ),
        (
            Color::from_hex("#26f7fd", AlphaPosition::None).unwrap(),
            "bright light blue",
        ),
        (
            Color::from_hex("#aefd6c", AlphaPosition::None).unwrap(),
            "light lime",
        ),
        (
            Color::from_hex("#9b8f55", AlphaPosition::None).unwrap(),
            "dark khaki",
        ),
        (
            Color::from_hex("#ffad01", AlphaPosition::None).unwrap(),
            "orange yellow",
        ),
        (
            Color::from_hex("#c69c04", AlphaPosition::None).unwrap(),
            "ocre",
        ),
        (
            Color::from_hex("#f4d054", AlphaPosition::None).unwrap(),
            "maize",
        ),
        (
            Color::from_hex("#de9dac", AlphaPosition::None).unwrap(),
            "faded pink",
        ),
        (
            Color::from_hex("#05480d", AlphaPosition::None).unwrap(),
            "british racing green",
        ),
        (
            Color::from_hex("#c9ae74", AlphaPosition::None).unwrap(),
            "sandstone",
        ),
        (
            Color::from_hex("#60460f", AlphaPosition::None).unwrap(),
            "mud brown",
        ),
        (
            Color::from_hex("#98f6b0", AlphaPosition::None).unwrap(),
            "light sea green",
        ),
        (
            Color::from_hex("#8af1fe", AlphaPosition::None).unwrap(),
            "robin egg blue",
        ),
        (
            Color::from_hex("#2ee8bb", AlphaPosition::None).unwrap(),
            "aqua marine",
        ),
        (
            Color::from_hex("#11875d", AlphaPosition::None).unwrap(),
            "dark sea green",
        ),
        (
            Color::from_hex("#fdb0c0", AlphaPosition::None).unwrap(),
            "soft pink",
        ),
        (
            Color::from_hex("#b16002", AlphaPosition::None).unwrap(),
            "orangey brown",
        ),
        (
            Color::from_hex("#f7022a", AlphaPosition::None).unwrap(),
            "cherry red",
        ),
        (
            Color::from_hex("#d5ab09", AlphaPosition::None).unwrap(),
            "burnt yellow",
        ),
        (
            Color::from_hex("#86775f", AlphaPosition::None).unwrap(),
            "brownish grey",
        ),
        (
            Color::from_hex("#c69f59", AlphaPosition::None).unwrap(),
            "camel",
        ),
        (
            Color::from_hex("#7a687f", AlphaPosition::None).unwrap(),
            "purplish grey",
        ),
        (
            Color::from_hex("#042e60", AlphaPosition::None).unwrap(),
            "marine",
        ),
        (
            Color::from_hex("#c88d94", AlphaPosition::None).unwrap(),
            "greyish pink",
        ),
        (
            Color::from_hex("#a5fbd5", AlphaPosition::None).unwrap(),
            "pale turquoise",
        ),
        (
            Color::from_hex("#fffe71", AlphaPosition::None).unwrap(),
            "pastel yellow",
        ),
        (
            Color::from_hex("#6241c7", AlphaPosition::None).unwrap(),
            "bluey purple",
        ),
        (
            Color::from_hex("#fffe40", AlphaPosition::None).unwrap(),
            "canary yellow",
        ),
        (
            Color::from_hex("#d3494e", AlphaPosition::None).unwrap(),
            "faded red",
        ),
        (
            Color::from_hex("#985e2b", AlphaPosition::None).unwrap(),
            "sepia",
        ),
        (
            Color::from_hex("#a6814c", AlphaPosition::None).unwrap(),
            "coffee",
        ),
        (
            Color::from_hex("#ff08e8", AlphaPosition::None).unwrap(),
            "bright magenta",
        ),
        (
            Color::from_hex("#9d7651", AlphaPosition::None).unwrap(),
            "mocha",
        ),
        (
            Color::from_hex("#feffca", AlphaPosition::None).unwrap(),
            "ecru",
        ),
        (
            Color::from_hex("#98568d", AlphaPosition::None).unwrap(),
            "purpleish",
        ),
        (
            Color::from_hex("#9e003a", AlphaPosition::None).unwrap(),
            "cranberry",
        ),
        (
            Color::from_hex("#287c37", AlphaPosition::None).unwrap(),
            "darkish green",
        ),
        (
            Color::from_hex("#b96902", AlphaPosition::None).unwrap(),
            "brown orange",
        ),
        (
            Color::from_hex("#ba6873", AlphaPosition::None).unwrap(),
            "dusky rose",
        ),
        (
            Color::from_hex("#ff7855", AlphaPosition::None).unwrap(),
            "melon",
        ),
        (
            Color::from_hex("#94b21c", AlphaPosition::None).unwrap(),
            "sickly green",
        ),
        (
            Color::from_hex("#c5c9c7", AlphaPosition::None).unwrap(),
            "silver",
        ),
        (
            Color::from_hex("#661aee", AlphaPosition::None).unwrap(),
            "purply blue",
        ),
        (
            Color::from_hex("#6140ef", AlphaPosition::None).unwrap(),
            "purpleish blue",
        ),
        (
            Color::from_hex("#9be5aa", AlphaPosition::None).unwrap(),
            "hospital green",
        ),
        (
            Color::from_hex("#7b5804", AlphaPosition::None).unwrap(),
            "shit brown",
        ),
        (
            Color::from_hex("#276ab3", AlphaPosition::None).unwrap(),
            "mid blue",
        ),
        (
            Color::from_hex("#feb308", AlphaPosition::None).unwrap(),
            "amber",
        ),
        (
            Color::from_hex("#8cfd7e", AlphaPosition::None).unwrap(),
            "easter green",
        ),
        (
            Color::from_hex("#6488ea", AlphaPosition::None).unwrap(),
            "soft blue",
        ),
        (
            Color::from_hex("#056eee", AlphaPosition::None).unwrap(),
            "cerulean blue",
        ),
        (
            Color::from_hex("#b27a01", AlphaPosition::None).unwrap(),
            "golden brown",
        ),
        (
            Color::from_hex("#0ffef9", AlphaPosition::None).unwrap(),
            "bright turquoise",
        ),
        (
            Color::from_hex("#fa2a55", AlphaPosition::None).unwrap(),
            "red pink",
        ),
        (
            Color::from_hex("#820747", AlphaPosition::None).unwrap(),
            "red purple",
        ),
        (
            Color::from_hex("#7a6a4f", AlphaPosition::None).unwrap(),
            "greyish brown",
        ),
        (
            Color::from_hex("#f4320c", AlphaPosition::None).unwrap(),
            "vermillion",
        ),
        (
            Color::from_hex("#a13905", AlphaPosition::None).unwrap(),
            "russet",
        ),
        (
            Color::from_hex("#6f828a", AlphaPosition::None).unwrap(),
            "steel grey",
        ),
        (
            Color::from_hex("#a55af4", AlphaPosition::None).unwrap(),
            "lighter purple",
        ),
        (
            Color::from_hex("#ad0afd", AlphaPosition::None).unwrap(),
            "bright violet",
        ),
        (
            Color::from_hex("#004577", AlphaPosition::None).unwrap(),
            "prussian blue",
        ),
        (
            Color::from_hex("#658d6d", AlphaPosition::None).unwrap(),
            "slate green",
        ),
        (
            Color::from_hex("#ca7b80", AlphaPosition::None).unwrap(),
            "dirty pink",
        ),
        (
            Color::from_hex("#005249", AlphaPosition::None).unwrap(),
            "dark blue green",
        ),
        (
            Color::from_hex("#2b5d34", AlphaPosition::None).unwrap(),
            "pine",
        ),
        (
            Color::from_hex("#bff128", AlphaPosition::None).unwrap(),
            "yellowy green",
        ),
        (
            Color::from_hex("#b59410", AlphaPosition::None).unwrap(),
            "dark gold",
        ),
        (
            Color::from_hex("#2976bb", AlphaPosition::None).unwrap(),
            "bluish",
        ),
        (
            Color::from_hex("#014182", AlphaPosition::None).unwrap(),
            "darkish blue",
        ),
        (
            Color::from_hex("#bb3f3f", AlphaPosition::None).unwrap(),
            "dull red",
        ),
        (
            Color::from_hex("#fc2647", AlphaPosition::None).unwrap(),
            "pinky red",
        ),
        (
            Color::from_hex("#a87900", AlphaPosition::None).unwrap(),
            "bronze",
        ),
        (
            Color::from_hex("#82cbb2", AlphaPosition::None).unwrap(),
            "pale teal",
        ),
        (
            Color::from_hex("#667c3e", AlphaPosition::None).unwrap(),
            "military green",
        ),
        (
            Color::from_hex("#fe46a5", AlphaPosition::None).unwrap(),
            "barbie pink",
        ),
        (
            Color::from_hex("#fe83cc", AlphaPosition::None).unwrap(),
            "bubblegum pink",
        ),
        (
            Color::from_hex("#94a617", AlphaPosition::None).unwrap(),
            "pea soup green",
        ),
        (
            Color::from_hex("#a88905", AlphaPosition::None).unwrap(),
            "dark mustard",
        ),
        (
            Color::from_hex("#7f5f00", AlphaPosition::None).unwrap(),
            "shit",
        ),
        (
            Color::from_hex("#9e43a2", AlphaPosition::None).unwrap(),
            "medium purple",
        ),
        (
            Color::from_hex("#062e03", AlphaPosition::None).unwrap(),
            "very dark green",
        ),
        (
            Color::from_hex("#8a6e45", AlphaPosition::None).unwrap(),
            "dirt",
        ),
        (
            Color::from_hex("#cc7a8b", AlphaPosition::None).unwrap(),
            "dusky pink",
        ),
        (
            Color::from_hex("#9e0168", AlphaPosition::None).unwrap(),
            "red violet",
        ),
        (
            Color::from_hex("#fdff38", AlphaPosition::None).unwrap(),
            "lemon yellow",
        ),
        (
            Color::from_hex("#c0fa8b", AlphaPosition::None).unwrap(),
            "pistachio",
        ),
        (
            Color::from_hex("#eedc5b", AlphaPosition::None).unwrap(),
            "dull yellow",
        ),
        (
            Color::from_hex("#7ebd01", AlphaPosition::None).unwrap(),
            "dark lime green",
        ),
        (
            Color::from_hex("#3b5b92", AlphaPosition::None).unwrap(),
            "denim blue",
        ),
        (
            Color::from_hex("#01889f", AlphaPosition::None).unwrap(),
            "teal blue",
        ),
        (
            Color::from_hex("#3d7afd", AlphaPosition::None).unwrap(),
            "lightish blue",
        ),
        (
            Color::from_hex("#5f34e7", AlphaPosition::None).unwrap(),
            "purpley blue",
        ),
        (
            Color::from_hex("#6d5acf", AlphaPosition::None).unwrap(),
            "light indigo",
        ),
        (
            Color::from_hex("#748500", AlphaPosition::None).unwrap(),
            "swamp green",
        ),
        (
            Color::from_hex("#706c11", AlphaPosition::None).unwrap(),
            "brown green",
        ),
        (
            Color::from_hex("#3c0008", AlphaPosition::None).unwrap(),
            "dark maroon",
        ),
        (
            Color::from_hex("#cb00f5", AlphaPosition::None).unwrap(),
            "hot purple",
        ),
        (
            Color::from_hex("#002d04", AlphaPosition::None).unwrap(),
            "dark forest green",
        ),
        (
            Color::from_hex("#658cbb", AlphaPosition::None).unwrap(),
            "faded blue",
        ),
        (
            Color::from_hex("#749551", AlphaPosition::None).unwrap(),
            "drab green",
        ),
        (
            Color::from_hex("#b9ff66", AlphaPosition::None).unwrap(),
            "light lime green",
        ),
        (
            Color::from_hex("#9dc100", AlphaPosition::None).unwrap(),
            "snot green",
        ),
        (
            Color::from_hex("#faee66", AlphaPosition::None).unwrap(),
            "yellowish",
        ),
        (
            Color::from_hex("#7efbb3", AlphaPosition::None).unwrap(),
            "light blue green",
        ),
        (
            Color::from_hex("#7b002c", AlphaPosition::None).unwrap(),
            "bordeaux",
        ),
        (
            Color::from_hex("#c292a1", AlphaPosition::None).unwrap(),
            "light mauve",
        ),
        (
            Color::from_hex("#017b92", AlphaPosition::None).unwrap(),
            "ocean",
        ),
        (
            Color::from_hex("#fcc006", AlphaPosition::None).unwrap(),
            "marigold",
        ),
        (
            Color::from_hex("#657432", AlphaPosition::None).unwrap(),
            "muddy green",
        ),
        (
            Color::from_hex("#d8863b", AlphaPosition::None).unwrap(),
            "dull orange",
        ),
        (
            Color::from_hex("#738595", AlphaPosition::None).unwrap(),
            "steel",
        ),
        (
            Color::from_hex("#aa23ff", AlphaPosition::None).unwrap(),
            "electric purple",
        ),
        (
            Color::from_hex("#08ff08", AlphaPosition::None).unwrap(),
            "fluorescent green",
        ),
        (
            Color::from_hex("#9b7a01", AlphaPosition::None).unwrap(),
            "yellowish brown",
        ),
        (
            Color::from_hex("#f29e8e", AlphaPosition::None).unwrap(),
            "blush",
        ),
        (
            Color::from_hex("#6fc276", AlphaPosition::None).unwrap(),
            "soft green",
        ),
        (
            Color::from_hex("#ff5b00", AlphaPosition::None).unwrap(),
            "bright orange",
        ),
        (
            Color::from_hex("#fdff52", AlphaPosition::None).unwrap(),
            "lemon",
        ),
        (
            Color::from_hex("#866f85", AlphaPosition::None).unwrap(),
            "purple grey",
        ),
        (
            Color::from_hex("#8ffe09", AlphaPosition::None).unwrap(),
            "acid green",
        ),
        (
            Color::from_hex("#eecffe", AlphaPosition::None).unwrap(),
            "pale lavender",
        ),
        (
            Color::from_hex("#510ac9", AlphaPosition::None).unwrap(),
            "violet blue",
        ),
        (
            Color::from_hex("#4f9153", AlphaPosition::None).unwrap(),
            "light forest green",
        ),
        (
            Color::from_hex("#9f2305", AlphaPosition::None).unwrap(),
            "burnt red",
        ),
        (
            Color::from_hex("#728639", AlphaPosition::None).unwrap(),
            "khaki green",
        ),
        (
            Color::from_hex("#de0c62", AlphaPosition::None).unwrap(),
            "cerise",
        ),
        (
            Color::from_hex("#916e99", AlphaPosition::None).unwrap(),
            "faded purple",
        ),
        (
            Color::from_hex("#ffb16d", AlphaPosition::None).unwrap(),
            "apricot",
        ),
        (
            Color::from_hex("#3c4d03", AlphaPosition::None).unwrap(),
            "dark olive green",
        ),
        (
            Color::from_hex("#7f7053", AlphaPosition::None).unwrap(),
            "grey brown",
        ),
        (
            Color::from_hex("#77926f", AlphaPosition::None).unwrap(),
            "green grey",
        ),
        (
            Color::from_hex("#010fcc", AlphaPosition::None).unwrap(),
            "true blue",
        ),
        (
            Color::from_hex("#ceaefa", AlphaPosition::None).unwrap(),
            "pale violet",
        ),
        (
            Color::from_hex("#8f99fb", AlphaPosition::None).unwrap(),
            "periwinkle blue",
        ),
        (
            Color::from_hex("#c6fcff", AlphaPosition::None).unwrap(),
            "light sky blue",
        ),
        (
            Color::from_hex("#5539cc", AlphaPosition::None).unwrap(),
            "blurple",
        ),
        (
            Color::from_hex("#544e03", AlphaPosition::None).unwrap(),
            "green brown",
        ),
        (
            Color::from_hex("#017a79", AlphaPosition::None).unwrap(),
            "bluegreen",
        ),
        (
            Color::from_hex("#01f9c6", AlphaPosition::None).unwrap(),
            "bright teal",
        ),
        (
            Color::from_hex("#c9b003", AlphaPosition::None).unwrap(),
            "brownish yellow",
        ),
        (
            Color::from_hex("#929901", AlphaPosition::None).unwrap(),
            "pea soup",
        ),
        (
            Color::from_hex("#0b5509", AlphaPosition::None).unwrap(),
            "forest",
        ),
        (
            Color::from_hex("#a00498", AlphaPosition::None).unwrap(),
            "barney purple",
        ),
        (
            Color::from_hex("#2000b1", AlphaPosition::None).unwrap(),
            "ultramarine",
        ),
        (
            Color::from_hex("#94568c", AlphaPosition::None).unwrap(),
            "purplish",
        ),
        (
            Color::from_hex("#c2be0e", AlphaPosition::None).unwrap(),
            "puke yellow",
        ),
        (
            Color::from_hex("#748b97", AlphaPosition::None).unwrap(),
            "bluish grey",
        ),
        (
            Color::from_hex("#665fd1", AlphaPosition::None).unwrap(),
            "dark periwinkle",
        ),
        (
            Color::from_hex("#9c6da5", AlphaPosition::None).unwrap(),
            "dark lilac",
        ),
        (
            Color::from_hex("#c44240", AlphaPosition::None).unwrap(),
            "reddish",
        ),
        (
            Color::from_hex("#a24857", AlphaPosition::None).unwrap(),
            "light maroon",
        ),
        (
            Color::from_hex("#825f87", AlphaPosition::None).unwrap(),
            "dusty purple",
        ),
        (
            Color::from_hex("#c9643b", AlphaPosition::None).unwrap(),
            "terra cotta",
        ),
        (
            Color::from_hex("#90b134", AlphaPosition::None).unwrap(),
            "avocado",
        ),
        (
            Color::from_hex("#01386a", AlphaPosition::None).unwrap(),
            "marine blue",
        ),
        (
            Color::from_hex("#25a36f", AlphaPosition::None).unwrap(),
            "teal green",
        ),
        (
            Color::from_hex("#59656d", AlphaPosition::None).unwrap(),
            "slate grey",
        ),
        (
            Color::from_hex("#75fd63", AlphaPosition::None).unwrap(),
            "lighter green",
        ),
        (
            Color::from_hex("#21fc0d", AlphaPosition::None).unwrap(),
            "electric green",
        ),
        (
            Color::from_hex("#5a86ad", AlphaPosition::None).unwrap(),
            "dusty blue",
        ),
        (
            Color::from_hex("#fec615", AlphaPosition::None).unwrap(),
            "golden yellow",
        ),
        (
            Color::from_hex("#fffd01", AlphaPosition::None).unwrap(),
            "bright yellow",
        ),
        (
            Color::from_hex("#dfc5fe", AlphaPosition::None).unwrap(),
            "light lavender",
        ),
        (
            Color::from_hex("#b26400", AlphaPosition::None).unwrap(),
            "umber",
        ),
        (
            Color::from_hex("#7f5e00", AlphaPosition::None).unwrap(),
            "poop",
        ),
        (
            Color::from_hex("#de7e5d", AlphaPosition::None).unwrap(),
            "dark peach",
        ),
        (
            Color::from_hex("#048243", AlphaPosition::None).unwrap(),
            "jungle green",
        ),
        (
            Color::from_hex("#ffffd4", AlphaPosition::None).unwrap(),
            "eggshell",
        ),
        (
            Color::from_hex("#3b638c", AlphaPosition::None).unwrap(),
            "denim",
        ),
        (
            Color::from_hex("#b79400", AlphaPosition::None).unwrap(),
            "yellow brown",
        ),
        (
            Color::from_hex("#84597e", AlphaPosition::None).unwrap(),
            "dull purple",
        ),
        (
            Color::from_hex("#411900", AlphaPosition::None).unwrap(),
            "chocolate brown",
        ),
        (
            Color::from_hex("#7b0323", AlphaPosition::None).unwrap(),
            "wine red",
        ),
        (
            Color::from_hex("#04d9ff", AlphaPosition::None).unwrap(),
            "neon blue",
        ),
        (
            Color::from_hex("#667e2c", AlphaPosition::None).unwrap(),
            "dirty green",
        ),
        (
            Color::from_hex("#fbeeac", AlphaPosition::None).unwrap(),
            "light tan",
        ),
        (
            Color::from_hex("#d7fffe", AlphaPosition::None).unwrap(),
            "ice blue",
        ),
        (
            Color::from_hex("#4e7496", AlphaPosition::None).unwrap(),
            "cadet blue",
        ),
        (
            Color::from_hex("#874c62", AlphaPosition::None).unwrap(),
            "dark mauve",
        ),
        (
            Color::from_hex("#d5ffff", AlphaPosition::None).unwrap(),
            "very light blue",
        ),
        (
            Color::from_hex("#826d8c", AlphaPosition::None).unwrap(),
            "grey purple",
        ),
        (
            Color::from_hex("#ffbacd", AlphaPosition::None).unwrap(),
            "pastel pink",
        ),
        (
            Color::from_hex("#d1ffbd", AlphaPosition::None).unwrap(),
            "very light green",
        ),
        (
            Color::from_hex("#448ee4", AlphaPosition::None).unwrap(),
            "dark sky blue",
        ),
        (
            Color::from_hex("#05472a", AlphaPosition::None).unwrap(),
            "evergreen",
        ),
        (
            Color::from_hex("#d5869d", AlphaPosition::None).unwrap(),
            "dull pink",
        ),
        (
            Color::from_hex("#3d0734", AlphaPosition::None).unwrap(),
            "aubergine",
        ),
        (
            Color::from_hex("#4a0100", AlphaPosition::None).unwrap(),
            "mahogany",
        ),
        (
            Color::from_hex("#f8481c", AlphaPosition::None).unwrap(),
            "reddish orange",
        ),
        (
            Color::from_hex("#02590f", AlphaPosition::None).unwrap(),
            "deep green",
        ),
        (
            Color::from_hex("#89a203", AlphaPosition::None).unwrap(),
            "vomit green",
        ),
        (
            Color::from_hex("#e03fd8", AlphaPosition::None).unwrap(),
            "purple pink",
        ),
        (
            Color::from_hex("#d58a94", AlphaPosition::None).unwrap(),
            "dusty pink",
        ),
        (
            Color::from_hex("#7bb274", AlphaPosition::None).unwrap(),
            "faded green",
        ),
        (
            Color::from_hex("#526525", AlphaPosition::None).unwrap(),
            "camo green",
        ),
        (
            Color::from_hex("#c94cbe", AlphaPosition::None).unwrap(),
            "pinky purple",
        ),
        (
            Color::from_hex("#db4bda", AlphaPosition::None).unwrap(),
            "pink purple",
        ),
        (
            Color::from_hex("#9e3623", AlphaPosition::None).unwrap(),
            "brownish red",
        ),
        (
            Color::from_hex("#b5485d", AlphaPosition::None).unwrap(),
            "dark rose",
        ),
        (
            Color::from_hex("#735c12", AlphaPosition::None).unwrap(),
            "mud",
        ),
        (
            Color::from_hex("#9c6d57", AlphaPosition::None).unwrap(),
            "brownish",
        ),
        (
            Color::from_hex("#028f1e", AlphaPosition::None).unwrap(),
            "emerald green",
        ),
        (
            Color::from_hex("#b1916e", AlphaPosition::None).unwrap(),
            "pale brown",
        ),
        (
            Color::from_hex("#49759c", AlphaPosition::None).unwrap(),
            "dull blue",
        ),
        (
            Color::from_hex("#a0450e", AlphaPosition::None).unwrap(),
            "burnt umber",
        ),
        (
            Color::from_hex("#39ad48", AlphaPosition::None).unwrap(),
            "medium green",
        ),
        (
            Color::from_hex("#b66a50", AlphaPosition::None).unwrap(),
            "clay",
        ),
        (
            Color::from_hex("#8cffdb", AlphaPosition::None).unwrap(),
            "light aqua",
        ),
        (
            Color::from_hex("#a4be5c", AlphaPosition::None).unwrap(),
            "light olive green",
        ),
        (
            Color::from_hex("#cb7723", AlphaPosition::None).unwrap(),
            "brownish orange",
        ),
        (
            Color::from_hex("#05696b", AlphaPosition::None).unwrap(),
            "dark aqua",
        ),
        (
            Color::from_hex("#ce5dae", AlphaPosition::None).unwrap(),
            "purplish pink",
        ),
        (
            Color::from_hex("#c85a53", AlphaPosition::None).unwrap(),
            "dark salmon",
        ),
        (
            Color::from_hex("#96ae8d", AlphaPosition::None).unwrap(),
            "greenish grey",
        ),
        (
            Color::from_hex("#1fa774", AlphaPosition::None).unwrap(),
            "jade",
        ),
        (
            Color::from_hex("#7a9703", AlphaPosition::None).unwrap(),
            "ugly green",
        ),
        (
            Color::from_hex("#ac9362", AlphaPosition::None).unwrap(),
            "dark beige",
        ),
        (
            Color::from_hex("#01a049", AlphaPosition::None).unwrap(),
            "emerald",
        ),
        (
            Color::from_hex("#d9544d", AlphaPosition::None).unwrap(),
            "pale red",
        ),
        (
            Color::from_hex("#fa5ff7", AlphaPosition::None).unwrap(),
            "light magenta",
        ),
        (
            Color::from_hex("#82cafc", AlphaPosition::None).unwrap(),
            "sky",
        ),
        (
            Color::from_hex("#acfffc", AlphaPosition::None).unwrap(),
            "light cyan",
        ),
        (
            Color::from_hex("#fcb001", AlphaPosition::None).unwrap(),
            "yellow orange",
        ),
        (
            Color::from_hex("#910951", AlphaPosition::None).unwrap(),
            "reddish purple",
        ),
        (
            Color::from_hex("#fe2c54", AlphaPosition::None).unwrap(),
            "reddish pink",
        ),
        (
            Color::from_hex("#c875c4", AlphaPosition::None).unwrap(),
            "orchid",
        ),
        (
            Color::from_hex("#cdc50a", AlphaPosition::None).unwrap(),
            "dirty yellow",
        ),
        (
            Color::from_hex("#fd411e", AlphaPosition::None).unwrap(),
            "orange red",
        ),
        (
            Color::from_hex("#9a0200", AlphaPosition::None).unwrap(),
            "deep red",
        ),
        (
            Color::from_hex("#be6400", AlphaPosition::None).unwrap(),
            "orange brown",
        ),
        (
            Color::from_hex("#030aa7", AlphaPosition::None).unwrap(),
            "cobalt blue",
        ),
        (
            Color::from_hex("#fe019a", AlphaPosition::None).unwrap(),
            "neon pink",
        ),
        (
            Color::from_hex("#f7879a", AlphaPosition::None).unwrap(),
            "rose pink",
        ),
        (
            Color::from_hex("#887191", AlphaPosition::None).unwrap(),
            "greyish purple",
        ),
        (
            Color::from_hex("#b00149", AlphaPosition::None).unwrap(),
            "raspberry",
        ),
        (
            Color::from_hex("#12e193", AlphaPosition::None).unwrap(),
            "aqua green",
        ),
        (
            Color::from_hex("#fe7b7c", AlphaPosition::None).unwrap(),
            "salmon pink",
        ),
        (
            Color::from_hex("#ff9408", AlphaPosition::None).unwrap(),
            "tangerine",
        ),
        (
            Color::from_hex("#6a6e09", AlphaPosition::None).unwrap(),
            "brownish green",
        ),
        (
            Color::from_hex("#8b2e16", AlphaPosition::None).unwrap(),
            "red brown",
        ),
        (
            Color::from_hex("#696112", AlphaPosition::None).unwrap(),
            "greenish brown",
        ),
        (
            Color::from_hex("#e17701", AlphaPosition::None).unwrap(),
            "pumpkin",
        ),
        (
            Color::from_hex("#0a481e", AlphaPosition::None).unwrap(),
            "pine green",
        ),
        (
            Color::from_hex("#343837", AlphaPosition::None).unwrap(),
            "charcoal",
        ),
        (
            Color::from_hex("#ffb7ce", AlphaPosition::None).unwrap(),
            "baby pink",
        ),
        (
            Color::from_hex("#6a79f7", AlphaPosition::None).unwrap(),
            "cornflower",
        ),
        (
            Color::from_hex("#5d06e9", AlphaPosition::None).unwrap(),
            "blue violet",
        ),
        (
            Color::from_hex("#3d1c02", AlphaPosition::None).unwrap(),
            "chocolate",
        ),
        (
            Color::from_hex("#82a67d", AlphaPosition::None).unwrap(),
            "greyish green",
        ),
        (
            Color::from_hex("#be0119", AlphaPosition::None).unwrap(),
            "scarlet",
        ),
        (
            Color::from_hex("#c9ff27", AlphaPosition::None).unwrap(),
            "green yellow",
        ),
        (
            Color::from_hex("#373e02", AlphaPosition::None).unwrap(),
            "dark olive",
        ),
        (
            Color::from_hex("#a9561e", AlphaPosition::None).unwrap(),
            "sienna",
        ),
        (
            Color::from_hex("#caa0ff", AlphaPosition::None).unwrap(),
            "pastel purple",
        ),
        (
            Color::from_hex("#ca6641", AlphaPosition::None).unwrap(),
            "terracotta",
        ),
        (
            Color::from_hex("#02d8e9", AlphaPosition::None).unwrap(),
            "aqua blue",
        ),
        (
            Color::from_hex("#88b378", AlphaPosition::None).unwrap(),
            "sage green",
        ),
        (
            Color::from_hex("#980002", AlphaPosition::None).unwrap(),
            "blood red",
        ),
        (
            Color::from_hex("#cb0162", AlphaPosition::None).unwrap(),
            "deep pink",
        ),
        (
            Color::from_hex("#5cac2d", AlphaPosition::None).unwrap(),
            "grass",
        ),
        (
            Color::from_hex("#769958", AlphaPosition::None).unwrap(),
            "moss",
        ),
        (
            Color::from_hex("#a2bffe", AlphaPosition::None).unwrap(),
            "pastel blue",
        ),
        (
            Color::from_hex("#10a674", AlphaPosition::None).unwrap(),
            "bluish green",
        ),
        (
            Color::from_hex("#06b48b", AlphaPosition::None).unwrap(),
            "green blue",
        ),
        (
            Color::from_hex("#af884a", AlphaPosition::None).unwrap(),
            "dark tan",
        ),
        (
            Color::from_hex("#0b8b87", AlphaPosition::None).unwrap(),
            "greenish blue",
        ),
        (
            Color::from_hex("#ffa756", AlphaPosition::None).unwrap(),
            "pale orange",
        ),
        (
            Color::from_hex("#a2a415", AlphaPosition::None).unwrap(),
            "vomit",
        ),
        (
            Color::from_hex("#154406", AlphaPosition::None).unwrap(),
            "forrest green",
        ),
        (
            Color::from_hex("#856798", AlphaPosition::None).unwrap(),
            "dark lavender",
        ),
        (
            Color::from_hex("#34013f", AlphaPosition::None).unwrap(),
            "dark violet",
        ),
        (
            Color::from_hex("#632de9", AlphaPosition::None).unwrap(),
            "purple blue",
        ),
        (
            Color::from_hex("#0a888a", AlphaPosition::None).unwrap(),
            "dark cyan",
        ),
        (
            Color::from_hex("#6f7632", AlphaPosition::None).unwrap(),
            "olive drab",
        ),
        (
            Color::from_hex("#d46a7e", AlphaPosition::None).unwrap(),
            "pinkish",
        ),
        (
            Color::from_hex("#1e488f", AlphaPosition::None).unwrap(),
            "cobalt",
        ),
        (
            Color::from_hex("#bc13fe", AlphaPosition::None).unwrap(),
            "neon purple",
        ),
        (
            Color::from_hex("#7ef4cc", AlphaPosition::None).unwrap(),
            "light turquoise",
        ),
        (
            Color::from_hex("#76cd26", AlphaPosition::None).unwrap(),
            "apple green",
        ),
        (
            Color::from_hex("#74a662", AlphaPosition::None).unwrap(),
            "dull green",
        ),
        (
            Color::from_hex("#80013f", AlphaPosition::None).unwrap(),
            "wine",
        ),
        (
            Color::from_hex("#b1d1fc", AlphaPosition::None).unwrap(),
            "powder blue",
        ),
        (
            Color::from_hex("#ffffe4", AlphaPosition::None).unwrap(),
            "off white",
        ),
        (
            Color::from_hex("#0652ff", AlphaPosition::None).unwrap(),
            "electric blue",
        ),
        (
            Color::from_hex("#045c5a", AlphaPosition::None).unwrap(),
            "dark turquoise",
        ),
        (
            Color::from_hex("#5729ce", AlphaPosition::None).unwrap(),
            "blue purple",
        ),
        (
            Color::from_hex("#069af3", AlphaPosition::None).unwrap(),
            "azure",
        ),
        (
            Color::from_hex("#FF0000", AlphaPosition::None).unwrap(),
            "bright red",
        ),
        (
            Color::from_hex("#f10c45", AlphaPosition::None).unwrap(),
            "pinkish red",
        ),
        (
            Color::from_hex("#5170d7", AlphaPosition::None).unwrap(),
            "cornflower blue",
        ),
        (
            Color::from_hex("#acbf69", AlphaPosition::None).unwrap(),
            "light olive",
        ),
        (
            Color::from_hex("#6c3461", AlphaPosition::None).unwrap(),
            "grape",
        ),
        (
            Color::from_hex("#5e819d", AlphaPosition::None).unwrap(),
            "greyish blue",
        ),
        (
            Color::from_hex("#601ef9", AlphaPosition::None).unwrap(),
            "purplish blue",
        ),
        (
            Color::from_hex("#b0dd16", AlphaPosition::None).unwrap(),
            "yellowish green",
        ),
        (
            Color::from_hex("#cdfd02", AlphaPosition::None).unwrap(),
            "greenish yellow",
        ),
        (
            Color::from_hex("#2c6fbb", AlphaPosition::None).unwrap(),
            "medium blue",
        ),
        (
            Color::from_hex("#c0737a", AlphaPosition::None).unwrap(),
            "dusty rose",
        ),
        (
            Color::from_hex("#d6b4fc", AlphaPosition::None).unwrap(),
            "light violet",
        ),
        (
            Color::from_hex("#020035", AlphaPosition::None).unwrap(),
            "midnight blue",
        ),
        (
            Color::from_hex("#703be7", AlphaPosition::None).unwrap(),
            "bluish purple",
        ),
        (
            Color::from_hex("#fd3c06", AlphaPosition::None).unwrap(),
            "red orange",
        ),
        (
            Color::from_hex("#960056", AlphaPosition::None).unwrap(),
            "dark magenta",
        ),
        (
            Color::from_hex("#40a368", AlphaPosition::None).unwrap(),
            "greenish",
        ),
        (
            Color::from_hex("#03719c", AlphaPosition::None).unwrap(),
            "ocean blue",
        ),
        (
            Color::from_hex("#fc5a50", AlphaPosition::None).unwrap(),
            "coral",
        ),
        (
            Color::from_hex("#ffffc2", AlphaPosition::None).unwrap(),
            "cream",
        ),
        (
            Color::from_hex("#7f2b0a", AlphaPosition::None).unwrap(),
            "reddish brown",
        ),
        (
            Color::from_hex("#b04e0f", AlphaPosition::None).unwrap(),
            "burnt sienna",
        ),
        (
            Color::from_hex("#a03623", AlphaPosition::None).unwrap(),
            "brick",
        ),
        (
            Color::from_hex("#87ae73", AlphaPosition::None).unwrap(),
            "sage",
        ),
        (
            Color::from_hex("#789b73", AlphaPosition::None).unwrap(),
            "grey green",
        ),
        (
            Color::from_hex("#ffffff", AlphaPosition::None).unwrap(),
            "white",
        ),
        (
            Color::from_hex("#98eff9", AlphaPosition::None).unwrap(),
            "robin's egg blue",
        ),
        (
            Color::from_hex("#658b38", AlphaPosition::None).unwrap(),
            "moss green",
        ),
        (
            Color::from_hex("#5a7d9a", AlphaPosition::None).unwrap(),
            "steel blue",
        ),
        (
            Color::from_hex("#380835", AlphaPosition::None).unwrap(),
            "eggplant",
        ),
        (
            Color::from_hex("#fffe7a", AlphaPosition::None).unwrap(),
            "light yellow",
        ),
        (
            Color::from_hex("#5ca904", AlphaPosition::None).unwrap(),
            "leaf green",
        ),
        (
            Color::from_hex("#d8dcd6", AlphaPosition::None).unwrap(),
            "light grey",
        ),
        (
            Color::from_hex("#a5a502", AlphaPosition::None).unwrap(),
            "puke",
        ),
        (
            Color::from_hex("#d648d7", AlphaPosition::None).unwrap(),
            "pinkish purple",
        ),
        (
            Color::from_hex("#047495", AlphaPosition::None).unwrap(),
            "sea blue",
        ),
        (
            Color::from_hex("#b790d4", AlphaPosition::None).unwrap(),
            "pale purple",
        ),
        (
            Color::from_hex("#5b7c99", AlphaPosition::None).unwrap(),
            "slate blue",
        ),
        (
            Color::from_hex("#607c8e", AlphaPosition::None).unwrap(),
            "blue grey",
        ),
        (
            Color::from_hex("#0b4008", AlphaPosition::None).unwrap(),
            "hunter green",
        ),
        (
            Color::from_hex("#ed0dd9", AlphaPosition::None).unwrap(),
            "fuchsia",
        ),
        (
            Color::from_hex("#8c000f", AlphaPosition::None).unwrap(),
            "crimson",
        ),
        (
            Color::from_hex("#ffff84", AlphaPosition::None).unwrap(),
            "pale yellow",
        ),
        (
            Color::from_hex("#bf9005", AlphaPosition::None).unwrap(),
            "ochre",
        ),
        (
            Color::from_hex("#d2bd0a", AlphaPosition::None).unwrap(),
            "mustard yellow",
        ),
        (
            Color::from_hex("#ff474c", AlphaPosition::None).unwrap(),
            "light red",
        ),
        (
            Color::from_hex("#0485d1", AlphaPosition::None).unwrap(),
            "cerulean",
        ),
        (
            Color::from_hex("#ffcfdc", AlphaPosition::None).unwrap(),
            "pale pink",
        ),
        (
            Color::from_hex("#040273", AlphaPosition::None).unwrap(),
            "deep blue",
        ),
        (
            Color::from_hex("#a83c09", AlphaPosition::None).unwrap(),
            "rust",
        ),
        (
            Color::from_hex("#90e4c1", AlphaPosition::None).unwrap(),
            "light teal",
        ),
        (
            Color::from_hex("#516572", AlphaPosition::None).unwrap(),
            "slate",
        ),
        (
            Color::from_hex("#fac205", AlphaPosition::None).unwrap(),
            "goldenrod",
        ),
        (
            Color::from_hex("#d5b60a", AlphaPosition::None).unwrap(),
            "dark yellow",
        ),
        (
            Color::from_hex("#363737", AlphaPosition::None).unwrap(),
            "dark grey",
        ),
        (
            Color::from_hex("#4b5d16", AlphaPosition::None).unwrap(),
            "army green",
        ),
        (
            Color::from_hex("#6b8ba4", AlphaPosition::None).unwrap(),
            "grey blue",
        ),
        (
            Color::from_hex("#80f9ad", AlphaPosition::None).unwrap(),
            "seafoam",
        ),
        (
            Color::from_hex("#a57e52", AlphaPosition::None).unwrap(),
            "puce",
        ),
        (
            Color::from_hex("#a9f971", AlphaPosition::None).unwrap(),
            "spring green",
        ),
        (
            Color::from_hex("#c65102", AlphaPosition::None).unwrap(),
            "dark orange",
        ),
        (
            Color::from_hex("#e2ca76", AlphaPosition::None).unwrap(),
            "sand",
        ),
        (
            Color::from_hex("#b0ff9d", AlphaPosition::None).unwrap(),
            "pastel green",
        ),
        (
            Color::from_hex("#9ffeb0", AlphaPosition::None).unwrap(),
            "mint",
        ),
        (
            Color::from_hex("#fdaa48", AlphaPosition::None).unwrap(),
            "light orange",
        ),
        (
            Color::from_hex("#fe01b1", AlphaPosition::None).unwrap(),
            "bright pink",
        ),
        (
            Color::from_hex("#c1f80a", AlphaPosition::None).unwrap(),
            "chartreuse",
        ),
        (
            Color::from_hex("#36013f", AlphaPosition::None).unwrap(),
            "deep purple",
        ),
        (
            Color::from_hex("#341c02", AlphaPosition::None).unwrap(),
            "dark brown",
        ),
        (
            Color::from_hex("#b9a281", AlphaPosition::None).unwrap(),
            "taupe",
        ),
        (
            Color::from_hex("#8eab12", AlphaPosition::None).unwrap(),
            "pea green",
        ),
        (
            Color::from_hex("#9aae07", AlphaPosition::None).unwrap(),
            "puke green",
        ),
        (
            Color::from_hex("#02ab2e", AlphaPosition::None).unwrap(),
            "kelly green",
        ),
        (
            Color::from_hex("#7af9ab", AlphaPosition::None).unwrap(),
            "seafoam green",
        ),
        (
            Color::from_hex("#137e6d", AlphaPosition::None).unwrap(),
            "blue green",
        ),
        (
            Color::from_hex("#aaa662", AlphaPosition::None).unwrap(),
            "khaki",
        ),
        (
            Color::from_hex("#610023", AlphaPosition::None).unwrap(),
            "burgundy",
        ),
        (
            Color::from_hex("#014d4e", AlphaPosition::None).unwrap(),
            "dark teal",
        ),
        (
            Color::from_hex("#8f1402", AlphaPosition::None).unwrap(),
            "brick red",
        ),
        (
            Color::from_hex("#4b006e", AlphaPosition::None).unwrap(),
            "royal purple",
        ),
        (
            Color::from_hex("#580f41", AlphaPosition::None).unwrap(),
            "plum",
        ),
        (
            Color::from_hex("#8fff9f", AlphaPosition::None).unwrap(),
            "mint green",
        ),
        (
            Color::from_hex("#dbb40c", AlphaPosition::None).unwrap(),
            "gold",
        ),
        (
            Color::from_hex("#a2cffe", AlphaPosition::None).unwrap(),
            "baby blue",
        ),
        (
            Color::from_hex("#c0fb2d", AlphaPosition::None).unwrap(),
            "yellow green",
        ),
        (
            Color::from_hex("#be03fd", AlphaPosition::None).unwrap(),
            "bright purple",
        ),
        (
            Color::from_hex("#840000", AlphaPosition::None).unwrap(),
            "dark red",
        ),
        (
            Color::from_hex("#d0fefe", AlphaPosition::None).unwrap(),
            "pale blue",
        ),
        (
            Color::from_hex("#3f9b0b", AlphaPosition::None).unwrap(),
            "grass green",
        ),
        (
            Color::from_hex("#01153e", AlphaPosition::None).unwrap(),
            "navy",
        ),
        (
            Color::from_hex("#04d8b2", AlphaPosition::None).unwrap(),
            "aquamarine",
        ),
        (
            Color::from_hex("#c04e01", AlphaPosition::None).unwrap(),
            "burnt orange",
        ),
        (
            Color::from_hex("#0cff0c", AlphaPosition::None).unwrap(),
            "neon green",
        ),
        (
            Color::from_hex("#0165fc", AlphaPosition::None).unwrap(),
            "bright blue",
        ),
        (
            Color::from_hex("#cf6275", AlphaPosition::None).unwrap(),
            "rose",
        ),
        (
            Color::from_hex("#ffd1df", AlphaPosition::None).unwrap(),
            "light pink",
        ),
        (
            Color::from_hex("#ceb301", AlphaPosition::None).unwrap(),
            "mustard",
        ),
        (
            Color::from_hex("#380282", AlphaPosition::None).unwrap(),
            "indigo",
        ),
        (
            Color::from_hex("#aaff32", AlphaPosition::None).unwrap(),
            "lime",
        ),
        (
            Color::from_hex("#53fca1", AlphaPosition::None).unwrap(),
            "sea green",
        ),
        (
            Color::from_hex("#8e82fe", AlphaPosition::None).unwrap(),
            "periwinkle",
        ),
        (
            Color::from_hex("#cb416b", AlphaPosition::None).unwrap(),
            "dark pink",
        ),
        (
            Color::from_hex("#677a04", AlphaPosition::None).unwrap(),
            "olive green",
        ),
        (
            Color::from_hex("#ffb07c", AlphaPosition::None).unwrap(),
            "peach",
        ),
        (
            Color::from_hex("#c7fdb5", AlphaPosition::None).unwrap(),
            "pale green",
        ),
        (
            Color::from_hex("#ad8150", AlphaPosition::None).unwrap(),
            "light brown",
        ),
        (
            Color::from_hex("#ff028d", AlphaPosition::None).unwrap(),
            "hot pink",
        ),
        (
            Color::from_hex("#000000", AlphaPosition::None).unwrap(),
            "black",
        ),
        (
            Color::from_hex("#cea2fd", AlphaPosition::None).unwrap(),
            "lilac",
        ),
        (
            Color::from_hex("#001146", AlphaPosition::None).unwrap(),
            "navy blue",
        ),
        (
            Color::from_hex("#0504aa", AlphaPosition::None).unwrap(),
            "royal blue",
        ),
        (
            Color::from_hex("#e6daa6", AlphaPosition::None).unwrap(),
            "beige",
        ),
        (
            Color::from_hex("#ff796c", AlphaPosition::None).unwrap(),
            "salmon",
        ),
        (
            Color::from_hex("#6e750e", AlphaPosition::None).unwrap(),
            "olive",
        ),
        (
            Color::from_hex("#650021", AlphaPosition::None).unwrap(),
            "maroon",
        ),
        (
            Color::from_hex("#01ff07", AlphaPosition::None).unwrap(),
            "bright green",
        ),
        (
            Color::from_hex("#35063e", AlphaPosition::None).unwrap(),
            "dark purple",
        ),
        (
            Color::from_hex("#ae7181", AlphaPosition::None).unwrap(),
            "mauve",
        ),
        (
            Color::from_hex("#06470c", AlphaPosition::None).unwrap(),
            "forest green",
        ),
        (
            Color::from_hex("#13eac9", AlphaPosition::None).unwrap(),
            "aqua",
        ),
        (
            Color::from_hex("#00ffff", AlphaPosition::None).unwrap(),
            "cyan",
        ),
        (
            Color::from_hex("#d1b26f", AlphaPosition::None).unwrap(),
            "tan",
        ),
        (
            Color::from_hex("#00035b", AlphaPosition::None).unwrap(),
            "dark blue",
        ),
        (
            Color::from_hex("#c79fef", AlphaPosition::None).unwrap(),
            "lavender",
        ),
        (
            Color::from_hex("#06c2ac", AlphaPosition::None).unwrap(),
            "turquoise",
        ),
        (
            Color::from_hex("#033500", AlphaPosition::None).unwrap(),
            "dark green",
        ),
        (
            Color::from_hex("#9a0eea", AlphaPosition::None).unwrap(),
            "violet",
        ),
        (
            Color::from_hex("#bf77f6", AlphaPosition::None).unwrap(),
            "light purple",
        ),
        (
            Color::from_hex("#89fe05", AlphaPosition::None).unwrap(),
            "lime green",
        ),
        (
            Color::from_hex("#929591", AlphaPosition::None).unwrap(),
            "grey",
        ),
        (
            Color::from_hex("#75bbfd", AlphaPosition::None).unwrap(),
            "sky blue",
        ),
        (
            Color::from_hex("#ffff14", AlphaPosition::None).unwrap(),
            "yellow",
        ),
        (
            Color::from_hex("#c20078", AlphaPosition::None).unwrap(),
            "magenta",
        ),
        (
            Color::from_hex("#96f97b", AlphaPosition::None).unwrap(),
            "light green",
        ),
        (
            Color::from_hex("#f97306", AlphaPosition::None).unwrap(),
            "orange",
        ),
        (
            Color::from_hex("#029386", AlphaPosition::None).unwrap(),
            "teal",
        ),
        (
            Color::from_hex("#95d0fc", AlphaPosition::None).unwrap(),
            "light blue",
        ),
        (
            Color::from_hex("#e50000", AlphaPosition::None).unwrap(),
            "red",
        ),
        (
            Color::from_hex("#653700", AlphaPosition::None).unwrap(),
            "brown",
        ),
        (
            Color::from_hex("#ff81c0", AlphaPosition::None).unwrap(),
            "pink",
        ),
        (
            Color::from_hex("#0343df", AlphaPosition::None).unwrap(),
            "blue",
        ),
        (
            Color::from_hex("#15b01a", AlphaPosition::None).unwrap(),
            "green",
        ),
        (
            Color::from_hex("#7e1e9c", AlphaPosition::None).unwrap(),
            "purple",
        ),
    ])
}
