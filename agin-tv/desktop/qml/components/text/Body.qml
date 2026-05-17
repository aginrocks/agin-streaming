import QtQuick
import AginTV

BaseText {
    id: root

    enum Size {
        Large,
        Medium,
        Small
    }
    property int size: Body.Size.Large

    lineHeightMode: Text.FixedHeight

    font.pixelSize: {
        switch (size) {
        case Body.Size.Large:
            return Theme.spacing.s(4);
        case Body.Size.Medium:
            return Theme.spacing.s(3.5);
        case Body.Size.Small:
            return Theme.spacing.s(3);
        default:
            return Theme.spacing.s(4);
        }
    }

    lineHeight: {
        switch (size) {
        case Body.Size.Large:
            return Theme.spacing.s(6);
        case Body.Size.Medium:
            return Theme.spacing.s(5);
        case Body.Size.Small:
            return Theme.spacing.s(4);
        default:
            return Theme.spacing.s(6);
        }
    }

    font.letterSpacing: {
        switch (size) {
        case Body.Size.Large:
            return 0.5;
        case Body.Size.Medium:
            return 0.25;
        case Body.Size.Small:
            return 0.4;
        default:
            return 0.5;
        }
    }

    font.variableAxes: ({
            "ROND": 0,
            "wght": 400
        })
}
