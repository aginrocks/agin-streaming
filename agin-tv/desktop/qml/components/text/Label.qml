import QtQuick
import AginTV

BaseText {
    id: root

    enum Size {
        Large,
        Medium,
        Small
    }
    property int size: Label.Size.Large

    lineHeightMode: Text.FixedHeight

    font.pixelSize: {
        switch (size) {
        case Label.Size.Large:
            return Theme.spacing.s(3.5);
        case Label.Size.Medium:
            return Theme.spacing.s(3);
        case Label.Size.Small:
            return Theme.spacing.s(2.75);
        default:
            return Theme.spacing.s(3.5);
        }
    }

    lineHeight: {
        switch (size) {
        case Label.Size.Large:
            return Theme.spacing.s(5);
        case Label.Size.Medium:
            return Theme.spacing.s(4);
        case Label.Size.Small:
            return Theme.spacing.s(4);
        default:
            return Theme.spacing.s(5);
        }
    }

    font.letterSpacing: {
        switch (size) {
        case Label.Size.Large:
            return 0.1;
        case Label.Size.Medium:
            return 0.5;
        case Label.Size.Small:
            return 0.5;
        default:
            return 0.1;
        }
    }

    font.variableAxes: ({
            "ROND": 0,
            "wght": 500
        })
}
