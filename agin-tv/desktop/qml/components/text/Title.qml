import QtQuick
import AginTV

BaseText {
    id: root

    enum Size {
        Large,
        Medium,
        Small
    }
    property int size: Title.Size.Large

    lineHeightMode: Text.FixedHeight

    font.pixelSize: {
        switch (size) {
        case Title.Size.Large:
            return Theme.spacing.s(5.5);
        case Title.Size.Medium:
            return Theme.spacing.s(4);
        case Title.Size.Small:
            return Theme.spacing.s(3.5);
        default:
            return Theme.spacing.s(5.5);
        }
    }

    lineHeight: {
        switch (size) {
        case Title.Size.Large:
            return Theme.spacing.s(7);
        case Title.Size.Medium:
            return Theme.spacing.s(6);
        case Title.Size.Small:
            return Theme.spacing.s(5);
        default:
            return Theme.spacing.s(7);
        }
    }

    font.letterSpacing: {
        switch (size) {
        case Title.Size.Large:
            return 0;
        case Title.Size.Medium:
            return 0.15;
        case Title.Size.Small:
            return 0.1;
        default:
            return 0;
        }
    }

    font.variableAxes: ({
            "ROND": 0,
            "wght": size === Title.Size.Large ? 400 : 500
        })
}
