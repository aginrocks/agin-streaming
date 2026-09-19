import QtQuick
import QtQuick.Window
import QtQuick.Controls
import QtQuick.Layouts
import AginTV
import "js/request.js" as XHR

Window {
    id: window
    width: 960
    height: 540
    visible: true
    title: qsTr("Agin TV Navigation Demo")
    color: Theme.colors.background

    property string source: "https://image.tmdb.org/t/p/original/6bzabqH399ioM3nZScwZtzGaHIy.jpg"
    property real weight: 400
    Login {
        z: 1
        anchors.centerIn: parent
    }
    Hero {
        z: -1
        activeSource: window.source
    }
    /*
    TabBar {
        anchors {
            top: parent.top
            left: parent.right
            topMargin: Theme.safeArea.top
            leftMargin: Theme.safeArea.left
        }
    }
     */
}
