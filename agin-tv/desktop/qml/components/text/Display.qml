import QtQuick
import AginTV

BaseText {
    id: root

    enum Level {
        H1 = 1,
        H2 = 2,
        H3 = 3,
        H4 = 4,
        H5 = 5,
        H6 = 6
    }

    property int level: Heading.Level.H1

    function fontSize(level) {
        switch (level) {
        case Heading.Level.H1:
            return Theme.spacing.s(12);
        case Heading.Level.H2:
            return Theme.spacing.s(10);
        case Heading.Level.H3:
            return Theme.spacing.s(8);
        case Heading.Level.H4:
            return Theme.spacing.s(7);
        case Heading.Level.H5:
            return Theme.spacing.s(6);
        case Heading.Level.H6:
            return Theme.spacing.s(5);
        default:
            return Theme.spacing.s(6);
        }
    }

    function fontWeight(level) {
        switch (level) {
        case Heading.Level.H1:
        case Heading.Level.H2:
            return 600;
        case Heading.Level.H3:
            return 550;
        default:
            return 500;
        }
    }

    font.pixelSize: fontSize(level)
    font.variableAxes: {
        "ROND": 100,
        "wght": fontWeight(level)
    }
}
