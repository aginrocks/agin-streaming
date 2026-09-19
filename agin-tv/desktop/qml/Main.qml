import QtQuick 6.8
import QtQuick.Window 6.8
import QtQuick.VirtualKeyboard 6.8
import QtQuick.Controls 6.8
import QtQuick.Layouts 6.8
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

    Behavior on weight {
        NumberAnimation {
            duration: 300
        }
    }

    Fonts {}

    Hero {
        activeSource: window.source

        CoverRow {}
    }

    TabBar {
        anchors {
            top: parent.top
            left: parent.left
            topMargin: Theme.safeArea.top
            leftMargin: Theme.safeArea.left
        }
    }
}
