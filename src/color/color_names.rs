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
            return Some(name.to_owned());
        } else {
            return name(color, false, extended, xkcd);
        }
    } else if extended {
        if let Some(name) = w3c_extended_names().get(&color) {
            return Some(name.to_owned());
        } else {
            return name(color, basic, false, xkcd);
        }
    } else if xkcd {
        if let Some(name) = xkcd_names().get(&color) {
            return Some(name.to_owned());
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
pub fn w3c_basic_names() -> HashMap<Color, String> {
    HashMap::from([
        (Color::rgb(0, 0, 0), "black".to_string()),
        (Color::rgb(192, 192, 192), "silver".to_string()),
        (Color::rgb(128, 128, 128), "gray".to_string()),
        (Color::rgb(255, 255, 255), "white".to_string()),
        (Color::rgb(128, 0, 0), "maroon".to_string()),
        (Color::rgb(255, 0, 0), "red".to_string()),
        (Color::rgb(128, 0, 128), "purple".to_string()),
        (Color::rgb(255, 0, 255), "fuchsia".to_string()),
        (Color::rgb(0, 128, 0), "green".to_string()),
        (Color::rgb(0, 255, 0), "lime".to_string()),
        (Color::rgb(128, 128, 0), "olive".to_string()),
        (Color::rgb(255, 255, 0), "yellow".to_string()),
        (Color::rgb(0, 0, 128), "navy".to_string()),
        (Color::rgb(0, 0, 255), "blue".to_string()),
        (Color::rgb(0, 128, 128), "teal".to_string()),
        (Color::rgb(0, 255, 255), "aqua".to_string()),
    ])
}

/// Returns the [w3c extended color keywords](https://www.w3.org/TR/css-color-3/#svg-color).
///
/// The names are mapped to their corresponding color.
/// To get the name of a color the [name] function is preferred.
pub fn w3c_extended_names() -> HashMap<Color, String> {
    HashMap::from([
        (Color::rgb(240, 248, 255), "aliceblue".to_string()),
        (Color::rgb(250, 235, 215), "antiquewhite".to_string()),
        (Color::rgb(0, 255, 255), "aqua".to_string()),
        (Color::rgb(127, 255, 212), "aquamarine".to_string()),
        (Color::rgb(240, 255, 255), "azure".to_string()),
        (Color::rgb(245, 245, 220), "beige".to_string()),
        (Color::rgb(255, 228, 196), "bisque".to_string()),
        (Color::rgb(0, 0, 0), "black".to_string()),
        (Color::rgb(255, 235, 205), "blanchedalmond".to_string()),
        (Color::rgb(0, 0, 255), "blue".to_string()),
        (Color::rgb(138, 43, 226), "blueviolet".to_string()),
        (Color::rgb(165, 42, 42), "brown".to_string()),
        (Color::rgb(222, 184, 135), "burlywood".to_string()),
        (Color::rgb(95, 158, 160), "cadetblue".to_string()),
        (Color::rgb(127, 255, 0), "chartreuse".to_string()),
        (Color::rgb(210, 105, 30), "chocolate".to_string()),
        (Color::rgb(255, 127, 80), "coral".to_string()),
        (Color::rgb(100, 149, 237), "cornflowerblue".to_string()),
        (Color::rgb(255, 248, 220), "cornsilk".to_string()),
        (Color::rgb(220, 20, 60), "crimson".to_string()),
        (Color::rgb(0, 255, 255), "cyan".to_string()),
        (Color::rgb(0, 0, 139), "darkblue".to_string()),
        (Color::rgb(0, 139, 139), "darkcyan".to_string()),
        (Color::rgb(184, 134, 11), "darkgoldenrod".to_string()),
        (Color::rgb(169, 169, 169), "darkgray".to_string()),
        (Color::rgb(0, 100, 0), "darkgreen".to_string()),
        (Color::rgb(169, 169, 169), "darkgrey".to_string()),
        (Color::rgb(189, 183, 107), "darkkhaki".to_string()),
        (Color::rgb(139, 0, 139), "darkmagenta".to_string()),
        (Color::rgb(85, 107, 47), "darkolivegreen".to_string()),
        (Color::rgb(255, 140, 0), "darkorange".to_string()),
        (Color::rgb(153, 50, 204), "darkorchid".to_string()),
        (Color::rgb(139, 0, 0), "darkred".to_string()),
        (Color::rgb(233, 150, 122), "darksalmon".to_string()),
        (Color::rgb(143, 188, 143), "darkseagreen".to_string()),
        (Color::rgb(72, 61, 139), "darkslateblue".to_string()),
        (Color::rgb(47, 79, 79), "darkslategray".to_string()),
        (Color::rgb(47, 79, 79), "darkslategrey".to_string()),
        (Color::rgb(0, 206, 209), "darkturquoise".to_string()),
        (Color::rgb(148, 0, 211), "darkviolet".to_string()),
        (Color::rgb(255, 20, 147), "deeppink".to_string()),
        (Color::rgb(0, 191, 255), "deepskyblue".to_string()),
        (Color::rgb(105, 105, 105), "dimgray".to_string()),
        (Color::rgb(105, 105, 105), "dimgrey".to_string()),
        (Color::rgb(30, 144, 255), "dodgerblue".to_string()),
        (Color::rgb(178, 34, 34), "firebrick".to_string()),
        (Color::rgb(255, 250, 240), "floralwhite".to_string()),
        (Color::rgb(34, 139, 34), "forestgreen".to_string()),
        (Color::rgb(255, 0, 255), "fuchsia".to_string()),
        (Color::rgb(220, 220, 220), "gainsboro".to_string()),
        (Color::rgb(248, 248, 255), "ghostwhite".to_string()),
        (Color::rgb(255, 215, 0), "gold".to_string()),
        (Color::rgb(218, 165, 32), "goldenrod".to_string()),
        (Color::rgb(128, 128, 128), "gray".to_string()),
        (Color::rgb(0, 128, 0), "green".to_string()),
        (Color::rgb(173, 255, 47), "greenyellow".to_string()),
        (Color::rgb(128, 128, 128), "grey".to_string()),
        (Color::rgb(240, 255, 240), "honeydew".to_string()),
        (Color::rgb(255, 105, 180), "hotpink".to_string()),
        (Color::rgb(205, 92, 92), "indianred".to_string()),
        (Color::rgb(75, 0, 130), "indigo".to_string()),
        (Color::rgb(255, 255, 240), "ivory".to_string()),
        (Color::rgb(240, 230, 140), "khaki".to_string()),
        (Color::rgb(230, 230, 250), "lavender".to_string()),
        (Color::rgb(255, 240, 245), "lavenderblush".to_string()),
        (Color::rgb(124, 252, 0), "lawngreen".to_string()),
        (Color::rgb(255, 250, 205), "lemonchiffon".to_string()),
        (Color::rgb(173, 216, 230), "lightblue".to_string()),
        (Color::rgb(240, 128, 128), "lightcoral".to_string()),
        (Color::rgb(224, 255, 255), "lightcyan".to_string()),
        (
            Color::rgb(250, 250, 210),
            "lightgoldenrodyellow".to_string(),
        ),
        (Color::rgb(211, 211, 211), "lightgray".to_string()),
        (Color::rgb(144, 238, 144), "lightgreen".to_string()),
        (Color::rgb(211, 211, 211), "lightgrey".to_string()),
        (Color::rgb(255, 182, 193), "lightpink".to_string()),
        (Color::rgb(255, 160, 122), "lightsalmon".to_string()),
        (Color::rgb(32, 178, 170), "lightseagreen".to_string()),
        (Color::rgb(135, 206, 250), "lightskyblue".to_string()),
        (Color::rgb(119, 136, 153), "lightslategray".to_string()),
        (Color::rgb(119, 136, 153), "lightslategrey".to_string()),
        (Color::rgb(176, 196, 222), "lightsteelblue".to_string()),
        (Color::rgb(255, 255, 224), "lightyellow".to_string()),
        (Color::rgb(0, 255, 0), "lime".to_string()),
        (Color::rgb(50, 205, 50), "limegreen".to_string()),
        (Color::rgb(250, 240, 230), "linen".to_string()),
        (Color::rgb(255, 0, 255), "magenta".to_string()),
        (Color::rgb(128, 0, 0), "maroon".to_string()),
        (Color::rgb(102, 205, 170), "mediumaquamarine".to_string()),
        (Color::rgb(0, 0, 205), "mediumblue".to_string()),
        (Color::rgb(186, 85, 211), "mediumorchid".to_string()),
        (Color::rgb(147, 112, 219), "mediumpurple".to_string()),
        (Color::rgb(60, 179, 113), "mediumseagreen".to_string()),
        (Color::rgb(123, 104, 238), "mediumslateblue".to_string()),
        (Color::rgb(0, 250, 154), "mediumspringgreen".to_string()),
        (Color::rgb(72, 209, 204), "mediumturquoise".to_string()),
        (Color::rgb(199, 21, 133), "mediumvioletred".to_string()),
        (Color::rgb(25, 25, 112), "midnightblue".to_string()),
        (Color::rgb(245, 255, 250), "mintcream".to_string()),
        (Color::rgb(255, 228, 225), "mistyrose".to_string()),
        (Color::rgb(255, 228, 181), "moccasin".to_string()),
        (Color::rgb(255, 222, 173), "navajowhite".to_string()),
        (Color::rgb(0, 0, 128), "navy".to_string()),
        (Color::rgb(253, 245, 230), "oldlace".to_string()),
        (Color::rgb(128, 128, 0), "olive".to_string()),
        (Color::rgb(107, 142, 35), "olivedrab".to_string()),
        (Color::rgb(255, 165, 0), "orange".to_string()),
        (Color::rgb(255, 69, 0), "orangered".to_string()),
        (Color::rgb(218, 112, 214), "orchid".to_string()),
        (Color::rgb(238, 232, 170), "palegoldenrod".to_string()),
        (Color::rgb(152, 251, 152), "palegreen".to_string()),
        (Color::rgb(175, 238, 238), "paleturquoise".to_string()),
        (Color::rgb(219, 112, 147), "palevioletred".to_string()),
        (Color::rgb(255, 239, 213), "papayawhip".to_string()),
        (Color::rgb(255, 218, 185), "peachpuff".to_string()),
        (Color::rgb(205, 133, 63), "peru".to_string()),
        (Color::rgb(255, 192, 203), "pink".to_string()),
        (Color::rgb(221, 160, 221), "plum".to_string()),
        (Color::rgb(176, 224, 230), "powderblue".to_string()),
        (Color::rgb(128, 0, 128), "purple".to_string()),
        (Color::rgb(255, 0, 0), "red".to_string()),
        (Color::rgb(188, 143, 143), "rosybrown".to_string()),
        (Color::rgb(65, 105, 225), "royalblue".to_string()),
        (Color::rgb(139, 69, 19), "saddlebrown".to_string()),
        (Color::rgb(250, 128, 114), "salmon".to_string()),
        (Color::rgb(244, 164, 96), "sandybrown".to_string()),
        (Color::rgb(46, 139, 87), "seagreen".to_string()),
        (Color::rgb(255, 245, 238), "seashell".to_string()),
        (Color::rgb(160, 82, 45), "sienna".to_string()),
        (Color::rgb(192, 192, 192), "silver".to_string()),
        (Color::rgb(135, 206, 235), "skyblue".to_string()),
        (Color::rgb(106, 90, 205), "slateblue".to_string()),
        (Color::rgb(112, 128, 144), "slategray".to_string()),
        (Color::rgb(112, 128, 144), "slategrey".to_string()),
        (Color::rgb(255, 250, 250), "snow".to_string()),
        (Color::rgb(0, 255, 127), "springgreen".to_string()),
        (Color::rgb(70, 130, 180), "steelblue".to_string()),
        (Color::rgb(210, 180, 140), "tan".to_string()),
        (Color::rgb(0, 128, 128), "teal".to_string()),
        (Color::rgb(216, 191, 216), "thistle".to_string()),
        (Color::rgb(255, 99, 71), "tomato".to_string()),
        (Color::rgb(64, 224, 208), "turquoise".to_string()),
        (Color::rgb(238, 130, 238), "violet".to_string()),
        (Color::rgb(245, 222, 179), "wheat".to_string()),
        (Color::rgb(255, 255, 255), "white".to_string()),
        (Color::rgb(245, 245, 245), "whitesmoke".to_string()),
        (Color::rgb(255, 255, 0), "yellow".to_string()),
        (Color::rgb(154, 205, 50), "yellowgreen".to_string()),
    ])
}

/// Returns the [xkcd color names](https://xkcd.com/color/rgb.txt).
///
/// The names are mapped to their corresponding color.
/// To get the name of a color the [name] function is preferred.
pub fn xkcd_names() -> HashMap<Color, String> {
    HashMap::from([
        (
            Color::from_hex("#acc2d9", AlphaPosition::None).unwrap(),
            "cloudy blue".to_string(),
        ),
        (
            Color::from_hex("#56ae57", AlphaPosition::None).unwrap(),
            "dark pastel green".to_string(),
        ),
        (
            Color::from_hex("#b2996e", AlphaPosition::None).unwrap(),
            "dust".to_string(),
        ),
        (
            Color::from_hex("#a8ff04", AlphaPosition::None).unwrap(),
            "electric lime".to_string(),
        ),
        (
            Color::from_hex("#69d84f", AlphaPosition::None).unwrap(),
            "fresh green".to_string(),
        ),
        (
            Color::from_hex("#894585", AlphaPosition::None).unwrap(),
            "light eggplant".to_string(),
        ),
        (
            Color::from_hex("#70b23f", AlphaPosition::None).unwrap(),
            "nasty green".to_string(),
        ),
        (
            Color::from_hex("#d4ffff", AlphaPosition::None).unwrap(),
            "really light blue".to_string(),
        ),
        (
            Color::from_hex("#65ab7c", AlphaPosition::None).unwrap(),
            "tea".to_string(),
        ),
        (
            Color::from_hex("#952e8f", AlphaPosition::None).unwrap(),
            "warm purple".to_string(),
        ),
        (
            Color::from_hex("#fcfc81", AlphaPosition::None).unwrap(),
            "yellowish tan".to_string(),
        ),
        (
            Color::from_hex("#a5a391", AlphaPosition::None).unwrap(),
            "cement".to_string(),
        ),
        (
            Color::from_hex("#388004", AlphaPosition::None).unwrap(),
            "dark grass green".to_string(),
        ),
        (
            Color::from_hex("#4c9085", AlphaPosition::None).unwrap(),
            "dusty teal".to_string(),
        ),
        (
            Color::from_hex("#5e9b8a", AlphaPosition::None).unwrap(),
            "grey teal".to_string(),
        ),
        (
            Color::from_hex("#efb435", AlphaPosition::None).unwrap(),
            "macaroni and cheese".to_string(),
        ),
        (
            Color::from_hex("#d99b82", AlphaPosition::None).unwrap(),
            "pinkish tan".to_string(),
        ),
        (
            Color::from_hex("#0a5f38", AlphaPosition::None).unwrap(),
            "spruce".to_string(),
        ),
        (
            Color::from_hex("#0c06f7", AlphaPosition::None).unwrap(),
            "strong blue".to_string(),
        ),
        (
            Color::from_hex("#61de2a", AlphaPosition::None).unwrap(),
            "toxic green".to_string(),
        ),
        (
            Color::from_hex("#3778bf", AlphaPosition::None).unwrap(),
            "windows blue".to_string(),
        ),
        (
            Color::from_hex("#2242c7", AlphaPosition::None).unwrap(),
            "blue blue".to_string(),
        ),
        (
            Color::from_hex("#533cc6", AlphaPosition::None).unwrap(),
            "blue with a hint of purple".to_string(),
        ),
        (
            Color::from_hex("#9bb53c", AlphaPosition::None).unwrap(),
            "booger".to_string(),
        ),
        (
            Color::from_hex("#05ffa6", AlphaPosition::None).unwrap(),
            "bright sea green".to_string(),
        ),
        (
            Color::from_hex("#1f6357", AlphaPosition::None).unwrap(),
            "dark green blue".to_string(),
        ),
        (
            Color::from_hex("#017374", AlphaPosition::None).unwrap(),
            "deep turquoise".to_string(),
        ),
        (
            Color::from_hex("#0cb577", AlphaPosition::None).unwrap(),
            "green teal".to_string(),
        ),
        (
            Color::from_hex("#ff0789", AlphaPosition::None).unwrap(),
            "strong pink".to_string(),
        ),
        (
            Color::from_hex("#afa88b", AlphaPosition::None).unwrap(),
            "bland".to_string(),
        ),
        (
            Color::from_hex("#08787f", AlphaPosition::None).unwrap(),
            "deep aqua".to_string(),
        ),
        (
            Color::from_hex("#dd85d7", AlphaPosition::None).unwrap(),
            "lavender pink".to_string(),
        ),
        (
            Color::from_hex("#a6c875", AlphaPosition::None).unwrap(),
            "light moss green".to_string(),
        ),
        (
            Color::from_hex("#a7ffb5", AlphaPosition::None).unwrap(),
            "light seafoam green".to_string(),
        ),
        (
            Color::from_hex("#c2b709", AlphaPosition::None).unwrap(),
            "olive yellow".to_string(),
        ),
        (
            Color::from_hex("#e78ea5", AlphaPosition::None).unwrap(),
            "pig pink".to_string(),
        ),
        (
            Color::from_hex("#966ebd", AlphaPosition::None).unwrap(),
            "deep lilac".to_string(),
        ),
        (
            Color::from_hex("#ccad60", AlphaPosition::None).unwrap(),
            "desert".to_string(),
        ),
        (
            Color::from_hex("#ac86a8", AlphaPosition::None).unwrap(),
            "dusty lavender".to_string(),
        ),
        (
            Color::from_hex("#947e94", AlphaPosition::None).unwrap(),
            "purpley grey".to_string(),
        ),
        (
            Color::from_hex("#983fb2", AlphaPosition::None).unwrap(),
            "purply".to_string(),
        ),
        (
            Color::from_hex("#ff63e9", AlphaPosition::None).unwrap(),
            "candy pink".to_string(),
        ),
        (
            Color::from_hex("#b2fba5", AlphaPosition::None).unwrap(),
            "light pastel green".to_string(),
        ),
        (
            Color::from_hex("#63b365", AlphaPosition::None).unwrap(),
            "boring green".to_string(),
        ),
        (
            Color::from_hex("#8ee53f", AlphaPosition::None).unwrap(),
            "kiwi green".to_string(),
        ),
        (
            Color::from_hex("#b7e1a1", AlphaPosition::None).unwrap(),
            "light grey green".to_string(),
        ),
        (
            Color::from_hex("#ff6f52", AlphaPosition::None).unwrap(),
            "orange pink".to_string(),
        ),
        (
            Color::from_hex("#bdf8a3", AlphaPosition::None).unwrap(),
            "tea green".to_string(),
        ),
        (
            Color::from_hex("#d3b683", AlphaPosition::None).unwrap(),
            "very light brown".to_string(),
        ),
        (
            Color::from_hex("#fffcc4", AlphaPosition::None).unwrap(),
            "egg shell".to_string(),
        ),
        (
            Color::from_hex("#430541", AlphaPosition::None).unwrap(),
            "eggplant purple".to_string(),
        ),
        (
            Color::from_hex("#ffb2d0", AlphaPosition::None).unwrap(),
            "powder pink".to_string(),
        ),
        (
            Color::from_hex("#997570", AlphaPosition::None).unwrap(),
            "reddish grey".to_string(),
        ),
        (
            Color::from_hex("#ad900d", AlphaPosition::None).unwrap(),
            "baby shit brown".to_string(),
        ),
        (
            Color::from_hex("#c48efd", AlphaPosition::None).unwrap(),
            "liliac".to_string(),
        ),
        (
            Color::from_hex("#507b9c", AlphaPosition::None).unwrap(),
            "stormy blue".to_string(),
        ),
        (
            Color::from_hex("#7d7103", AlphaPosition::None).unwrap(),
            "ugly brown".to_string(),
        ),
        (
            Color::from_hex("#fffd78", AlphaPosition::None).unwrap(),
            "custard".to_string(),
        ),
        (
            Color::from_hex("#da467d", AlphaPosition::None).unwrap(),
            "darkish pink".to_string(),
        ),
        (
            Color::from_hex("#410200", AlphaPosition::None).unwrap(),
            "deep brown".to_string(),
        ),
        (
            Color::from_hex("#c9d179", AlphaPosition::None).unwrap(),
            "greenish beige".to_string(),
        ),
        (
            Color::from_hex("#fffa86", AlphaPosition::None).unwrap(),
            "manilla".to_string(),
        ),
        (
            Color::from_hex("#5684ae", AlphaPosition::None).unwrap(),
            "off blue".to_string(),
        ),
        (
            Color::from_hex("#6b7c85", AlphaPosition::None).unwrap(),
            "battleship grey".to_string(),
        ),
        (
            Color::from_hex("#6f6c0a", AlphaPosition::None).unwrap(),
            "browny green".to_string(),
        ),
        (
            Color::from_hex("#7e4071", AlphaPosition::None).unwrap(),
            "bruise".to_string(),
        ),
        (
            Color::from_hex("#009337", AlphaPosition::None).unwrap(),
            "kelley green".to_string(),
        ),
        (
            Color::from_hex("#d0e429", AlphaPosition::None).unwrap(),
            "sickly yellow".to_string(),
        ),
        (
            Color::from_hex("#fff917", AlphaPosition::None).unwrap(),
            "sunny yellow".to_string(),
        ),
        (
            Color::from_hex("#1d5dec", AlphaPosition::None).unwrap(),
            "azul".to_string(),
        ),
        (
            Color::from_hex("#054907", AlphaPosition::None).unwrap(),
            "darkgreen".to_string(),
        ),
        (
            Color::from_hex("#b5ce08", AlphaPosition::None).unwrap(),
            "green/yellow".to_string(),
        ),
        (
            Color::from_hex("#8fb67b", AlphaPosition::None).unwrap(),
            "lichen".to_string(),
        ),
        (
            Color::from_hex("#c8ffb0", AlphaPosition::None).unwrap(),
            "light light green".to_string(),
        ),
        (
            Color::from_hex("#fdde6c", AlphaPosition::None).unwrap(),
            "pale gold".to_string(),
        ),
        (
            Color::from_hex("#ffdf22", AlphaPosition::None).unwrap(),
            "sun yellow".to_string(),
        ),
        (
            Color::from_hex("#a9be70", AlphaPosition::None).unwrap(),
            "tan green".to_string(),
        ),
        (
            Color::from_hex("#6832e3", AlphaPosition::None).unwrap(),
            "burple".to_string(),
        ),
        (
            Color::from_hex("#fdb147", AlphaPosition::None).unwrap(),
            "butterscotch".to_string(),
        ),
        (
            Color::from_hex("#c7ac7d", AlphaPosition::None).unwrap(),
            "toupe".to_string(),
        ),
        (
            Color::from_hex("#fff39a", AlphaPosition::None).unwrap(),
            "dark cream".to_string(),
        ),
        (
            Color::from_hex("#850e04", AlphaPosition::None).unwrap(),
            "indian red".to_string(),
        ),
        (
            Color::from_hex("#efc0fe", AlphaPosition::None).unwrap(),
            "light lavendar".to_string(),
        ),
        (
            Color::from_hex("#40fd14", AlphaPosition::None).unwrap(),
            "poison green".to_string(),
        ),
        (
            Color::from_hex("#b6c406", AlphaPosition::None).unwrap(),
            "baby puke green".to_string(),
        ),
        (
            Color::from_hex("#9dff00", AlphaPosition::None).unwrap(),
            "bright yellow green".to_string(),
        ),
        (
            Color::from_hex("#3c4142", AlphaPosition::None).unwrap(),
            "charcoal grey".to_string(),
        ),
        (
            Color::from_hex("#f2ab15", AlphaPosition::None).unwrap(),
            "squash".to_string(),
        ),
        (
            Color::from_hex("#ac4f06", AlphaPosition::None).unwrap(),
            "cinnamon".to_string(),
        ),
        (
            Color::from_hex("#c4fe82", AlphaPosition::None).unwrap(),
            "light pea green".to_string(),
        ),
        (
            Color::from_hex("#2cfa1f", AlphaPosition::None).unwrap(),
            "radioactive green".to_string(),
        ),
        (
            Color::from_hex("#9a6200", AlphaPosition::None).unwrap(),
            "raw sienna".to_string(),
        ),
        (
            Color::from_hex("#ca9bf7", AlphaPosition::None).unwrap(),
            "baby purple".to_string(),
        ),
        (
            Color::from_hex("#875f42", AlphaPosition::None).unwrap(),
            "cocoa".to_string(),
        ),
        (
            Color::from_hex("#3a2efe", AlphaPosition::None).unwrap(),
            "light royal blue".to_string(),
        ),
        (
            Color::from_hex("#fd8d49", AlphaPosition::None).unwrap(),
            "orangeish".to_string(),
        ),
        (
            Color::from_hex("#8b3103", AlphaPosition::None).unwrap(),
            "rust brown".to_string(),
        ),
        (
            Color::from_hex("#cba560", AlphaPosition::None).unwrap(),
            "sand brown".to_string(),
        ),
        (
            Color::from_hex("#698339", AlphaPosition::None).unwrap(),
            "swamp".to_string(),
        ),
        (
            Color::from_hex("#0cdc73", AlphaPosition::None).unwrap(),
            "tealish green".to_string(),
        ),
        (
            Color::from_hex("#b75203", AlphaPosition::None).unwrap(),
            "burnt siena".to_string(),
        ),
        (
            Color::from_hex("#7f8f4e", AlphaPosition::None).unwrap(),
            "camo".to_string(),
        ),
        (
            Color::from_hex("#26538d", AlphaPosition::None).unwrap(),
            "dusk blue".to_string(),
        ),
        (
            Color::from_hex("#63a950", AlphaPosition::None).unwrap(),
            "fern".to_string(),
        ),
        (
            Color::from_hex("#c87f89", AlphaPosition::None).unwrap(),
            "old rose".to_string(),
        ),
        (
            Color::from_hex("#b1fc99", AlphaPosition::None).unwrap(),
            "pale light green".to_string(),
        ),
        (
            Color::from_hex("#ff9a8a", AlphaPosition::None).unwrap(),
            "peachy pink".to_string(),
        ),
        (
            Color::from_hex("#f6688e", AlphaPosition::None).unwrap(),
            "rosy pink".to_string(),
        ),
        (
            Color::from_hex("#76fda8", AlphaPosition::None).unwrap(),
            "light bluish green".to_string(),
        ),
        (
            Color::from_hex("#53fe5c", AlphaPosition::None).unwrap(),
            "light bright green".to_string(),
        ),
        (
            Color::from_hex("#4efd54", AlphaPosition::None).unwrap(),
            "light neon green".to_string(),
        ),
        (
            Color::from_hex("#a0febf", AlphaPosition::None).unwrap(),
            "light seafoam".to_string(),
        ),
        (
            Color::from_hex("#7bf2da", AlphaPosition::None).unwrap(),
            "tiffany blue".to_string(),
        ),
        (
            Color::from_hex("#bcf5a6", AlphaPosition::None).unwrap(),
            "washed out green".to_string(),
        ),
        (
            Color::from_hex("#ca6b02", AlphaPosition::None).unwrap(),
            "browny orange".to_string(),
        ),
        (
            Color::from_hex("#107ab0", AlphaPosition::None).unwrap(),
            "nice blue".to_string(),
        ),
        (
            Color::from_hex("#2138ab", AlphaPosition::None).unwrap(),
            "sapphire".to_string(),
        ),
        (
            Color::from_hex("#719f91", AlphaPosition::None).unwrap(),
            "greyish teal".to_string(),
        ),
        (
            Color::from_hex("#fdb915", AlphaPosition::None).unwrap(),
            "orangey yellow".to_string(),
        ),
        (
            Color::from_hex("#fefcaf", AlphaPosition::None).unwrap(),
            "parchment".to_string(),
        ),
        (
            Color::from_hex("#fcf679", AlphaPosition::None).unwrap(),
            "straw".to_string(),
        ),
        (
            Color::from_hex("#1d0200", AlphaPosition::None).unwrap(),
            "very dark brown".to_string(),
        ),
        (
            Color::from_hex("#cb6843", AlphaPosition::None).unwrap(),
            "terracota".to_string(),
        ),
        (
            Color::from_hex("#31668a", AlphaPosition::None).unwrap(),
            "ugly blue".to_string(),
        ),
        (
            Color::from_hex("#247afd", AlphaPosition::None).unwrap(),
            "clear blue".to_string(),
        ),
        (
            Color::from_hex("#ffffb6", AlphaPosition::None).unwrap(),
            "creme".to_string(),
        ),
        (
            Color::from_hex("#90fda9", AlphaPosition::None).unwrap(),
            "foam green".to_string(),
        ),
        (
            Color::from_hex("#86a17d", AlphaPosition::None).unwrap(),
            "grey/green".to_string(),
        ),
        (
            Color::from_hex("#fddc5c", AlphaPosition::None).unwrap(),
            "light gold".to_string(),
        ),
        (
            Color::from_hex("#78d1b6", AlphaPosition::None).unwrap(),
            "seafoam blue".to_string(),
        ),
        (
            Color::from_hex("#13bbaf", AlphaPosition::None).unwrap(),
            "topaz".to_string(),
        ),
        (
            Color::from_hex("#fb5ffc", AlphaPosition::None).unwrap(),
            "violet pink".to_string(),
        ),
        (
            Color::from_hex("#20f986", AlphaPosition::None).unwrap(),
            "wintergreen".to_string(),
        ),
        (
            Color::from_hex("#ffe36e", AlphaPosition::None).unwrap(),
            "yellow tan".to_string(),
        ),
        (
            Color::from_hex("#9d0759", AlphaPosition::None).unwrap(),
            "dark fuchsia".to_string(),
        ),
        (
            Color::from_hex("#3a18b1", AlphaPosition::None).unwrap(),
            "indigo blue".to_string(),
        ),
        (
            Color::from_hex("#c2ff89", AlphaPosition::None).unwrap(),
            "light yellowish green".to_string(),
        ),
        (
            Color::from_hex("#d767ad", AlphaPosition::None).unwrap(),
            "pale magenta".to_string(),
        ),
        (
            Color::from_hex("#720058", AlphaPosition::None).unwrap(),
            "rich purple".to_string(),
        ),
        (
            Color::from_hex("#ffda03", AlphaPosition::None).unwrap(),
            "sunflower yellow".to_string(),
        ),
        (
            Color::from_hex("#01c08d", AlphaPosition::None).unwrap(),
            "green/blue".to_string(),
        ),
        (
            Color::from_hex("#ac7434", AlphaPosition::None).unwrap(),
            "leather".to_string(),
        ),
        (
            Color::from_hex("#014600", AlphaPosition::None).unwrap(),
            "racing green".to_string(),
        ),
        (
            Color::from_hex("#9900fa", AlphaPosition::None).unwrap(),
            "vivid purple".to_string(),
        ),
        (
            Color::from_hex("#02066f", AlphaPosition::None).unwrap(),
            "dark royal blue".to_string(),
        ),
        (
            Color::from_hex("#8e7618", AlphaPosition::None).unwrap(),
            "hazel".to_string(),
        ),
        (
            Color::from_hex("#d1768f", AlphaPosition::None).unwrap(),
            "muted pink".to_string(),
        ),
        (
            Color::from_hex("#96b403", AlphaPosition::None).unwrap(),
            "booger green".to_string(),
        ),
        (
            Color::from_hex("#fdff63", AlphaPosition::None).unwrap(),
            "canary".to_string(),
        ),
        (
            Color::from_hex("#95a3a6", AlphaPosition::None).unwrap(),
            "cool grey".to_string(),
        ),
        (
            Color::from_hex("#7f684e", AlphaPosition::None).unwrap(),
            "dark taupe".to_string(),
        ),
        (
            Color::from_hex("#751973", AlphaPosition::None).unwrap(),
            "darkish purple".to_string(),
        ),
        (
            Color::from_hex("#089404", AlphaPosition::None).unwrap(),
            "true green".to_string(),
        ),
        (
            Color::from_hex("#ff6163", AlphaPosition::None).unwrap(),
            "coral pink".to_string(),
        ),
        (
            Color::from_hex("#598556", AlphaPosition::None).unwrap(),
            "dark sage".to_string(),
        ),
        (
            Color::from_hex("#214761", AlphaPosition::None).unwrap(),
            "dark slate blue".to_string(),
        ),
        (
            Color::from_hex("#3c73a8", AlphaPosition::None).unwrap(),
            "flat blue".to_string(),
        ),
        (
            Color::from_hex("#ba9e88", AlphaPosition::None).unwrap(),
            "mushroom".to_string(),
        ),
        (
            Color::from_hex("#021bf9", AlphaPosition::None).unwrap(),
            "rich blue".to_string(),
        ),
        (
            Color::from_hex("#734a65", AlphaPosition::None).unwrap(),
            "dirty purple".to_string(),
        ),
        (
            Color::from_hex("#23c48b", AlphaPosition::None).unwrap(),
            "greenblue".to_string(),
        ),
        (
            Color::from_hex("#8fae22", AlphaPosition::None).unwrap(),
            "icky green".to_string(),
        ),
        (
            Color::from_hex("#e6f2a2", AlphaPosition::None).unwrap(),
            "light khaki".to_string(),
        ),
        (
            Color::from_hex("#4b57db", AlphaPosition::None).unwrap(),
            "warm blue".to_string(),
        ),
        (
            Color::from_hex("#d90166", AlphaPosition::None).unwrap(),
            "dark hot pink".to_string(),
        ),
        (
            Color::from_hex("#015482", AlphaPosition::None).unwrap(),
            "deep sea blue".to_string(),
        ),
        (
            Color::from_hex("#9d0216", AlphaPosition::None).unwrap(),
            "carmine".to_string(),
        ),
        (
            Color::from_hex("#728f02", AlphaPosition::None).unwrap(),
            "dark yellow green".to_string(),
        ),
        (
            Color::from_hex("#ffe5ad", AlphaPosition::None).unwrap(),
            "pale peach".to_string(),
        ),
        (
            Color::from_hex("#4e0550", AlphaPosition::None).unwrap(),
            "plum purple".to_string(),
        ),
        (
            Color::from_hex("#f9bc08", AlphaPosition::None).unwrap(),
            "golden rod".to_string(),
        ),
        (
            Color::from_hex("#ff073a", AlphaPosition::None).unwrap(),
            "neon red".to_string(),
        ),
        (
            Color::from_hex("#c77986", AlphaPosition::None).unwrap(),
            "old pink".to_string(),
        ),
        (
            Color::from_hex("#d6fffe", AlphaPosition::None).unwrap(),
            "very pale blue".to_string(),
        ),
        (
            Color::from_hex("#fe4b03", AlphaPosition::None).unwrap(),
            "blood orange".to_string(),
        ),
        (
            Color::from_hex("#fd5956", AlphaPosition::None).unwrap(),
            "grapefruit".to_string(),
        ),
        (
            Color::from_hex("#fce166", AlphaPosition::None).unwrap(),
            "sand yellow".to_string(),
        ),
        (
            Color::from_hex("#b2713d", AlphaPosition::None).unwrap(),
            "clay brown".to_string(),
        ),
        (
            Color::from_hex("#1f3b4d", AlphaPosition::None).unwrap(),
            "dark blue grey".to_string(),
        ),
        (
            Color::from_hex("#699d4c", AlphaPosition::None).unwrap(),
            "flat green".to_string(),
        ),
        (
            Color::from_hex("#56fca2", AlphaPosition::None).unwrap(),
            "light green blue".to_string(),
        ),
        (
            Color::from_hex("#fb5581", AlphaPosition::None).unwrap(),
            "warm pink".to_string(),
        ),
        (
            Color::from_hex("#3e82fc", AlphaPosition::None).unwrap(),
            "dodger blue".to_string(),
        ),
        (
            Color::from_hex("#a0bf16", AlphaPosition::None).unwrap(),
            "gross green".to_string(),
        ),
        (
            Color::from_hex("#d6fffa", AlphaPosition::None).unwrap(),
            "ice".to_string(),
        ),
        (
            Color::from_hex("#4f738e", AlphaPosition::None).unwrap(),
            "metallic blue".to_string(),
        ),
        (
            Color::from_hex("#ffb19a", AlphaPosition::None).unwrap(),
            "pale salmon".to_string(),
        ),
        (
            Color::from_hex("#5c8b15", AlphaPosition::None).unwrap(),
            "sap green".to_string(),
        ),
        (
            Color::from_hex("#54ac68", AlphaPosition::None).unwrap(),
            "algae".to_string(),
        ),
        (
            Color::from_hex("#89a0b0", AlphaPosition::None).unwrap(),
            "bluey grey".to_string(),
        ),
        (
            Color::from_hex("#7ea07a", AlphaPosition::None).unwrap(),
            "greeny grey".to_string(),
        ),
        (
            Color::from_hex("#1bfc06", AlphaPosition::None).unwrap(),
            "highlighter green".to_string(),
        ),
        (
            Color::from_hex("#cafffb", AlphaPosition::None).unwrap(),
            "light light blue".to_string(),
        ),
        (
            Color::from_hex("#b6ffbb", AlphaPosition::None).unwrap(),
            "light mint".to_string(),
        ),
        (
            Color::from_hex("#a75e09", AlphaPosition::None).unwrap(),
            "raw umber".to_string(),
        ),
        (
            Color::from_hex("#152eff", AlphaPosition::None).unwrap(),
            "vivid blue".to_string(),
        ),
        (
            Color::from_hex("#8d5eb7", AlphaPosition::None).unwrap(),
            "deep lavender".to_string(),
        ),
        (
            Color::from_hex("#5f9e8f", AlphaPosition::None).unwrap(),
            "dull teal".to_string(),
        ),
        (
            Color::from_hex("#63f7b4", AlphaPosition::None).unwrap(),
            "light greenish blue".to_string(),
        ),
        (
            Color::from_hex("#606602", AlphaPosition::None).unwrap(),
            "mud green".to_string(),
        ),
        (
            Color::from_hex("#fc86aa", AlphaPosition::None).unwrap(),
            "pinky".to_string(),
        ),
        (
            Color::from_hex("#8c0034", AlphaPosition::None).unwrap(),
            "red wine".to_string(),
        ),
        (
            Color::from_hex("#758000", AlphaPosition::None).unwrap(),
            "shit green".to_string(),
        ),
        (
            Color::from_hex("#ab7e4c", AlphaPosition::None).unwrap(),
            "tan brown".to_string(),
        ),
        (
            Color::from_hex("#030764", AlphaPosition::None).unwrap(),
            "darkblue".to_string(),
        ),
        (
            Color::from_hex("#fe86a4", AlphaPosition::None).unwrap(),
            "rosa".to_string(),
        ),
        (
            Color::from_hex("#d5174e", AlphaPosition::None).unwrap(),
            "lipstick".to_string(),
        ),
        (
            Color::from_hex("#fed0fc", AlphaPosition::None).unwrap(),
            "pale mauve".to_string(),
        ),
        (
            Color::from_hex("#680018", AlphaPosition::None).unwrap(),
            "claret".to_string(),
        ),
        (
            Color::from_hex("#fedf08", AlphaPosition::None).unwrap(),
            "dandelion".to_string(),
        ),
        (
            Color::from_hex("#fe420f", AlphaPosition::None).unwrap(),
            "orangered".to_string(),
        ),
        (
            Color::from_hex("#6f7c00", AlphaPosition::None).unwrap(),
            "poop green".to_string(),
        ),
        (
            Color::from_hex("#ca0147", AlphaPosition::None).unwrap(),
            "ruby".to_string(),
        ),
        (
            Color::from_hex("#1b2431", AlphaPosition::None).unwrap(),
            "dark".to_string(),
        ),
        (
            Color::from_hex("#00fbb0", AlphaPosition::None).unwrap(),
            "greenish turquoise".to_string(),
        ),
        (
            Color::from_hex("#db5856", AlphaPosition::None).unwrap(),
            "pastel red".to_string(),
        ),
        (
            Color::from_hex("#ddd618", AlphaPosition::None).unwrap(),
            "piss yellow".to_string(),
        ),
        (
            Color::from_hex("#41fdfe", AlphaPosition::None).unwrap(),
            "bright cyan".to_string(),
        ),
        (
            Color::from_hex("#cf524e", AlphaPosition::None).unwrap(),
            "dark coral".to_string(),
        ),
        (
            Color::from_hex("#21c36f", AlphaPosition::None).unwrap(),
            "algae green".to_string(),
        ),
        (
            Color::from_hex("#a90308", AlphaPosition::None).unwrap(),
            "darkish red".to_string(),
        ),
        (
            Color::from_hex("#6e1005", AlphaPosition::None).unwrap(),
            "reddy brown".to_string(),
        ),
        (
            Color::from_hex("#fe828c", AlphaPosition::None).unwrap(),
            "blush pink".to_string(),
        ),
        (
            Color::from_hex("#4b6113", AlphaPosition::None).unwrap(),
            "camouflage green".to_string(),
        ),
        (
            Color::from_hex("#4da409", AlphaPosition::None).unwrap(),
            "lawn green".to_string(),
        ),
        (
            Color::from_hex("#beae8a", AlphaPosition::None).unwrap(),
            "putty".to_string(),
        ),
        (
            Color::from_hex("#0339f8", AlphaPosition::None).unwrap(),
            "vibrant blue".to_string(),
        ),
        (
            Color::from_hex("#a88f59", AlphaPosition::None).unwrap(),
            "dark sand".to_string(),
        ),
        (
            Color::from_hex("#5d21d0", AlphaPosition::None).unwrap(),
            "purple/blue".to_string(),
        ),
        (
            Color::from_hex("#feb209", AlphaPosition::None).unwrap(),
            "saffron".to_string(),
        ),
        (
            Color::from_hex("#4e518b", AlphaPosition::None).unwrap(),
            "twilight".to_string(),
        ),
        (
            Color::from_hex("#964e02", AlphaPosition::None).unwrap(),
            "warm brown".to_string(),
        ),
        (
            Color::from_hex("#85a3b2", AlphaPosition::None).unwrap(),
            "bluegrey".to_string(),
        ),
        (
            Color::from_hex("#ff69af", AlphaPosition::None).unwrap(),
            "bubble gum pink".to_string(),
        ),
        (
            Color::from_hex("#c3fbf4", AlphaPosition::None).unwrap(),
            "duck egg blue".to_string(),
        ),
        (
            Color::from_hex("#2afeb7", AlphaPosition::None).unwrap(),
            "greenish cyan".to_string(),
        ),
        (
            Color::from_hex("#005f6a", AlphaPosition::None).unwrap(),
            "petrol".to_string(),
        ),
        (
            Color::from_hex("#0c1793", AlphaPosition::None).unwrap(),
            "royal".to_string(),
        ),
        (
            Color::from_hex("#ffff81", AlphaPosition::None).unwrap(),
            "butter".to_string(),
        ),
        (
            Color::from_hex("#f0833a", AlphaPosition::None).unwrap(),
            "dusty orange".to_string(),
        ),
        (
            Color::from_hex("#f1f33f", AlphaPosition::None).unwrap(),
            "off yellow".to_string(),
        ),
        (
            Color::from_hex("#b1d27b", AlphaPosition::None).unwrap(),
            "pale olive green".to_string(),
        ),
        (
            Color::from_hex("#fc824a", AlphaPosition::None).unwrap(),
            "orangish".to_string(),
        ),
        (
            Color::from_hex("#71aa34", AlphaPosition::None).unwrap(),
            "leaf".to_string(),
        ),
        (
            Color::from_hex("#b7c9e2", AlphaPosition::None).unwrap(),
            "light blue grey".to_string(),
        ),
        (
            Color::from_hex("#4b0101", AlphaPosition::None).unwrap(),
            "dried blood".to_string(),
        ),
        (
            Color::from_hex("#a552e6", AlphaPosition::None).unwrap(),
            "lightish purple".to_string(),
        ),
        (
            Color::from_hex("#af2f0d", AlphaPosition::None).unwrap(),
            "rusty red".to_string(),
        ),
        (
            Color::from_hex("#8b88f8", AlphaPosition::None).unwrap(),
            "lavender blue".to_string(),
        ),
        (
            Color::from_hex("#9af764", AlphaPosition::None).unwrap(),
            "light grass green".to_string(),
        ),
        (
            Color::from_hex("#a6fbb2", AlphaPosition::None).unwrap(),
            "light mint green".to_string(),
        ),
        (
            Color::from_hex("#ffc512", AlphaPosition::None).unwrap(),
            "sunflower".to_string(),
        ),
        (
            Color::from_hex("#750851", AlphaPosition::None).unwrap(),
            "velvet".to_string(),
        ),
        (
            Color::from_hex("#c14a09", AlphaPosition::None).unwrap(),
            "brick orange".to_string(),
        ),
        (
            Color::from_hex("#fe2f4a", AlphaPosition::None).unwrap(),
            "lightish red".to_string(),
        ),
        (
            Color::from_hex("#0203e2", AlphaPosition::None).unwrap(),
            "pure blue".to_string(),
        ),
        (
            Color::from_hex("#0a437a", AlphaPosition::None).unwrap(),
            "twilight blue".to_string(),
        ),
        (
            Color::from_hex("#a50055", AlphaPosition::None).unwrap(),
            "violet red".to_string(),
        ),
        (
            Color::from_hex("#ae8b0c", AlphaPosition::None).unwrap(),
            "yellowy brown".to_string(),
        ),
        (
            Color::from_hex("#fd798f", AlphaPosition::None).unwrap(),
            "carnation".to_string(),
        ),
        (
            Color::from_hex("#bfac05", AlphaPosition::None).unwrap(),
            "muddy yellow".to_string(),
        ),
        (
            Color::from_hex("#3eaf76", AlphaPosition::None).unwrap(),
            "dark seafoam green".to_string(),
        ),
        (
            Color::from_hex("#c74767", AlphaPosition::None).unwrap(),
            "deep rose".to_string(),
        ),
        (
            Color::from_hex("#b9484e", AlphaPosition::None).unwrap(),
            "dusty red".to_string(),
        ),
        (
            Color::from_hex("#647d8e", AlphaPosition::None).unwrap(),
            "grey/blue".to_string(),
        ),
        (
            Color::from_hex("#bffe28", AlphaPosition::None).unwrap(),
            "lemon lime".to_string(),
        ),
        (
            Color::from_hex("#d725de", AlphaPosition::None).unwrap(),
            "purple/pink".to_string(),
        ),
        (
            Color::from_hex("#b29705", AlphaPosition::None).unwrap(),
            "brown yellow".to_string(),
        ),
        (
            Color::from_hex("#673a3f", AlphaPosition::None).unwrap(),
            "purple brown".to_string(),
        ),
        (
            Color::from_hex("#a87dc2", AlphaPosition::None).unwrap(),
            "wisteria".to_string(),
        ),
        (
            Color::from_hex("#fafe4b", AlphaPosition::None).unwrap(),
            "banana yellow".to_string(),
        ),
        (
            Color::from_hex("#c0022f", AlphaPosition::None).unwrap(),
            "lipstick red".to_string(),
        ),
        (
            Color::from_hex("#0e87cc", AlphaPosition::None).unwrap(),
            "water blue".to_string(),
        ),
        (
            Color::from_hex("#8d8468", AlphaPosition::None).unwrap(),
            "brown grey".to_string(),
        ),
        (
            Color::from_hex("#ad03de", AlphaPosition::None).unwrap(),
            "vibrant purple".to_string(),
        ),
        (
            Color::from_hex("#8cff9e", AlphaPosition::None).unwrap(),
            "baby green".to_string(),
        ),
        (
            Color::from_hex("#94ac02", AlphaPosition::None).unwrap(),
            "barf green".to_string(),
        ),
        (
            Color::from_hex("#c4fff7", AlphaPosition::None).unwrap(),
            "eggshell blue".to_string(),
        ),
        (
            Color::from_hex("#fdee73", AlphaPosition::None).unwrap(),
            "sandy yellow".to_string(),
        ),
        (
            Color::from_hex("#33b864", AlphaPosition::None).unwrap(),
            "cool green".to_string(),
        ),
        (
            Color::from_hex("#fff9d0", AlphaPosition::None).unwrap(),
            "pale".to_string(),
        ),
        (
            Color::from_hex("#758da3", AlphaPosition::None).unwrap(),
            "blue/grey".to_string(),
        ),
        (
            Color::from_hex("#f504c9", AlphaPosition::None).unwrap(),
            "hot magenta".to_string(),
        ),
        (
            Color::from_hex("#77a1b5", AlphaPosition::None).unwrap(),
            "greyblue".to_string(),
        ),
        (
            Color::from_hex("#8756e4", AlphaPosition::None).unwrap(),
            "purpley".to_string(),
        ),
        (
            Color::from_hex("#889717", AlphaPosition::None).unwrap(),
            "baby shit green".to_string(),
        ),
        (
            Color::from_hex("#c27e79", AlphaPosition::None).unwrap(),
            "brownish pink".to_string(),
        ),
        (
            Color::from_hex("#017371", AlphaPosition::None).unwrap(),
            "dark aquamarine".to_string(),
        ),
        (
            Color::from_hex("#9f8303", AlphaPosition::None).unwrap(),
            "diarrhea".to_string(),
        ),
        (
            Color::from_hex("#f7d560", AlphaPosition::None).unwrap(),
            "light mustard".to_string(),
        ),
        (
            Color::from_hex("#bdf6fe", AlphaPosition::None).unwrap(),
            "pale sky blue".to_string(),
        ),
        (
            Color::from_hex("#75b84f", AlphaPosition::None).unwrap(),
            "turtle green".to_string(),
        ),
        (
            Color::from_hex("#9cbb04", AlphaPosition::None).unwrap(),
            "bright olive".to_string(),
        ),
        (
            Color::from_hex("#29465b", AlphaPosition::None).unwrap(),
            "dark grey blue".to_string(),
        ),
        (
            Color::from_hex("#696006", AlphaPosition::None).unwrap(),
            "greeny brown".to_string(),
        ),
        (
            Color::from_hex("#adf802", AlphaPosition::None).unwrap(),
            "lemon green".to_string(),
        ),
        (
            Color::from_hex("#c1c6fc", AlphaPosition::None).unwrap(),
            "light periwinkle".to_string(),
        ),
        (
            Color::from_hex("#35ad6b", AlphaPosition::None).unwrap(),
            "seaweed green".to_string(),
        ),
        (
            Color::from_hex("#fffd37", AlphaPosition::None).unwrap(),
            "sunshine yellow".to_string(),
        ),
        (
            Color::from_hex("#a442a0", AlphaPosition::None).unwrap(),
            "ugly purple".to_string(),
        ),
        (
            Color::from_hex("#f36196", AlphaPosition::None).unwrap(),
            "medium pink".to_string(),
        ),
        (
            Color::from_hex("#947706", AlphaPosition::None).unwrap(),
            "puke brown".to_string(),
        ),
        (
            Color::from_hex("#fff4f2", AlphaPosition::None).unwrap(),
            "very light pink".to_string(),
        ),
        (
            Color::from_hex("#1e9167", AlphaPosition::None).unwrap(),
            "viridian".to_string(),
        ),
        (
            Color::from_hex("#b5c306", AlphaPosition::None).unwrap(),
            "bile".to_string(),
        ),
        (
            Color::from_hex("#feff7f", AlphaPosition::None).unwrap(),
            "faded yellow".to_string(),
        ),
        (
            Color::from_hex("#cffdbc", AlphaPosition::None).unwrap(),
            "very pale green".to_string(),
        ),
        (
            Color::from_hex("#0add08", AlphaPosition::None).unwrap(),
            "vibrant green".to_string(),
        ),
        (
            Color::from_hex("#87fd05", AlphaPosition::None).unwrap(),
            "bright lime".to_string(),
        ),
        (
            Color::from_hex("#1ef876", AlphaPosition::None).unwrap(),
            "spearmint".to_string(),
        ),
        (
            Color::from_hex("#7bfdc7", AlphaPosition::None).unwrap(),
            "light aquamarine".to_string(),
        ),
        (
            Color::from_hex("#bcecac", AlphaPosition::None).unwrap(),
            "light sage".to_string(),
        ),
        (
            Color::from_hex("#bbf90f", AlphaPosition::None).unwrap(),
            "yellowgreen".to_string(),
        ),
        (
            Color::from_hex("#ab9004", AlphaPosition::None).unwrap(),
            "baby poo".to_string(),
        ),
        (
            Color::from_hex("#1fb57a", AlphaPosition::None).unwrap(),
            "dark seafoam".to_string(),
        ),
        (
            Color::from_hex("#00555a", AlphaPosition::None).unwrap(),
            "deep teal".to_string(),
        ),
        (
            Color::from_hex("#a484ac", AlphaPosition::None).unwrap(),
            "heather".to_string(),
        ),
        (
            Color::from_hex("#c45508", AlphaPosition::None).unwrap(),
            "rust orange".to_string(),
        ),
        (
            Color::from_hex("#3f829d", AlphaPosition::None).unwrap(),
            "dirty blue".to_string(),
        ),
        (
            Color::from_hex("#548d44", AlphaPosition::None).unwrap(),
            "fern green".to_string(),
        ),
        (
            Color::from_hex("#c95efb", AlphaPosition::None).unwrap(),
            "bright lilac".to_string(),
        ),
        (
            Color::from_hex("#3ae57f", AlphaPosition::None).unwrap(),
            "weird green".to_string(),
        ),
        (
            Color::from_hex("#016795", AlphaPosition::None).unwrap(),
            "peacock blue".to_string(),
        ),
        (
            Color::from_hex("#87a922", AlphaPosition::None).unwrap(),
            "avocado green".to_string(),
        ),
        (
            Color::from_hex("#f0944d", AlphaPosition::None).unwrap(),
            "faded orange".to_string(),
        ),
        (
            Color::from_hex("#5d1451", AlphaPosition::None).unwrap(),
            "grape purple".to_string(),
        ),
        (
            Color::from_hex("#25ff29", AlphaPosition::None).unwrap(),
            "hot green".to_string(),
        ),
        (
            Color::from_hex("#d0fe1d", AlphaPosition::None).unwrap(),
            "lime yellow".to_string(),
        ),
        (
            Color::from_hex("#ffa62b", AlphaPosition::None).unwrap(),
            "mango".to_string(),
        ),
        (
            Color::from_hex("#01b44c", AlphaPosition::None).unwrap(),
            "shamrock".to_string(),
        ),
        (
            Color::from_hex("#ff6cb5", AlphaPosition::None).unwrap(),
            "bubblegum".to_string(),
        ),
        (
            Color::from_hex("#6b4247", AlphaPosition::None).unwrap(),
            "purplish brown".to_string(),
        ),
        (
            Color::from_hex("#c7c10c", AlphaPosition::None).unwrap(),
            "vomit yellow".to_string(),
        ),
        (
            Color::from_hex("#b7fffa", AlphaPosition::None).unwrap(),
            "pale cyan".to_string(),
        ),
        (
            Color::from_hex("#aeff6e", AlphaPosition::None).unwrap(),
            "key lime".to_string(),
        ),
        (
            Color::from_hex("#ec2d01", AlphaPosition::None).unwrap(),
            "tomato red".to_string(),
        ),
        (
            Color::from_hex("#76ff7b", AlphaPosition::None).unwrap(),
            "lightgreen".to_string(),
        ),
        (
            Color::from_hex("#730039", AlphaPosition::None).unwrap(),
            "merlot".to_string(),
        ),
        (
            Color::from_hex("#040348", AlphaPosition::None).unwrap(),
            "night blue".to_string(),
        ),
        (
            Color::from_hex("#df4ec8", AlphaPosition::None).unwrap(),
            "purpleish pink".to_string(),
        ),
        (
            Color::from_hex("#6ecb3c", AlphaPosition::None).unwrap(),
            "apple".to_string(),
        ),
        (
            Color::from_hex("#8f9805", AlphaPosition::None).unwrap(),
            "baby poop green".to_string(),
        ),
        (
            Color::from_hex("#5edc1f", AlphaPosition::None).unwrap(),
            "green apple".to_string(),
        ),
        (
            Color::from_hex("#d94ff5", AlphaPosition::None).unwrap(),
            "heliotrope".to_string(),
        ),
        (
            Color::from_hex("#c8fd3d", AlphaPosition::None).unwrap(),
            "yellow/green".to_string(),
        ),
        (
            Color::from_hex("#070d0d", AlphaPosition::None).unwrap(),
            "almost black".to_string(),
        ),
        (
            Color::from_hex("#4984b8", AlphaPosition::None).unwrap(),
            "cool blue".to_string(),
        ),
        (
            Color::from_hex("#51b73b", AlphaPosition::None).unwrap(),
            "leafy green".to_string(),
        ),
        (
            Color::from_hex("#ac7e04", AlphaPosition::None).unwrap(),
            "mustard brown".to_string(),
        ),
        (
            Color::from_hex("#4e5481", AlphaPosition::None).unwrap(),
            "dusk".to_string(),
        ),
        (
            Color::from_hex("#876e4b", AlphaPosition::None).unwrap(),
            "dull brown".to_string(),
        ),
        (
            Color::from_hex("#58bc08", AlphaPosition::None).unwrap(),
            "frog green".to_string(),
        ),
        (
            Color::from_hex("#2fef10", AlphaPosition::None).unwrap(),
            "vivid green".to_string(),
        ),
        (
            Color::from_hex("#2dfe54", AlphaPosition::None).unwrap(),
            "bright light green".to_string(),
        ),
        (
            Color::from_hex("#0aff02", AlphaPosition::None).unwrap(),
            "fluro green".to_string(),
        ),
        (
            Color::from_hex("#9cef43", AlphaPosition::None).unwrap(),
            "kiwi".to_string(),
        ),
        (
            Color::from_hex("#18d17b", AlphaPosition::None).unwrap(),
            "seaweed".to_string(),
        ),
        (
            Color::from_hex("#35530a", AlphaPosition::None).unwrap(),
            "navy green".to_string(),
        ),
        (
            Color::from_hex("#1805db", AlphaPosition::None).unwrap(),
            "ultramarine blue".to_string(),
        ),
        (
            Color::from_hex("#6258c4", AlphaPosition::None).unwrap(),
            "iris".to_string(),
        ),
        (
            Color::from_hex("#ff964f", AlphaPosition::None).unwrap(),
            "pastel orange".to_string(),
        ),
        (
            Color::from_hex("#ffab0f", AlphaPosition::None).unwrap(),
            "yellowish orange".to_string(),
        ),
        (
            Color::from_hex("#8f8ce7", AlphaPosition::None).unwrap(),
            "perrywinkle".to_string(),
        ),
        (
            Color::from_hex("#24bca8", AlphaPosition::None).unwrap(),
            "tealish".to_string(),
        ),
        (
            Color::from_hex("#3f012c", AlphaPosition::None).unwrap(),
            "dark plum".to_string(),
        ),
        (
            Color::from_hex("#cbf85f", AlphaPosition::None).unwrap(),
            "pear".to_string(),
        ),
        (
            Color::from_hex("#ff724c", AlphaPosition::None).unwrap(),
            "pinkish orange".to_string(),
        ),
        (
            Color::from_hex("#280137", AlphaPosition::None).unwrap(),
            "midnight purple".to_string(),
        ),
        (
            Color::from_hex("#b36ff6", AlphaPosition::None).unwrap(),
            "light urple".to_string(),
        ),
        (
            Color::from_hex("#48c072", AlphaPosition::None).unwrap(),
            "dark mint".to_string(),
        ),
        (
            Color::from_hex("#bccb7a", AlphaPosition::None).unwrap(),
            "greenish tan".to_string(),
        ),
        (
            Color::from_hex("#a8415b", AlphaPosition::None).unwrap(),
            "light burgundy".to_string(),
        ),
        (
            Color::from_hex("#06b1c4", AlphaPosition::None).unwrap(),
            "turquoise blue".to_string(),
        ),
        (
            Color::from_hex("#cd7584", AlphaPosition::None).unwrap(),
            "ugly pink".to_string(),
        ),
        (
            Color::from_hex("#f1da7a", AlphaPosition::None).unwrap(),
            "sandy".to_string(),
        ),
        (
            Color::from_hex("#ff0490", AlphaPosition::None).unwrap(),
            "electric pink".to_string(),
        ),
        (
            Color::from_hex("#805b87", AlphaPosition::None).unwrap(),
            "muted purple".to_string(),
        ),
        (
            Color::from_hex("#50a747", AlphaPosition::None).unwrap(),
            "mid green".to_string(),
        ),
        (
            Color::from_hex("#a8a495", AlphaPosition::None).unwrap(),
            "greyish".to_string(),
        ),
        (
            Color::from_hex("#cfff04", AlphaPosition::None).unwrap(),
            "neon yellow".to_string(),
        ),
        (
            Color::from_hex("#ffff7e", AlphaPosition::None).unwrap(),
            "banana".to_string(),
        ),
        (
            Color::from_hex("#ff7fa7", AlphaPosition::None).unwrap(),
            "carnation pink".to_string(),
        ),
        (
            Color::from_hex("#ef4026", AlphaPosition::None).unwrap(),
            "tomato".to_string(),
        ),
        (
            Color::from_hex("#3c9992", AlphaPosition::None).unwrap(),
            "sea".to_string(),
        ),
        (
            Color::from_hex("#886806", AlphaPosition::None).unwrap(),
            "muddy brown".to_string(),
        ),
        (
            Color::from_hex("#04f489", AlphaPosition::None).unwrap(),
            "turquoise green".to_string(),
        ),
        (
            Color::from_hex("#fef69e", AlphaPosition::None).unwrap(),
            "buff".to_string(),
        ),
        (
            Color::from_hex("#cfaf7b", AlphaPosition::None).unwrap(),
            "fawn".to_string(),
        ),
        (
            Color::from_hex("#3b719f", AlphaPosition::None).unwrap(),
            "muted blue".to_string(),
        ),
        (
            Color::from_hex("#fdc1c5", AlphaPosition::None).unwrap(),
            "pale rose".to_string(),
        ),
        (
            Color::from_hex("#20c073", AlphaPosition::None).unwrap(),
            "dark mint green".to_string(),
        ),
        (
            Color::from_hex("#9b5fc0", AlphaPosition::None).unwrap(),
            "amethyst".to_string(),
        ),
        (
            Color::from_hex("#0f9b8e", AlphaPosition::None).unwrap(),
            "blue/green".to_string(),
        ),
        (
            Color::from_hex("#742802", AlphaPosition::None).unwrap(),
            "chestnut".to_string(),
        ),
        (
            Color::from_hex("#9db92c", AlphaPosition::None).unwrap(),
            "sick green".to_string(),
        ),
        (
            Color::from_hex("#a4bf20", AlphaPosition::None).unwrap(),
            "pea".to_string(),
        ),
        (
            Color::from_hex("#cd5909", AlphaPosition::None).unwrap(),
            "rusty orange".to_string(),
        ),
        (
            Color::from_hex("#ada587", AlphaPosition::None).unwrap(),
            "stone".to_string(),
        ),
        (
            Color::from_hex("#be013c", AlphaPosition::None).unwrap(),
            "rose red".to_string(),
        ),
        (
            Color::from_hex("#b8ffeb", AlphaPosition::None).unwrap(),
            "pale aqua".to_string(),
        ),
        (
            Color::from_hex("#dc4d01", AlphaPosition::None).unwrap(),
            "deep orange".to_string(),
        ),
        (
            Color::from_hex("#a2653e", AlphaPosition::None).unwrap(),
            "earth".to_string(),
        ),
        (
            Color::from_hex("#638b27", AlphaPosition::None).unwrap(),
            "mossy green".to_string(),
        ),
        (
            Color::from_hex("#419c03", AlphaPosition::None).unwrap(),
            "grassy green".to_string(),
        ),
        (
            Color::from_hex("#b1ff65", AlphaPosition::None).unwrap(),
            "pale lime green".to_string(),
        ),
        (
            Color::from_hex("#9dbcd4", AlphaPosition::None).unwrap(),
            "light grey blue".to_string(),
        ),
        (
            Color::from_hex("#fdfdfe", AlphaPosition::None).unwrap(),
            "pale grey".to_string(),
        ),
        (
            Color::from_hex("#77ab56", AlphaPosition::None).unwrap(),
            "asparagus".to_string(),
        ),
        (
            Color::from_hex("#464196", AlphaPosition::None).unwrap(),
            "blueberry".to_string(),
        ),
        (
            Color::from_hex("#990147", AlphaPosition::None).unwrap(),
            "purple red".to_string(),
        ),
        (
            Color::from_hex("#befd73", AlphaPosition::None).unwrap(),
            "pale lime".to_string(),
        ),
        (
            Color::from_hex("#32bf84", AlphaPosition::None).unwrap(),
            "greenish teal".to_string(),
        ),
        (
            Color::from_hex("#af6f09", AlphaPosition::None).unwrap(),
            "caramel".to_string(),
        ),
        (
            Color::from_hex("#a0025c", AlphaPosition::None).unwrap(),
            "deep magenta".to_string(),
        ),
        (
            Color::from_hex("#ffd8b1", AlphaPosition::None).unwrap(),
            "light peach".to_string(),
        ),
        (
            Color::from_hex("#7f4e1e", AlphaPosition::None).unwrap(),
            "milk chocolate".to_string(),
        ),
        (
            Color::from_hex("#bf9b0c", AlphaPosition::None).unwrap(),
            "ocher".to_string(),
        ),
        (
            Color::from_hex("#6ba353", AlphaPosition::None).unwrap(),
            "off green".to_string(),
        ),
        (
            Color::from_hex("#f075e6", AlphaPosition::None).unwrap(),
            "purply pink".to_string(),
        ),
        (
            Color::from_hex("#7bc8f6", AlphaPosition::None).unwrap(),
            "lightblue".to_string(),
        ),
        (
            Color::from_hex("#475f94", AlphaPosition::None).unwrap(),
            "dusky blue".to_string(),
        ),
        (
            Color::from_hex("#f5bf03", AlphaPosition::None).unwrap(),
            "golden".to_string(),
        ),
        (
            Color::from_hex("#fffeb6", AlphaPosition::None).unwrap(),
            "light beige".to_string(),
        ),
        (
            Color::from_hex("#fffd74", AlphaPosition::None).unwrap(),
            "butter yellow".to_string(),
        ),
        (
            Color::from_hex("#895b7b", AlphaPosition::None).unwrap(),
            "dusky purple".to_string(),
        ),
        (
            Color::from_hex("#436bad", AlphaPosition::None).unwrap(),
            "french blue".to_string(),
        ),
        (
            Color::from_hex("#d0c101", AlphaPosition::None).unwrap(),
            "ugly yellow".to_string(),
        ),
        (
            Color::from_hex("#c6f808", AlphaPosition::None).unwrap(),
            "greeny yellow".to_string(),
        ),
        (
            Color::from_hex("#f43605", AlphaPosition::None).unwrap(),
            "orangish red".to_string(),
        ),
        (
            Color::from_hex("#02c14d", AlphaPosition::None).unwrap(),
            "shamrock green".to_string(),
        ),
        (
            Color::from_hex("#b25f03", AlphaPosition::None).unwrap(),
            "orangish brown".to_string(),
        ),
        (
            Color::from_hex("#2a7e19", AlphaPosition::None).unwrap(),
            "tree green".to_string(),
        ),
        (
            Color::from_hex("#490648", AlphaPosition::None).unwrap(),
            "deep violet".to_string(),
        ),
        (
            Color::from_hex("#536267", AlphaPosition::None).unwrap(),
            "gunmetal".to_string(),
        ),
        (
            Color::from_hex("#5a06ef", AlphaPosition::None).unwrap(),
            "blue/purple".to_string(),
        ),
        (
            Color::from_hex("#cf0234", AlphaPosition::None).unwrap(),
            "cherry".to_string(),
        ),
        (
            Color::from_hex("#c4a661", AlphaPosition::None).unwrap(),
            "sandy brown".to_string(),
        ),
        (
            Color::from_hex("#978a84", AlphaPosition::None).unwrap(),
            "warm grey".to_string(),
        ),
        (
            Color::from_hex("#1f0954", AlphaPosition::None).unwrap(),
            "dark indigo".to_string(),
        ),
        (
            Color::from_hex("#03012d", AlphaPosition::None).unwrap(),
            "midnight".to_string(),
        ),
        (
            Color::from_hex("#2bb179", AlphaPosition::None).unwrap(),
            "bluey green".to_string(),
        ),
        (
            Color::from_hex("#c3909b", AlphaPosition::None).unwrap(),
            "grey pink".to_string(),
        ),
        (
            Color::from_hex("#a66fb5", AlphaPosition::None).unwrap(),
            "soft purple".to_string(),
        ),
        (
            Color::from_hex("#770001", AlphaPosition::None).unwrap(),
            "blood".to_string(),
        ),
        (
            Color::from_hex("#922b05", AlphaPosition::None).unwrap(),
            "brown red".to_string(),
        ),
        (
            Color::from_hex("#7d7f7c", AlphaPosition::None).unwrap(),
            "medium grey".to_string(),
        ),
        (
            Color::from_hex("#990f4b", AlphaPosition::None).unwrap(),
            "berry".to_string(),
        ),
        (
            Color::from_hex("#8f7303", AlphaPosition::None).unwrap(),
            "poo".to_string(),
        ),
        (
            Color::from_hex("#c83cb9", AlphaPosition::None).unwrap(),
            "purpley pink".to_string(),
        ),
        (
            Color::from_hex("#fea993", AlphaPosition::None).unwrap(),
            "light salmon".to_string(),
        ),
        (
            Color::from_hex("#acbb0d", AlphaPosition::None).unwrap(),
            "snot".to_string(),
        ),
        (
            Color::from_hex("#c071fe", AlphaPosition::None).unwrap(),
            "easter purple".to_string(),
        ),
        (
            Color::from_hex("#ccfd7f", AlphaPosition::None).unwrap(),
            "light yellow green".to_string(),
        ),
        (
            Color::from_hex("#00022e", AlphaPosition::None).unwrap(),
            "dark navy blue".to_string(),
        ),
        (
            Color::from_hex("#828344", AlphaPosition::None).unwrap(),
            "drab".to_string(),
        ),
        (
            Color::from_hex("#ffc5cb", AlphaPosition::None).unwrap(),
            "light rose".to_string(),
        ),
        (
            Color::from_hex("#ab1239", AlphaPosition::None).unwrap(),
            "rouge".to_string(),
        ),
        (
            Color::from_hex("#b0054b", AlphaPosition::None).unwrap(),
            "purplish red".to_string(),
        ),
        (
            Color::from_hex("#99cc04", AlphaPosition::None).unwrap(),
            "slime green".to_string(),
        ),
        (
            Color::from_hex("#937c00", AlphaPosition::None).unwrap(),
            "baby poop".to_string(),
        ),
        (
            Color::from_hex("#019529", AlphaPosition::None).unwrap(),
            "irish green".to_string(),
        ),
        (
            Color::from_hex("#ef1de7", AlphaPosition::None).unwrap(),
            "pink/purple".to_string(),
        ),
        (
            Color::from_hex("#000435", AlphaPosition::None).unwrap(),
            "dark navy".to_string(),
        ),
        (
            Color::from_hex("#42b395", AlphaPosition::None).unwrap(),
            "greeny blue".to_string(),
        ),
        (
            Color::from_hex("#9d5783", AlphaPosition::None).unwrap(),
            "light plum".to_string(),
        ),
        (
            Color::from_hex("#c8aca9", AlphaPosition::None).unwrap(),
            "pinkish grey".to_string(),
        ),
        (
            Color::from_hex("#c87606", AlphaPosition::None).unwrap(),
            "dirty orange".to_string(),
        ),
        (
            Color::from_hex("#aa2704", AlphaPosition::None).unwrap(),
            "rust red".to_string(),
        ),
        (
            Color::from_hex("#e4cbff", AlphaPosition::None).unwrap(),
            "pale lilac".to_string(),
        ),
        (
            Color::from_hex("#fa4224", AlphaPosition::None).unwrap(),
            "orangey red".to_string(),
        ),
        (
            Color::from_hex("#0804f9", AlphaPosition::None).unwrap(),
            "primary blue".to_string(),
        ),
        (
            Color::from_hex("#5cb200", AlphaPosition::None).unwrap(),
            "kermit green".to_string(),
        ),
        (
            Color::from_hex("#76424e", AlphaPosition::None).unwrap(),
            "brownish purple".to_string(),
        ),
        (
            Color::from_hex("#6c7a0e", AlphaPosition::None).unwrap(),
            "murky green".to_string(),
        ),
        (
            Color::from_hex("#fbdd7e", AlphaPosition::None).unwrap(),
            "wheat".to_string(),
        ),
        (
            Color::from_hex("#2a0134", AlphaPosition::None).unwrap(),
            "very dark purple".to_string(),
        ),
        (
            Color::from_hex("#044a05", AlphaPosition::None).unwrap(),
            "bottle green".to_string(),
        ),
        (
            Color::from_hex("#fd4659", AlphaPosition::None).unwrap(),
            "watermelon".to_string(),
        ),
        (
            Color::from_hex("#0d75f8", AlphaPosition::None).unwrap(),
            "deep sky blue".to_string(),
        ),
        (
            Color::from_hex("#fe0002", AlphaPosition::None).unwrap(),
            "fire engine red".to_string(),
        ),
        (
            Color::from_hex("#cb9d06", AlphaPosition::None).unwrap(),
            "yellow ochre".to_string(),
        ),
        (
            Color::from_hex("#fb7d07", AlphaPosition::None).unwrap(),
            "pumpkin orange".to_string(),
        ),
        (
            Color::from_hex("#b9cc81", AlphaPosition::None).unwrap(),
            "pale olive".to_string(),
        ),
        (
            Color::from_hex("#edc8ff", AlphaPosition::None).unwrap(),
            "light lilac".to_string(),
        ),
        (
            Color::from_hex("#61e160", AlphaPosition::None).unwrap(),
            "lightish green".to_string(),
        ),
        (
            Color::from_hex("#8ab8fe", AlphaPosition::None).unwrap(),
            "carolina blue".to_string(),
        ),
        (
            Color::from_hex("#920a4e", AlphaPosition::None).unwrap(),
            "mulberry".to_string(),
        ),
        (
            Color::from_hex("#fe02a2", AlphaPosition::None).unwrap(),
            "shocking pink".to_string(),
        ),
        (
            Color::from_hex("#9a3001", AlphaPosition::None).unwrap(),
            "auburn".to_string(),
        ),
        (
            Color::from_hex("#65fe08", AlphaPosition::None).unwrap(),
            "bright lime green".to_string(),
        ),
        (
            Color::from_hex("#befdb7", AlphaPosition::None).unwrap(),
            "celadon".to_string(),
        ),
        (
            Color::from_hex("#b17261", AlphaPosition::None).unwrap(),
            "pinkish brown".to_string(),
        ),
        (
            Color::from_hex("#885f01", AlphaPosition::None).unwrap(),
            "poo brown".to_string(),
        ),
        (
            Color::from_hex("#02ccfe", AlphaPosition::None).unwrap(),
            "bright sky blue".to_string(),
        ),
        (
            Color::from_hex("#c1fd95", AlphaPosition::None).unwrap(),
            "celery".to_string(),
        ),
        (
            Color::from_hex("#836539", AlphaPosition::None).unwrap(),
            "dirt brown".to_string(),
        ),
        (
            Color::from_hex("#fb2943", AlphaPosition::None).unwrap(),
            "strawberry".to_string(),
        ),
        (
            Color::from_hex("#84b701", AlphaPosition::None).unwrap(),
            "dark lime".to_string(),
        ),
        (
            Color::from_hex("#b66325", AlphaPosition::None).unwrap(),
            "copper".to_string(),
        ),
        (
            Color::from_hex("#7f5112", AlphaPosition::None).unwrap(),
            "medium brown".to_string(),
        ),
        (
            Color::from_hex("#5fa052", AlphaPosition::None).unwrap(),
            "muted green".to_string(),
        ),
        (
            Color::from_hex("#6dedfd", AlphaPosition::None).unwrap(),
            "robin's egg".to_string(),
        ),
        (
            Color::from_hex("#0bf9ea", AlphaPosition::None).unwrap(),
            "bright aqua".to_string(),
        ),
        (
            Color::from_hex("#c760ff", AlphaPosition::None).unwrap(),
            "bright lavender".to_string(),
        ),
        (
            Color::from_hex("#ffffcb", AlphaPosition::None).unwrap(),
            "ivory".to_string(),
        ),
        (
            Color::from_hex("#f6cefc", AlphaPosition::None).unwrap(),
            "very light purple".to_string(),
        ),
        (
            Color::from_hex("#155084", AlphaPosition::None).unwrap(),
            "light navy".to_string(),
        ),
        (
            Color::from_hex("#f5054f", AlphaPosition::None).unwrap(),
            "pink red".to_string(),
        ),
        (
            Color::from_hex("#645403", AlphaPosition::None).unwrap(),
            "olive brown".to_string(),
        ),
        (
            Color::from_hex("#7a5901", AlphaPosition::None).unwrap(),
            "poop brown".to_string(),
        ),
        (
            Color::from_hex("#a8b504", AlphaPosition::None).unwrap(),
            "mustard green".to_string(),
        ),
        (
            Color::from_hex("#3d9973", AlphaPosition::None).unwrap(),
            "ocean green".to_string(),
        ),
        (
            Color::from_hex("#000133", AlphaPosition::None).unwrap(),
            "very dark blue".to_string(),
        ),
        (
            Color::from_hex("#76a973", AlphaPosition::None).unwrap(),
            "dusty green".to_string(),
        ),
        (
            Color::from_hex("#2e5a88", AlphaPosition::None).unwrap(),
            "light navy blue".to_string(),
        ),
        (
            Color::from_hex("#0bf77d", AlphaPosition::None).unwrap(),
            "minty green".to_string(),
        ),
        (
            Color::from_hex("#bd6c48", AlphaPosition::None).unwrap(),
            "adobe".to_string(),
        ),
        (
            Color::from_hex("#ac1db8", AlphaPosition::None).unwrap(),
            "barney".to_string(),
        ),
        (
            Color::from_hex("#2baf6a", AlphaPosition::None).unwrap(),
            "jade green".to_string(),
        ),
        (
            Color::from_hex("#26f7fd", AlphaPosition::None).unwrap(),
            "bright light blue".to_string(),
        ),
        (
            Color::from_hex("#aefd6c", AlphaPosition::None).unwrap(),
            "light lime".to_string(),
        ),
        (
            Color::from_hex("#9b8f55", AlphaPosition::None).unwrap(),
            "dark khaki".to_string(),
        ),
        (
            Color::from_hex("#ffad01", AlphaPosition::None).unwrap(),
            "orange yellow".to_string(),
        ),
        (
            Color::from_hex("#c69c04", AlphaPosition::None).unwrap(),
            "ocre".to_string(),
        ),
        (
            Color::from_hex("#f4d054", AlphaPosition::None).unwrap(),
            "maize".to_string(),
        ),
        (
            Color::from_hex("#de9dac", AlphaPosition::None).unwrap(),
            "faded pink".to_string(),
        ),
        (
            Color::from_hex("#05480d", AlphaPosition::None).unwrap(),
            "british racing green".to_string(),
        ),
        (
            Color::from_hex("#c9ae74", AlphaPosition::None).unwrap(),
            "sandstone".to_string(),
        ),
        (
            Color::from_hex("#60460f", AlphaPosition::None).unwrap(),
            "mud brown".to_string(),
        ),
        (
            Color::from_hex("#98f6b0", AlphaPosition::None).unwrap(),
            "light sea green".to_string(),
        ),
        (
            Color::from_hex("#8af1fe", AlphaPosition::None).unwrap(),
            "robin egg blue".to_string(),
        ),
        (
            Color::from_hex("#2ee8bb", AlphaPosition::None).unwrap(),
            "aqua marine".to_string(),
        ),
        (
            Color::from_hex("#11875d", AlphaPosition::None).unwrap(),
            "dark sea green".to_string(),
        ),
        (
            Color::from_hex("#fdb0c0", AlphaPosition::None).unwrap(),
            "soft pink".to_string(),
        ),
        (
            Color::from_hex("#b16002", AlphaPosition::None).unwrap(),
            "orangey brown".to_string(),
        ),
        (
            Color::from_hex("#f7022a", AlphaPosition::None).unwrap(),
            "cherry red".to_string(),
        ),
        (
            Color::from_hex("#d5ab09", AlphaPosition::None).unwrap(),
            "burnt yellow".to_string(),
        ),
        (
            Color::from_hex("#86775f", AlphaPosition::None).unwrap(),
            "brownish grey".to_string(),
        ),
        (
            Color::from_hex("#c69f59", AlphaPosition::None).unwrap(),
            "camel".to_string(),
        ),
        (
            Color::from_hex("#7a687f", AlphaPosition::None).unwrap(),
            "purplish grey".to_string(),
        ),
        (
            Color::from_hex("#042e60", AlphaPosition::None).unwrap(),
            "marine".to_string(),
        ),
        (
            Color::from_hex("#c88d94", AlphaPosition::None).unwrap(),
            "greyish pink".to_string(),
        ),
        (
            Color::from_hex("#a5fbd5", AlphaPosition::None).unwrap(),
            "pale turquoise".to_string(),
        ),
        (
            Color::from_hex("#fffe71", AlphaPosition::None).unwrap(),
            "pastel yellow".to_string(),
        ),
        (
            Color::from_hex("#6241c7", AlphaPosition::None).unwrap(),
            "bluey purple".to_string(),
        ),
        (
            Color::from_hex("#fffe40", AlphaPosition::None).unwrap(),
            "canary yellow".to_string(),
        ),
        (
            Color::from_hex("#d3494e", AlphaPosition::None).unwrap(),
            "faded red".to_string(),
        ),
        (
            Color::from_hex("#985e2b", AlphaPosition::None).unwrap(),
            "sepia".to_string(),
        ),
        (
            Color::from_hex("#a6814c", AlphaPosition::None).unwrap(),
            "coffee".to_string(),
        ),
        (
            Color::from_hex("#ff08e8", AlphaPosition::None).unwrap(),
            "bright magenta".to_string(),
        ),
        (
            Color::from_hex("#9d7651", AlphaPosition::None).unwrap(),
            "mocha".to_string(),
        ),
        (
            Color::from_hex("#feffca", AlphaPosition::None).unwrap(),
            "ecru".to_string(),
        ),
        (
            Color::from_hex("#98568d", AlphaPosition::None).unwrap(),
            "purpleish".to_string(),
        ),
        (
            Color::from_hex("#9e003a", AlphaPosition::None).unwrap(),
            "cranberry".to_string(),
        ),
        (
            Color::from_hex("#287c37", AlphaPosition::None).unwrap(),
            "darkish green".to_string(),
        ),
        (
            Color::from_hex("#b96902", AlphaPosition::None).unwrap(),
            "brown orange".to_string(),
        ),
        (
            Color::from_hex("#ba6873", AlphaPosition::None).unwrap(),
            "dusky rose".to_string(),
        ),
        (
            Color::from_hex("#ff7855", AlphaPosition::None).unwrap(),
            "melon".to_string(),
        ),
        (
            Color::from_hex("#94b21c", AlphaPosition::None).unwrap(),
            "sickly green".to_string(),
        ),
        (
            Color::from_hex("#c5c9c7", AlphaPosition::None).unwrap(),
            "silver".to_string(),
        ),
        (
            Color::from_hex("#661aee", AlphaPosition::None).unwrap(),
            "purply blue".to_string(),
        ),
        (
            Color::from_hex("#6140ef", AlphaPosition::None).unwrap(),
            "purpleish blue".to_string(),
        ),
        (
            Color::from_hex("#9be5aa", AlphaPosition::None).unwrap(),
            "hospital green".to_string(),
        ),
        (
            Color::from_hex("#7b5804", AlphaPosition::None).unwrap(),
            "shit brown".to_string(),
        ),
        (
            Color::from_hex("#276ab3", AlphaPosition::None).unwrap(),
            "mid blue".to_string(),
        ),
        (
            Color::from_hex("#feb308", AlphaPosition::None).unwrap(),
            "amber".to_string(),
        ),
        (
            Color::from_hex("#8cfd7e", AlphaPosition::None).unwrap(),
            "easter green".to_string(),
        ),
        (
            Color::from_hex("#6488ea", AlphaPosition::None).unwrap(),
            "soft blue".to_string(),
        ),
        (
            Color::from_hex("#056eee", AlphaPosition::None).unwrap(),
            "cerulean blue".to_string(),
        ),
        (
            Color::from_hex("#b27a01", AlphaPosition::None).unwrap(),
            "golden brown".to_string(),
        ),
        (
            Color::from_hex("#0ffef9", AlphaPosition::None).unwrap(),
            "bright turquoise".to_string(),
        ),
        (
            Color::from_hex("#fa2a55", AlphaPosition::None).unwrap(),
            "red pink".to_string(),
        ),
        (
            Color::from_hex("#820747", AlphaPosition::None).unwrap(),
            "red purple".to_string(),
        ),
        (
            Color::from_hex("#7a6a4f", AlphaPosition::None).unwrap(),
            "greyish brown".to_string(),
        ),
        (
            Color::from_hex("#f4320c", AlphaPosition::None).unwrap(),
            "vermillion".to_string(),
        ),
        (
            Color::from_hex("#a13905", AlphaPosition::None).unwrap(),
            "russet".to_string(),
        ),
        (
            Color::from_hex("#6f828a", AlphaPosition::None).unwrap(),
            "steel grey".to_string(),
        ),
        (
            Color::from_hex("#a55af4", AlphaPosition::None).unwrap(),
            "lighter purple".to_string(),
        ),
        (
            Color::from_hex("#ad0afd", AlphaPosition::None).unwrap(),
            "bright violet".to_string(),
        ),
        (
            Color::from_hex("#004577", AlphaPosition::None).unwrap(),
            "prussian blue".to_string(),
        ),
        (
            Color::from_hex("#658d6d", AlphaPosition::None).unwrap(),
            "slate green".to_string(),
        ),
        (
            Color::from_hex("#ca7b80", AlphaPosition::None).unwrap(),
            "dirty pink".to_string(),
        ),
        (
            Color::from_hex("#005249", AlphaPosition::None).unwrap(),
            "dark blue green".to_string(),
        ),
        (
            Color::from_hex("#2b5d34", AlphaPosition::None).unwrap(),
            "pine".to_string(),
        ),
        (
            Color::from_hex("#bff128", AlphaPosition::None).unwrap(),
            "yellowy green".to_string(),
        ),
        (
            Color::from_hex("#b59410", AlphaPosition::None).unwrap(),
            "dark gold".to_string(),
        ),
        (
            Color::from_hex("#2976bb", AlphaPosition::None).unwrap(),
            "bluish".to_string(),
        ),
        (
            Color::from_hex("#014182", AlphaPosition::None).unwrap(),
            "darkish blue".to_string(),
        ),
        (
            Color::from_hex("#bb3f3f", AlphaPosition::None).unwrap(),
            "dull red".to_string(),
        ),
        (
            Color::from_hex("#fc2647", AlphaPosition::None).unwrap(),
            "pinky red".to_string(),
        ),
        (
            Color::from_hex("#a87900", AlphaPosition::None).unwrap(),
            "bronze".to_string(),
        ),
        (
            Color::from_hex("#82cbb2", AlphaPosition::None).unwrap(),
            "pale teal".to_string(),
        ),
        (
            Color::from_hex("#667c3e", AlphaPosition::None).unwrap(),
            "military green".to_string(),
        ),
        (
            Color::from_hex("#fe46a5", AlphaPosition::None).unwrap(),
            "barbie pink".to_string(),
        ),
        (
            Color::from_hex("#fe83cc", AlphaPosition::None).unwrap(),
            "bubblegum pink".to_string(),
        ),
        (
            Color::from_hex("#94a617", AlphaPosition::None).unwrap(),
            "pea soup green".to_string(),
        ),
        (
            Color::from_hex("#a88905", AlphaPosition::None).unwrap(),
            "dark mustard".to_string(),
        ),
        (
            Color::from_hex("#7f5f00", AlphaPosition::None).unwrap(),
            "shit".to_string(),
        ),
        (
            Color::from_hex("#9e43a2", AlphaPosition::None).unwrap(),
            "medium purple".to_string(),
        ),
        (
            Color::from_hex("#062e03", AlphaPosition::None).unwrap(),
            "very dark green".to_string(),
        ),
        (
            Color::from_hex("#8a6e45", AlphaPosition::None).unwrap(),
            "dirt".to_string(),
        ),
        (
            Color::from_hex("#cc7a8b", AlphaPosition::None).unwrap(),
            "dusky pink".to_string(),
        ),
        (
            Color::from_hex("#9e0168", AlphaPosition::None).unwrap(),
            "red violet".to_string(),
        ),
        (
            Color::from_hex("#fdff38", AlphaPosition::None).unwrap(),
            "lemon yellow".to_string(),
        ),
        (
            Color::from_hex("#c0fa8b", AlphaPosition::None).unwrap(),
            "pistachio".to_string(),
        ),
        (
            Color::from_hex("#eedc5b", AlphaPosition::None).unwrap(),
            "dull yellow".to_string(),
        ),
        (
            Color::from_hex("#7ebd01", AlphaPosition::None).unwrap(),
            "dark lime green".to_string(),
        ),
        (
            Color::from_hex("#3b5b92", AlphaPosition::None).unwrap(),
            "denim blue".to_string(),
        ),
        (
            Color::from_hex("#01889f", AlphaPosition::None).unwrap(),
            "teal blue".to_string(),
        ),
        (
            Color::from_hex("#3d7afd", AlphaPosition::None).unwrap(),
            "lightish blue".to_string(),
        ),
        (
            Color::from_hex("#5f34e7", AlphaPosition::None).unwrap(),
            "purpley blue".to_string(),
        ),
        (
            Color::from_hex("#6d5acf", AlphaPosition::None).unwrap(),
            "light indigo".to_string(),
        ),
        (
            Color::from_hex("#748500", AlphaPosition::None).unwrap(),
            "swamp green".to_string(),
        ),
        (
            Color::from_hex("#706c11", AlphaPosition::None).unwrap(),
            "brown green".to_string(),
        ),
        (
            Color::from_hex("#3c0008", AlphaPosition::None).unwrap(),
            "dark maroon".to_string(),
        ),
        (
            Color::from_hex("#cb00f5", AlphaPosition::None).unwrap(),
            "hot purple".to_string(),
        ),
        (
            Color::from_hex("#002d04", AlphaPosition::None).unwrap(),
            "dark forest green".to_string(),
        ),
        (
            Color::from_hex("#658cbb", AlphaPosition::None).unwrap(),
            "faded blue".to_string(),
        ),
        (
            Color::from_hex("#749551", AlphaPosition::None).unwrap(),
            "drab green".to_string(),
        ),
        (
            Color::from_hex("#b9ff66", AlphaPosition::None).unwrap(),
            "light lime green".to_string(),
        ),
        (
            Color::from_hex("#9dc100", AlphaPosition::None).unwrap(),
            "snot green".to_string(),
        ),
        (
            Color::from_hex("#faee66", AlphaPosition::None).unwrap(),
            "yellowish".to_string(),
        ),
        (
            Color::from_hex("#7efbb3", AlphaPosition::None).unwrap(),
            "light blue green".to_string(),
        ),
        (
            Color::from_hex("#7b002c", AlphaPosition::None).unwrap(),
            "bordeaux".to_string(),
        ),
        (
            Color::from_hex("#c292a1", AlphaPosition::None).unwrap(),
            "light mauve".to_string(),
        ),
        (
            Color::from_hex("#017b92", AlphaPosition::None).unwrap(),
            "ocean".to_string(),
        ),
        (
            Color::from_hex("#fcc006", AlphaPosition::None).unwrap(),
            "marigold".to_string(),
        ),
        (
            Color::from_hex("#657432", AlphaPosition::None).unwrap(),
            "muddy green".to_string(),
        ),
        (
            Color::from_hex("#d8863b", AlphaPosition::None).unwrap(),
            "dull orange".to_string(),
        ),
        (
            Color::from_hex("#738595", AlphaPosition::None).unwrap(),
            "steel".to_string(),
        ),
        (
            Color::from_hex("#aa23ff", AlphaPosition::None).unwrap(),
            "electric purple".to_string(),
        ),
        (
            Color::from_hex("#08ff08", AlphaPosition::None).unwrap(),
            "fluorescent green".to_string(),
        ),
        (
            Color::from_hex("#9b7a01", AlphaPosition::None).unwrap(),
            "yellowish brown".to_string(),
        ),
        (
            Color::from_hex("#f29e8e", AlphaPosition::None).unwrap(),
            "blush".to_string(),
        ),
        (
            Color::from_hex("#6fc276", AlphaPosition::None).unwrap(),
            "soft green".to_string(),
        ),
        (
            Color::from_hex("#ff5b00", AlphaPosition::None).unwrap(),
            "bright orange".to_string(),
        ),
        (
            Color::from_hex("#fdff52", AlphaPosition::None).unwrap(),
            "lemon".to_string(),
        ),
        (
            Color::from_hex("#866f85", AlphaPosition::None).unwrap(),
            "purple grey".to_string(),
        ),
        (
            Color::from_hex("#8ffe09", AlphaPosition::None).unwrap(),
            "acid green".to_string(),
        ),
        (
            Color::from_hex("#eecffe", AlphaPosition::None).unwrap(),
            "pale lavender".to_string(),
        ),
        (
            Color::from_hex("#510ac9", AlphaPosition::None).unwrap(),
            "violet blue".to_string(),
        ),
        (
            Color::from_hex("#4f9153", AlphaPosition::None).unwrap(),
            "light forest green".to_string(),
        ),
        (
            Color::from_hex("#9f2305", AlphaPosition::None).unwrap(),
            "burnt red".to_string(),
        ),
        (
            Color::from_hex("#728639", AlphaPosition::None).unwrap(),
            "khaki green".to_string(),
        ),
        (
            Color::from_hex("#de0c62", AlphaPosition::None).unwrap(),
            "cerise".to_string(),
        ),
        (
            Color::from_hex("#916e99", AlphaPosition::None).unwrap(),
            "faded purple".to_string(),
        ),
        (
            Color::from_hex("#ffb16d", AlphaPosition::None).unwrap(),
            "apricot".to_string(),
        ),
        (
            Color::from_hex("#3c4d03", AlphaPosition::None).unwrap(),
            "dark olive green".to_string(),
        ),
        (
            Color::from_hex("#7f7053", AlphaPosition::None).unwrap(),
            "grey brown".to_string(),
        ),
        (
            Color::from_hex("#77926f", AlphaPosition::None).unwrap(),
            "green grey".to_string(),
        ),
        (
            Color::from_hex("#010fcc", AlphaPosition::None).unwrap(),
            "true blue".to_string(),
        ),
        (
            Color::from_hex("#ceaefa", AlphaPosition::None).unwrap(),
            "pale violet".to_string(),
        ),
        (
            Color::from_hex("#8f99fb", AlphaPosition::None).unwrap(),
            "periwinkle blue".to_string(),
        ),
        (
            Color::from_hex("#c6fcff", AlphaPosition::None).unwrap(),
            "light sky blue".to_string(),
        ),
        (
            Color::from_hex("#5539cc", AlphaPosition::None).unwrap(),
            "blurple".to_string(),
        ),
        (
            Color::from_hex("#544e03", AlphaPosition::None).unwrap(),
            "green brown".to_string(),
        ),
        (
            Color::from_hex("#017a79", AlphaPosition::None).unwrap(),
            "bluegreen".to_string(),
        ),
        (
            Color::from_hex("#01f9c6", AlphaPosition::None).unwrap(),
            "bright teal".to_string(),
        ),
        (
            Color::from_hex("#c9b003", AlphaPosition::None).unwrap(),
            "brownish yellow".to_string(),
        ),
        (
            Color::from_hex("#929901", AlphaPosition::None).unwrap(),
            "pea soup".to_string(),
        ),
        (
            Color::from_hex("#0b5509", AlphaPosition::None).unwrap(),
            "forest".to_string(),
        ),
        (
            Color::from_hex("#a00498", AlphaPosition::None).unwrap(),
            "barney purple".to_string(),
        ),
        (
            Color::from_hex("#2000b1", AlphaPosition::None).unwrap(),
            "ultramarine".to_string(),
        ),
        (
            Color::from_hex("#94568c", AlphaPosition::None).unwrap(),
            "purplish".to_string(),
        ),
        (
            Color::from_hex("#c2be0e", AlphaPosition::None).unwrap(),
            "puke yellow".to_string(),
        ),
        (
            Color::from_hex("#748b97", AlphaPosition::None).unwrap(),
            "bluish grey".to_string(),
        ),
        (
            Color::from_hex("#665fd1", AlphaPosition::None).unwrap(),
            "dark periwinkle".to_string(),
        ),
        (
            Color::from_hex("#9c6da5", AlphaPosition::None).unwrap(),
            "dark lilac".to_string(),
        ),
        (
            Color::from_hex("#c44240", AlphaPosition::None).unwrap(),
            "reddish".to_string(),
        ),
        (
            Color::from_hex("#a24857", AlphaPosition::None).unwrap(),
            "light maroon".to_string(),
        ),
        (
            Color::from_hex("#825f87", AlphaPosition::None).unwrap(),
            "dusty purple".to_string(),
        ),
        (
            Color::from_hex("#c9643b", AlphaPosition::None).unwrap(),
            "terra cotta".to_string(),
        ),
        (
            Color::from_hex("#90b134", AlphaPosition::None).unwrap(),
            "avocado".to_string(),
        ),
        (
            Color::from_hex("#01386a", AlphaPosition::None).unwrap(),
            "marine blue".to_string(),
        ),
        (
            Color::from_hex("#25a36f", AlphaPosition::None).unwrap(),
            "teal green".to_string(),
        ),
        (
            Color::from_hex("#59656d", AlphaPosition::None).unwrap(),
            "slate grey".to_string(),
        ),
        (
            Color::from_hex("#75fd63", AlphaPosition::None).unwrap(),
            "lighter green".to_string(),
        ),
        (
            Color::from_hex("#21fc0d", AlphaPosition::None).unwrap(),
            "electric green".to_string(),
        ),
        (
            Color::from_hex("#5a86ad", AlphaPosition::None).unwrap(),
            "dusty blue".to_string(),
        ),
        (
            Color::from_hex("#fec615", AlphaPosition::None).unwrap(),
            "golden yellow".to_string(),
        ),
        (
            Color::from_hex("#fffd01", AlphaPosition::None).unwrap(),
            "bright yellow".to_string(),
        ),
        (
            Color::from_hex("#dfc5fe", AlphaPosition::None).unwrap(),
            "light lavender".to_string(),
        ),
        (
            Color::from_hex("#b26400", AlphaPosition::None).unwrap(),
            "umber".to_string(),
        ),
        (
            Color::from_hex("#7f5e00", AlphaPosition::None).unwrap(),
            "poop".to_string(),
        ),
        (
            Color::from_hex("#de7e5d", AlphaPosition::None).unwrap(),
            "dark peach".to_string(),
        ),
        (
            Color::from_hex("#048243", AlphaPosition::None).unwrap(),
            "jungle green".to_string(),
        ),
        (
            Color::from_hex("#ffffd4", AlphaPosition::None).unwrap(),
            "eggshell".to_string(),
        ),
        (
            Color::from_hex("#3b638c", AlphaPosition::None).unwrap(),
            "denim".to_string(),
        ),
        (
            Color::from_hex("#b79400", AlphaPosition::None).unwrap(),
            "yellow brown".to_string(),
        ),
        (
            Color::from_hex("#84597e", AlphaPosition::None).unwrap(),
            "dull purple".to_string(),
        ),
        (
            Color::from_hex("#411900", AlphaPosition::None).unwrap(),
            "chocolate brown".to_string(),
        ),
        (
            Color::from_hex("#7b0323", AlphaPosition::None).unwrap(),
            "wine red".to_string(),
        ),
        (
            Color::from_hex("#04d9ff", AlphaPosition::None).unwrap(),
            "neon blue".to_string(),
        ),
        (
            Color::from_hex("#667e2c", AlphaPosition::None).unwrap(),
            "dirty green".to_string(),
        ),
        (
            Color::from_hex("#fbeeac", AlphaPosition::None).unwrap(),
            "light tan".to_string(),
        ),
        (
            Color::from_hex("#d7fffe", AlphaPosition::None).unwrap(),
            "ice blue".to_string(),
        ),
        (
            Color::from_hex("#4e7496", AlphaPosition::None).unwrap(),
            "cadet blue".to_string(),
        ),
        (
            Color::from_hex("#874c62", AlphaPosition::None).unwrap(),
            "dark mauve".to_string(),
        ),
        (
            Color::from_hex("#d5ffff", AlphaPosition::None).unwrap(),
            "very light blue".to_string(),
        ),
        (
            Color::from_hex("#826d8c", AlphaPosition::None).unwrap(),
            "grey purple".to_string(),
        ),
        (
            Color::from_hex("#ffbacd", AlphaPosition::None).unwrap(),
            "pastel pink".to_string(),
        ),
        (
            Color::from_hex("#d1ffbd", AlphaPosition::None).unwrap(),
            "very light green".to_string(),
        ),
        (
            Color::from_hex("#448ee4", AlphaPosition::None).unwrap(),
            "dark sky blue".to_string(),
        ),
        (
            Color::from_hex("#05472a", AlphaPosition::None).unwrap(),
            "evergreen".to_string(),
        ),
        (
            Color::from_hex("#d5869d", AlphaPosition::None).unwrap(),
            "dull pink".to_string(),
        ),
        (
            Color::from_hex("#3d0734", AlphaPosition::None).unwrap(),
            "aubergine".to_string(),
        ),
        (
            Color::from_hex("#4a0100", AlphaPosition::None).unwrap(),
            "mahogany".to_string(),
        ),
        (
            Color::from_hex("#f8481c", AlphaPosition::None).unwrap(),
            "reddish orange".to_string(),
        ),
        (
            Color::from_hex("#02590f", AlphaPosition::None).unwrap(),
            "deep green".to_string(),
        ),
        (
            Color::from_hex("#89a203", AlphaPosition::None).unwrap(),
            "vomit green".to_string(),
        ),
        (
            Color::from_hex("#e03fd8", AlphaPosition::None).unwrap(),
            "purple pink".to_string(),
        ),
        (
            Color::from_hex("#d58a94", AlphaPosition::None).unwrap(),
            "dusty pink".to_string(),
        ),
        (
            Color::from_hex("#7bb274", AlphaPosition::None).unwrap(),
            "faded green".to_string(),
        ),
        (
            Color::from_hex("#526525", AlphaPosition::None).unwrap(),
            "camo green".to_string(),
        ),
        (
            Color::from_hex("#c94cbe", AlphaPosition::None).unwrap(),
            "pinky purple".to_string(),
        ),
        (
            Color::from_hex("#db4bda", AlphaPosition::None).unwrap(),
            "pink purple".to_string(),
        ),
        (
            Color::from_hex("#9e3623", AlphaPosition::None).unwrap(),
            "brownish red".to_string(),
        ),
        (
            Color::from_hex("#b5485d", AlphaPosition::None).unwrap(),
            "dark rose".to_string(),
        ),
        (
            Color::from_hex("#735c12", AlphaPosition::None).unwrap(),
            "mud".to_string(),
        ),
        (
            Color::from_hex("#9c6d57", AlphaPosition::None).unwrap(),
            "brownish".to_string(),
        ),
        (
            Color::from_hex("#028f1e", AlphaPosition::None).unwrap(),
            "emerald green".to_string(),
        ),
        (
            Color::from_hex("#b1916e", AlphaPosition::None).unwrap(),
            "pale brown".to_string(),
        ),
        (
            Color::from_hex("#49759c", AlphaPosition::None).unwrap(),
            "dull blue".to_string(),
        ),
        (
            Color::from_hex("#a0450e", AlphaPosition::None).unwrap(),
            "burnt umber".to_string(),
        ),
        (
            Color::from_hex("#39ad48", AlphaPosition::None).unwrap(),
            "medium green".to_string(),
        ),
        (
            Color::from_hex("#b66a50", AlphaPosition::None).unwrap(),
            "clay".to_string(),
        ),
        (
            Color::from_hex("#8cffdb", AlphaPosition::None).unwrap(),
            "light aqua".to_string(),
        ),
        (
            Color::from_hex("#a4be5c", AlphaPosition::None).unwrap(),
            "light olive green".to_string(),
        ),
        (
            Color::from_hex("#cb7723", AlphaPosition::None).unwrap(),
            "brownish orange".to_string(),
        ),
        (
            Color::from_hex("#05696b", AlphaPosition::None).unwrap(),
            "dark aqua".to_string(),
        ),
        (
            Color::from_hex("#ce5dae", AlphaPosition::None).unwrap(),
            "purplish pink".to_string(),
        ),
        (
            Color::from_hex("#c85a53", AlphaPosition::None).unwrap(),
            "dark salmon".to_string(),
        ),
        (
            Color::from_hex("#96ae8d", AlphaPosition::None).unwrap(),
            "greenish grey".to_string(),
        ),
        (
            Color::from_hex("#1fa774", AlphaPosition::None).unwrap(),
            "jade".to_string(),
        ),
        (
            Color::from_hex("#7a9703", AlphaPosition::None).unwrap(),
            "ugly green".to_string(),
        ),
        (
            Color::from_hex("#ac9362", AlphaPosition::None).unwrap(),
            "dark beige".to_string(),
        ),
        (
            Color::from_hex("#01a049", AlphaPosition::None).unwrap(),
            "emerald".to_string(),
        ),
        (
            Color::from_hex("#d9544d", AlphaPosition::None).unwrap(),
            "pale red".to_string(),
        ),
        (
            Color::from_hex("#fa5ff7", AlphaPosition::None).unwrap(),
            "light magenta".to_string(),
        ),
        (
            Color::from_hex("#82cafc", AlphaPosition::None).unwrap(),
            "sky".to_string(),
        ),
        (
            Color::from_hex("#acfffc", AlphaPosition::None).unwrap(),
            "light cyan".to_string(),
        ),
        (
            Color::from_hex("#fcb001", AlphaPosition::None).unwrap(),
            "yellow orange".to_string(),
        ),
        (
            Color::from_hex("#910951", AlphaPosition::None).unwrap(),
            "reddish purple".to_string(),
        ),
        (
            Color::from_hex("#fe2c54", AlphaPosition::None).unwrap(),
            "reddish pink".to_string(),
        ),
        (
            Color::from_hex("#c875c4", AlphaPosition::None).unwrap(),
            "orchid".to_string(),
        ),
        (
            Color::from_hex("#cdc50a", AlphaPosition::None).unwrap(),
            "dirty yellow".to_string(),
        ),
        (
            Color::from_hex("#fd411e", AlphaPosition::None).unwrap(),
            "orange red".to_string(),
        ),
        (
            Color::from_hex("#9a0200", AlphaPosition::None).unwrap(),
            "deep red".to_string(),
        ),
        (
            Color::from_hex("#be6400", AlphaPosition::None).unwrap(),
            "orange brown".to_string(),
        ),
        (
            Color::from_hex("#030aa7", AlphaPosition::None).unwrap(),
            "cobalt blue".to_string(),
        ),
        (
            Color::from_hex("#fe019a", AlphaPosition::None).unwrap(),
            "neon pink".to_string(),
        ),
        (
            Color::from_hex("#f7879a", AlphaPosition::None).unwrap(),
            "rose pink".to_string(),
        ),
        (
            Color::from_hex("#887191", AlphaPosition::None).unwrap(),
            "greyish purple".to_string(),
        ),
        (
            Color::from_hex("#b00149", AlphaPosition::None).unwrap(),
            "raspberry".to_string(),
        ),
        (
            Color::from_hex("#12e193", AlphaPosition::None).unwrap(),
            "aqua green".to_string(),
        ),
        (
            Color::from_hex("#fe7b7c", AlphaPosition::None).unwrap(),
            "salmon pink".to_string(),
        ),
        (
            Color::from_hex("#ff9408", AlphaPosition::None).unwrap(),
            "tangerine".to_string(),
        ),
        (
            Color::from_hex("#6a6e09", AlphaPosition::None).unwrap(),
            "brownish green".to_string(),
        ),
        (
            Color::from_hex("#8b2e16", AlphaPosition::None).unwrap(),
            "red brown".to_string(),
        ),
        (
            Color::from_hex("#696112", AlphaPosition::None).unwrap(),
            "greenish brown".to_string(),
        ),
        (
            Color::from_hex("#e17701", AlphaPosition::None).unwrap(),
            "pumpkin".to_string(),
        ),
        (
            Color::from_hex("#0a481e", AlphaPosition::None).unwrap(),
            "pine green".to_string(),
        ),
        (
            Color::from_hex("#343837", AlphaPosition::None).unwrap(),
            "charcoal".to_string(),
        ),
        (
            Color::from_hex("#ffb7ce", AlphaPosition::None).unwrap(),
            "baby pink".to_string(),
        ),
        (
            Color::from_hex("#6a79f7", AlphaPosition::None).unwrap(),
            "cornflower".to_string(),
        ),
        (
            Color::from_hex("#5d06e9", AlphaPosition::None).unwrap(),
            "blue violet".to_string(),
        ),
        (
            Color::from_hex("#3d1c02", AlphaPosition::None).unwrap(),
            "chocolate".to_string(),
        ),
        (
            Color::from_hex("#82a67d", AlphaPosition::None).unwrap(),
            "greyish green".to_string(),
        ),
        (
            Color::from_hex("#be0119", AlphaPosition::None).unwrap(),
            "scarlet".to_string(),
        ),
        (
            Color::from_hex("#c9ff27", AlphaPosition::None).unwrap(),
            "green yellow".to_string(),
        ),
        (
            Color::from_hex("#373e02", AlphaPosition::None).unwrap(),
            "dark olive".to_string(),
        ),
        (
            Color::from_hex("#a9561e", AlphaPosition::None).unwrap(),
            "sienna".to_string(),
        ),
        (
            Color::from_hex("#caa0ff", AlphaPosition::None).unwrap(),
            "pastel purple".to_string(),
        ),
        (
            Color::from_hex("#ca6641", AlphaPosition::None).unwrap(),
            "terracotta".to_string(),
        ),
        (
            Color::from_hex("#02d8e9", AlphaPosition::None).unwrap(),
            "aqua blue".to_string(),
        ),
        (
            Color::from_hex("#88b378", AlphaPosition::None).unwrap(),
            "sage green".to_string(),
        ),
        (
            Color::from_hex("#980002", AlphaPosition::None).unwrap(),
            "blood red".to_string(),
        ),
        (
            Color::from_hex("#cb0162", AlphaPosition::None).unwrap(),
            "deep pink".to_string(),
        ),
        (
            Color::from_hex("#5cac2d", AlphaPosition::None).unwrap(),
            "grass".to_string(),
        ),
        (
            Color::from_hex("#769958", AlphaPosition::None).unwrap(),
            "moss".to_string(),
        ),
        (
            Color::from_hex("#a2bffe", AlphaPosition::None).unwrap(),
            "pastel blue".to_string(),
        ),
        (
            Color::from_hex("#10a674", AlphaPosition::None).unwrap(),
            "bluish green".to_string(),
        ),
        (
            Color::from_hex("#06b48b", AlphaPosition::None).unwrap(),
            "green blue".to_string(),
        ),
        (
            Color::from_hex("#af884a", AlphaPosition::None).unwrap(),
            "dark tan".to_string(),
        ),
        (
            Color::from_hex("#0b8b87", AlphaPosition::None).unwrap(),
            "greenish blue".to_string(),
        ),
        (
            Color::from_hex("#ffa756", AlphaPosition::None).unwrap(),
            "pale orange".to_string(),
        ),
        (
            Color::from_hex("#a2a415", AlphaPosition::None).unwrap(),
            "vomit".to_string(),
        ),
        (
            Color::from_hex("#154406", AlphaPosition::None).unwrap(),
            "forrest green".to_string(),
        ),
        (
            Color::from_hex("#856798", AlphaPosition::None).unwrap(),
            "dark lavender".to_string(),
        ),
        (
            Color::from_hex("#34013f", AlphaPosition::None).unwrap(),
            "dark violet".to_string(),
        ),
        (
            Color::from_hex("#632de9", AlphaPosition::None).unwrap(),
            "purple blue".to_string(),
        ),
        (
            Color::from_hex("#0a888a", AlphaPosition::None).unwrap(),
            "dark cyan".to_string(),
        ),
        (
            Color::from_hex("#6f7632", AlphaPosition::None).unwrap(),
            "olive drab".to_string(),
        ),
        (
            Color::from_hex("#d46a7e", AlphaPosition::None).unwrap(),
            "pinkish".to_string(),
        ),
        (
            Color::from_hex("#1e488f", AlphaPosition::None).unwrap(),
            "cobalt".to_string(),
        ),
        (
            Color::from_hex("#bc13fe", AlphaPosition::None).unwrap(),
            "neon purple".to_string(),
        ),
        (
            Color::from_hex("#7ef4cc", AlphaPosition::None).unwrap(),
            "light turquoise".to_string(),
        ),
        (
            Color::from_hex("#76cd26", AlphaPosition::None).unwrap(),
            "apple green".to_string(),
        ),
        (
            Color::from_hex("#74a662", AlphaPosition::None).unwrap(),
            "dull green".to_string(),
        ),
        (
            Color::from_hex("#80013f", AlphaPosition::None).unwrap(),
            "wine".to_string(),
        ),
        (
            Color::from_hex("#b1d1fc", AlphaPosition::None).unwrap(),
            "powder blue".to_string(),
        ),
        (
            Color::from_hex("#ffffe4", AlphaPosition::None).unwrap(),
            "off white".to_string(),
        ),
        (
            Color::from_hex("#0652ff", AlphaPosition::None).unwrap(),
            "electric blue".to_string(),
        ),
        (
            Color::from_hex("#045c5a", AlphaPosition::None).unwrap(),
            "dark turquoise".to_string(),
        ),
        (
            Color::from_hex("#5729ce", AlphaPosition::None).unwrap(),
            "blue purple".to_string(),
        ),
        (
            Color::from_hex("#069af3", AlphaPosition::None).unwrap(),
            "azure".to_string(),
        ),
        (
            Color::from_hex("#FF0000", AlphaPosition::None).unwrap(),
            "bright red".to_string(),
        ),
        (
            Color::from_hex("#f10c45", AlphaPosition::None).unwrap(),
            "pinkish red".to_string(),
        ),
        (
            Color::from_hex("#5170d7", AlphaPosition::None).unwrap(),
            "cornflower blue".to_string(),
        ),
        (
            Color::from_hex("#acbf69", AlphaPosition::None).unwrap(),
            "light olive".to_string(),
        ),
        (
            Color::from_hex("#6c3461", AlphaPosition::None).unwrap(),
            "grape".to_string(),
        ),
        (
            Color::from_hex("#5e819d", AlphaPosition::None).unwrap(),
            "greyish blue".to_string(),
        ),
        (
            Color::from_hex("#601ef9", AlphaPosition::None).unwrap(),
            "purplish blue".to_string(),
        ),
        (
            Color::from_hex("#b0dd16", AlphaPosition::None).unwrap(),
            "yellowish green".to_string(),
        ),
        (
            Color::from_hex("#cdfd02", AlphaPosition::None).unwrap(),
            "greenish yellow".to_string(),
        ),
        (
            Color::from_hex("#2c6fbb", AlphaPosition::None).unwrap(),
            "medium blue".to_string(),
        ),
        (
            Color::from_hex("#c0737a", AlphaPosition::None).unwrap(),
            "dusty rose".to_string(),
        ),
        (
            Color::from_hex("#d6b4fc", AlphaPosition::None).unwrap(),
            "light violet".to_string(),
        ),
        (
            Color::from_hex("#020035", AlphaPosition::None).unwrap(),
            "midnight blue".to_string(),
        ),
        (
            Color::from_hex("#703be7", AlphaPosition::None).unwrap(),
            "bluish purple".to_string(),
        ),
        (
            Color::from_hex("#fd3c06", AlphaPosition::None).unwrap(),
            "red orange".to_string(),
        ),
        (
            Color::from_hex("#960056", AlphaPosition::None).unwrap(),
            "dark magenta".to_string(),
        ),
        (
            Color::from_hex("#40a368", AlphaPosition::None).unwrap(),
            "greenish".to_string(),
        ),
        (
            Color::from_hex("#03719c", AlphaPosition::None).unwrap(),
            "ocean blue".to_string(),
        ),
        (
            Color::from_hex("#fc5a50", AlphaPosition::None).unwrap(),
            "coral".to_string(),
        ),
        (
            Color::from_hex("#ffffc2", AlphaPosition::None).unwrap(),
            "cream".to_string(),
        ),
        (
            Color::from_hex("#7f2b0a", AlphaPosition::None).unwrap(),
            "reddish brown".to_string(),
        ),
        (
            Color::from_hex("#b04e0f", AlphaPosition::None).unwrap(),
            "burnt sienna".to_string(),
        ),
        (
            Color::from_hex("#a03623", AlphaPosition::None).unwrap(),
            "brick".to_string(),
        ),
        (
            Color::from_hex("#87ae73", AlphaPosition::None).unwrap(),
            "sage".to_string(),
        ),
        (
            Color::from_hex("#789b73", AlphaPosition::None).unwrap(),
            "grey green".to_string(),
        ),
        (
            Color::from_hex("#ffffff", AlphaPosition::None).unwrap(),
            "white".to_string(),
        ),
        (
            Color::from_hex("#98eff9", AlphaPosition::None).unwrap(),
            "robin's egg blue".to_string(),
        ),
        (
            Color::from_hex("#658b38", AlphaPosition::None).unwrap(),
            "moss green".to_string(),
        ),
        (
            Color::from_hex("#5a7d9a", AlphaPosition::None).unwrap(),
            "steel blue".to_string(),
        ),
        (
            Color::from_hex("#380835", AlphaPosition::None).unwrap(),
            "eggplant".to_string(),
        ),
        (
            Color::from_hex("#fffe7a", AlphaPosition::None).unwrap(),
            "light yellow".to_string(),
        ),
        (
            Color::from_hex("#5ca904", AlphaPosition::None).unwrap(),
            "leaf green".to_string(),
        ),
        (
            Color::from_hex("#d8dcd6", AlphaPosition::None).unwrap(),
            "light grey".to_string(),
        ),
        (
            Color::from_hex("#a5a502", AlphaPosition::None).unwrap(),
            "puke".to_string(),
        ),
        (
            Color::from_hex("#d648d7", AlphaPosition::None).unwrap(),
            "pinkish purple".to_string(),
        ),
        (
            Color::from_hex("#047495", AlphaPosition::None).unwrap(),
            "sea blue".to_string(),
        ),
        (
            Color::from_hex("#b790d4", AlphaPosition::None).unwrap(),
            "pale purple".to_string(),
        ),
        (
            Color::from_hex("#5b7c99", AlphaPosition::None).unwrap(),
            "slate blue".to_string(),
        ),
        (
            Color::from_hex("#607c8e", AlphaPosition::None).unwrap(),
            "blue grey".to_string(),
        ),
        (
            Color::from_hex("#0b4008", AlphaPosition::None).unwrap(),
            "hunter green".to_string(),
        ),
        (
            Color::from_hex("#ed0dd9", AlphaPosition::None).unwrap(),
            "fuchsia".to_string(),
        ),
        (
            Color::from_hex("#8c000f", AlphaPosition::None).unwrap(),
            "crimson".to_string(),
        ),
        (
            Color::from_hex("#ffff84", AlphaPosition::None).unwrap(),
            "pale yellow".to_string(),
        ),
        (
            Color::from_hex("#bf9005", AlphaPosition::None).unwrap(),
            "ochre".to_string(),
        ),
        (
            Color::from_hex("#d2bd0a", AlphaPosition::None).unwrap(),
            "mustard yellow".to_string(),
        ),
        (
            Color::from_hex("#ff474c", AlphaPosition::None).unwrap(),
            "light red".to_string(),
        ),
        (
            Color::from_hex("#0485d1", AlphaPosition::None).unwrap(),
            "cerulean".to_string(),
        ),
        (
            Color::from_hex("#ffcfdc", AlphaPosition::None).unwrap(),
            "pale pink".to_string(),
        ),
        (
            Color::from_hex("#040273", AlphaPosition::None).unwrap(),
            "deep blue".to_string(),
        ),
        (
            Color::from_hex("#a83c09", AlphaPosition::None).unwrap(),
            "rust".to_string(),
        ),
        (
            Color::from_hex("#90e4c1", AlphaPosition::None).unwrap(),
            "light teal".to_string(),
        ),
        (
            Color::from_hex("#516572", AlphaPosition::None).unwrap(),
            "slate".to_string(),
        ),
        (
            Color::from_hex("#fac205", AlphaPosition::None).unwrap(),
            "goldenrod".to_string(),
        ),
        (
            Color::from_hex("#d5b60a", AlphaPosition::None).unwrap(),
            "dark yellow".to_string(),
        ),
        (
            Color::from_hex("#363737", AlphaPosition::None).unwrap(),
            "dark grey".to_string(),
        ),
        (
            Color::from_hex("#4b5d16", AlphaPosition::None).unwrap(),
            "army green".to_string(),
        ),
        (
            Color::from_hex("#6b8ba4", AlphaPosition::None).unwrap(),
            "grey blue".to_string(),
        ),
        (
            Color::from_hex("#80f9ad", AlphaPosition::None).unwrap(),
            "seafoam".to_string(),
        ),
        (
            Color::from_hex("#a57e52", AlphaPosition::None).unwrap(),
            "puce".to_string(),
        ),
        (
            Color::from_hex("#a9f971", AlphaPosition::None).unwrap(),
            "spring green".to_string(),
        ),
        (
            Color::from_hex("#c65102", AlphaPosition::None).unwrap(),
            "dark orange".to_string(),
        ),
        (
            Color::from_hex("#e2ca76", AlphaPosition::None).unwrap(),
            "sand".to_string(),
        ),
        (
            Color::from_hex("#b0ff9d", AlphaPosition::None).unwrap(),
            "pastel green".to_string(),
        ),
        (
            Color::from_hex("#9ffeb0", AlphaPosition::None).unwrap(),
            "mint".to_string(),
        ),
        (
            Color::from_hex("#fdaa48", AlphaPosition::None).unwrap(),
            "light orange".to_string(),
        ),
        (
            Color::from_hex("#fe01b1", AlphaPosition::None).unwrap(),
            "bright pink".to_string(),
        ),
        (
            Color::from_hex("#c1f80a", AlphaPosition::None).unwrap(),
            "chartreuse".to_string(),
        ),
        (
            Color::from_hex("#36013f", AlphaPosition::None).unwrap(),
            "deep purple".to_string(),
        ),
        (
            Color::from_hex("#341c02", AlphaPosition::None).unwrap(),
            "dark brown".to_string(),
        ),
        (
            Color::from_hex("#b9a281", AlphaPosition::None).unwrap(),
            "taupe".to_string(),
        ),
        (
            Color::from_hex("#8eab12", AlphaPosition::None).unwrap(),
            "pea green".to_string(),
        ),
        (
            Color::from_hex("#9aae07", AlphaPosition::None).unwrap(),
            "puke green".to_string(),
        ),
        (
            Color::from_hex("#02ab2e", AlphaPosition::None).unwrap(),
            "kelly green".to_string(),
        ),
        (
            Color::from_hex("#7af9ab", AlphaPosition::None).unwrap(),
            "seafoam green".to_string(),
        ),
        (
            Color::from_hex("#137e6d", AlphaPosition::None).unwrap(),
            "blue green".to_string(),
        ),
        (
            Color::from_hex("#aaa662", AlphaPosition::None).unwrap(),
            "khaki".to_string(),
        ),
        (
            Color::from_hex("#610023", AlphaPosition::None).unwrap(),
            "burgundy".to_string(),
        ),
        (
            Color::from_hex("#014d4e", AlphaPosition::None).unwrap(),
            "dark teal".to_string(),
        ),
        (
            Color::from_hex("#8f1402", AlphaPosition::None).unwrap(),
            "brick red".to_string(),
        ),
        (
            Color::from_hex("#4b006e", AlphaPosition::None).unwrap(),
            "royal purple".to_string(),
        ),
        (
            Color::from_hex("#580f41", AlphaPosition::None).unwrap(),
            "plum".to_string(),
        ),
        (
            Color::from_hex("#8fff9f", AlphaPosition::None).unwrap(),
            "mint green".to_string(),
        ),
        (
            Color::from_hex("#dbb40c", AlphaPosition::None).unwrap(),
            "gold".to_string(),
        ),
        (
            Color::from_hex("#a2cffe", AlphaPosition::None).unwrap(),
            "baby blue".to_string(),
        ),
        (
            Color::from_hex("#c0fb2d", AlphaPosition::None).unwrap(),
            "yellow green".to_string(),
        ),
        (
            Color::from_hex("#be03fd", AlphaPosition::None).unwrap(),
            "bright purple".to_string(),
        ),
        (
            Color::from_hex("#840000", AlphaPosition::None).unwrap(),
            "dark red".to_string(),
        ),
        (
            Color::from_hex("#d0fefe", AlphaPosition::None).unwrap(),
            "pale blue".to_string(),
        ),
        (
            Color::from_hex("#3f9b0b", AlphaPosition::None).unwrap(),
            "grass green".to_string(),
        ),
        (
            Color::from_hex("#01153e", AlphaPosition::None).unwrap(),
            "navy".to_string(),
        ),
        (
            Color::from_hex("#04d8b2", AlphaPosition::None).unwrap(),
            "aquamarine".to_string(),
        ),
        (
            Color::from_hex("#c04e01", AlphaPosition::None).unwrap(),
            "burnt orange".to_string(),
        ),
        (
            Color::from_hex("#0cff0c", AlphaPosition::None).unwrap(),
            "neon green".to_string(),
        ),
        (
            Color::from_hex("#0165fc", AlphaPosition::None).unwrap(),
            "bright blue".to_string(),
        ),
        (
            Color::from_hex("#cf6275", AlphaPosition::None).unwrap(),
            "rose".to_string(),
        ),
        (
            Color::from_hex("#ffd1df", AlphaPosition::None).unwrap(),
            "light pink".to_string(),
        ),
        (
            Color::from_hex("#ceb301", AlphaPosition::None).unwrap(),
            "mustard".to_string(),
        ),
        (
            Color::from_hex("#380282", AlphaPosition::None).unwrap(),
            "indigo".to_string(),
        ),
        (
            Color::from_hex("#aaff32", AlphaPosition::None).unwrap(),
            "lime".to_string(),
        ),
        (
            Color::from_hex("#53fca1", AlphaPosition::None).unwrap(),
            "sea green".to_string(),
        ),
        (
            Color::from_hex("#8e82fe", AlphaPosition::None).unwrap(),
            "periwinkle".to_string(),
        ),
        (
            Color::from_hex("#cb416b", AlphaPosition::None).unwrap(),
            "dark pink".to_string(),
        ),
        (
            Color::from_hex("#677a04", AlphaPosition::None).unwrap(),
            "olive green".to_string(),
        ),
        (
            Color::from_hex("#ffb07c", AlphaPosition::None).unwrap(),
            "peach".to_string(),
        ),
        (
            Color::from_hex("#c7fdb5", AlphaPosition::None).unwrap(),
            "pale green".to_string(),
        ),
        (
            Color::from_hex("#ad8150", AlphaPosition::None).unwrap(),
            "light brown".to_string(),
        ),
        (
            Color::from_hex("#ff028d", AlphaPosition::None).unwrap(),
            "hot pink".to_string(),
        ),
        (
            Color::from_hex("#000000", AlphaPosition::None).unwrap(),
            "black".to_string(),
        ),
        (
            Color::from_hex("#cea2fd", AlphaPosition::None).unwrap(),
            "lilac".to_string(),
        ),
        (
            Color::from_hex("#001146", AlphaPosition::None).unwrap(),
            "navy blue".to_string(),
        ),
        (
            Color::from_hex("#0504aa", AlphaPosition::None).unwrap(),
            "royal blue".to_string(),
        ),
        (
            Color::from_hex("#e6daa6", AlphaPosition::None).unwrap(),
            "beige".to_string(),
        ),
        (
            Color::from_hex("#ff796c", AlphaPosition::None).unwrap(),
            "salmon".to_string(),
        ),
        (
            Color::from_hex("#6e750e", AlphaPosition::None).unwrap(),
            "olive".to_string(),
        ),
        (
            Color::from_hex("#650021", AlphaPosition::None).unwrap(),
            "maroon".to_string(),
        ),
        (
            Color::from_hex("#01ff07", AlphaPosition::None).unwrap(),
            "bright green".to_string(),
        ),
        (
            Color::from_hex("#35063e", AlphaPosition::None).unwrap(),
            "dark purple".to_string(),
        ),
        (
            Color::from_hex("#ae7181", AlphaPosition::None).unwrap(),
            "mauve".to_string(),
        ),
        (
            Color::from_hex("#06470c", AlphaPosition::None).unwrap(),
            "forest green".to_string(),
        ),
        (
            Color::from_hex("#13eac9", AlphaPosition::None).unwrap(),
            "aqua".to_string(),
        ),
        (
            Color::from_hex("#00ffff", AlphaPosition::None).unwrap(),
            "cyan".to_string(),
        ),
        (
            Color::from_hex("#d1b26f", AlphaPosition::None).unwrap(),
            "tan".to_string(),
        ),
        (
            Color::from_hex("#00035b", AlphaPosition::None).unwrap(),
            "dark blue".to_string(),
        ),
        (
            Color::from_hex("#c79fef", AlphaPosition::None).unwrap(),
            "lavender".to_string(),
        ),
        (
            Color::from_hex("#06c2ac", AlphaPosition::None).unwrap(),
            "turquoise".to_string(),
        ),
        (
            Color::from_hex("#033500", AlphaPosition::None).unwrap(),
            "dark green".to_string(),
        ),
        (
            Color::from_hex("#9a0eea", AlphaPosition::None).unwrap(),
            "violet".to_string(),
        ),
        (
            Color::from_hex("#bf77f6", AlphaPosition::None).unwrap(),
            "light purple".to_string(),
        ),
        (
            Color::from_hex("#89fe05", AlphaPosition::None).unwrap(),
            "lime green".to_string(),
        ),
        (
            Color::from_hex("#929591", AlphaPosition::None).unwrap(),
            "grey".to_string(),
        ),
        (
            Color::from_hex("#75bbfd", AlphaPosition::None).unwrap(),
            "sky blue".to_string(),
        ),
        (
            Color::from_hex("#ffff14", AlphaPosition::None).unwrap(),
            "yellow".to_string(),
        ),
        (
            Color::from_hex("#c20078", AlphaPosition::None).unwrap(),
            "magenta".to_string(),
        ),
        (
            Color::from_hex("#96f97b", AlphaPosition::None).unwrap(),
            "light green".to_string(),
        ),
        (
            Color::from_hex("#f97306", AlphaPosition::None).unwrap(),
            "orange".to_string(),
        ),
        (
            Color::from_hex("#029386", AlphaPosition::None).unwrap(),
            "teal".to_string(),
        ),
        (
            Color::from_hex("#95d0fc", AlphaPosition::None).unwrap(),
            "light blue".to_string(),
        ),
        (
            Color::from_hex("#e50000", AlphaPosition::None).unwrap(),
            "red".to_string(),
        ),
        (
            Color::from_hex("#653700", AlphaPosition::None).unwrap(),
            "brown".to_string(),
        ),
        (
            Color::from_hex("#ff81c0", AlphaPosition::None).unwrap(),
            "pink".to_string(),
        ),
        (
            Color::from_hex("#0343df", AlphaPosition::None).unwrap(),
            "blue".to_string(),
        ),
        (
            Color::from_hex("#15b01a", AlphaPosition::None).unwrap(),
            "green".to_string(),
        ),
        (
            Color::from_hex("#7e1e9c", AlphaPosition::None).unwrap(),
            "purple".to_string(),
        ),
    ])
}
