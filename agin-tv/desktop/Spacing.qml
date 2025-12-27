pragma Singleton
import QtQml 6.8
import QtQuick 6.8

QtObject {
    readonly property int unit: 4
    function s(multiplier) {
        return unit * multiplier;
    }
    readonly property int xs: unit * 1
    readonly property int sm: unit * 2
    readonly property int md: unit * 4
    readonly property int lg: unit * 6
    readonly property int xl: unit * 8
    readonly property int xl2: unit * 12
    readonly property int xl3: unit * 16
}
