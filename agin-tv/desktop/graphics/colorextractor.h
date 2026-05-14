#pragma once

#include <qtmetamacros.h>

#include <QObject>

class ColorExtractor: public QObject {
public:
    Q_OBJECT
    Q_PROPERTY(QVariantMap palette READ palette NOTIFY paletteChanged)
    Q_PROPERTY(
        bool isDarkMode READ isDarkMode WRITE setIsDarkMode NOTIFY
            darkModeChanged
    )
};