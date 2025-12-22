cmake_minimum_required(VERSION 3.16)

project(AginTV VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 REQUIRED COMPONENTS Quick)

qt_standard_project_setup(REQUIRES 6.8)

set(QT_QML_GENERATE_QMLLS_INI ON)

qt_add_executable(appAginTV
    main.cpp
)

qt_add_qml_module(appAginTV
    URI AginTV
    QML_FILES
        qml/Main.qml
        SOURCES input/inputaction.h
        SOURCES input/inputprovider.h
        SOURCES input/inputprovider.h input/inputprovider.cpp
        SOURCES input/keyboardinputprovider.h input/keyboardinputprovider.cpp
        SOURCES navigation/navigable.h navigation/navigable.cpp
)

# Qt for iOS sets MACOSX_BUNDLE_GUI_IDENTIFIER automatically since Qt 6.1.
# If you are developing for iOS or macOS you should consider setting an
# explicit, fixed bundle identifier manually though.
set_target_properties(appAginTV PROPERTIES
#    MACOSX_BUNDLE_GUI_IDENTIFIER com.example.appAginTV
    MACOSX_BUNDLE_BUNDLE_VERSION ${PROJECT_VERSION}
    MACOSX_BUNDLE_SHORT_VERSION_STRING ${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}
    MACOSX_BUNDLE TRUE
    WIN32_EXECUTABLE TRUE
)

target_link_libraries(appAginTV
    PRIVATE Qt6::Quick
)

target_include_directories(appAginTV PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/navigation
    ${CMAKE_CURRENT_SOURCE_DIR}/input
)

include(GNUInstallDirs)
install(TARGETS appAginTV
    BUNDLE DESTINATION .
    LIBRARY DESTINATION ${CMAKE_INSTALL_LIBDIR}
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR}
)
