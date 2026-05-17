pragma Singleton
import QtQml 6.8
import QtQuick 6.8

QtObject {

    readonly property Spacing spacing: Spacing {}
    readonly property Radius radius: Radius {}
    readonly property Colors colors: Colors {}
    readonly property Typography typography: Typography {}
    readonly property SafeArea safeArea: SafeArea {}
    readonly property Animations animations: Animations {}

    component Spacing: QtObject {
        readonly property int unit: 4
        function s(multiplier) {
            return unit * multiplier;
        }
        // readonly property int xs: unit * 1
        // readonly property int sm: unit * 2
        // readonly property int md: unit * 4
        // readonly property int lg: unit * 6
        // readonly property int xl: unit * 8
        // readonly property int xl2: unit * 12
        // readonly property int xl3: unit * 16
    }

    component Radius: QtObject {
        readonly property int sm: 2
        readonly property int md: 4
        readonly property int lg: 8
        readonly property int xl: 12
        readonly property int xl2: 16
        readonly property int xl3: 24
        readonly property int full: 9999
    }

    component Colors: QtObject {
        // Background
        readonly property color background: "#0a0a0a"
        readonly property color foreground: "#fafafa"

        // Card/Surface
        readonly property color card: "#1a1a1a"
        readonly property color cardForeground: "#fafafa"

        // Popover
        readonly property color popover: "#1a1a1a"
        readonly property color popoverForeground: "#fafafa"

        // Selection
        readonly property color selection: Qt.rgba(255, 255, 255, 0.4)

        // Primary
        readonly property color primary: "#fafafa"
        readonly property color primaryForeground: "#18181b"

        // Secondary
        readonly property color secondary: "#27272a"
        readonly property color secondaryForeground: "#fafafa"

        // Muted
        readonly property color muted: "#27272a"
        readonly property color mutedForeground: "#a1a1aa"

        // Accent
        readonly property color accent: "#27272a"
        readonly property color accentForeground: "#fafafa"

        // Destructive
        readonly property color destructive: "#7f1d1d"
        readonly property color destructiveForeground: "#fafafa"

        // Border & Input
        readonly property color border: "#27272a"
        readonly property color input: "#27272a"
        readonly property color ring: "#d4d4d8"

        // Shadow
        readonly property color shadow: Qt.rgba(0, 0, 0, 0.6)
        readonly property color shadowLight: Qt.rgba(0, 0, 0, 0.2)

        // Semi-transparent surface
        readonly property color transparentSurface: Qt.rgba(255, 255, 255, 0.1)
    }

    component Typography: QtObject {
        readonly property string primaryFont: "Google Sans Flex"

        // readonly property font headingLarge: Qt.font({
        //     family: primaryFont,
        //     pixelSize: Theme.spacing.s(12),
        //     variableAxes: {
        //         "wght": 700
        //     }
        // })
    }

    component SafeArea: QtObject {
        readonly property int left: Theme.spacing.s(24)
        readonly property int right: Theme.spacing.s(24)
        readonly property int top: Theme.spacing.s(16)
        readonly property int bottom: Theme.spacing.s(16)
    }

    component Animations: QtObject {
        readonly property int duration: 250
        readonly property int easing: Easing.InOutQuad
    }
}
