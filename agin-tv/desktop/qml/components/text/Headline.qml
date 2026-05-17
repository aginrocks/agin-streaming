import QtQuick
import AginTV

BaseText {
    id: root

    enum Size {
        Large,
        Medium,
        Small
    }
    property int size: Headline.Size.Large

    lineHeightMode: Text.FixedHeight

    font.pixelSize: {
        switch (size) {
        case Headline.Size.Large:
            return Theme.spacing.s(8);
        case Headline.Size.Medium:
            return Theme.spacing.s(7);
        case Headline.Size.Small:
            return Theme.spacing.s(6);
        default:
            return Theme.spacing.s(8);
        }
    }

    lineHeight: {
        switch (size) {
        case Headline.Size.Large:
            return Theme.spacing.s(10);
        case Headline.Size.Medium:
            return Theme.spacing.s(9);
        case Headline.Size.Small:
            return Theme.spacing.s(8);
        default:
            return Theme.spacing.s(10);
        }
    }

    font.variableAxes: ({
            "ROND": 0,
            "wght": 400
        })
}
