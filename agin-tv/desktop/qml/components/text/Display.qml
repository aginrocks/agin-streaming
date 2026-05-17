import QtQuick
import AginTV

BaseText {
    id: root

    enum Size {
        Large,
        Medium,
        Small
    }
    property int size: Display.Size.Large

    lineHeightMode: Text.FixedHeight

    font.pixelSize: {
        switch (size) {
        case Display.Size.Large:
            return Theme.spacing.s(14.25);
        case Display.Size.Medium:
            return Theme.spacing.s(11.25);
        case Display.Size.Small:
            return Theme.spacing.s(9);
        default:
            return Theme.spacing.s(14.25);
        }
    }

    lineHeight: {
        switch (size) {
        case Display.Size.Large:
            return Theme.spacing.s(16);
        case Display.Size.Medium:
            return Theme.spacing.s(13);
        case Display.Size.Small:
            return Theme.spacing.s(11);
        default:
            return Theme.spacing.s(16);
        }
    }

    font.letterSpacing: size === Display.Size.Large ? -0.25 : 0

    font.variableAxes: ({
            "ROND": 0,
            "wght": 500
        })
}
