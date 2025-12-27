cmake_minimum_required(VERSION 3.16)

project(AginTV VERSION 0.1 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 REQUIRED COMPONENTS Quick Gui Core Widgets)
find_package(SDL2 REQUIRED)

qt_policy(SET QTP0001 NEW)

qt_standard_project_setup(REQUIRES 6.8)

set(QT_QML_GENERATE_QMLLS_INI ON)

set(QJOYSTICKS_ROOT ${CMAKE_CURRENT_SOURCE_DIR}/external/QJoysticks)
file(RELATIVE_PATH QJOYSTICKS_QRC ${CMAKE_CURRENT_SOURCE_DIR}
    ${QJOYSTICKS_ROOT}/etc/resources/qjoysticks-res.qrc)

set(QJOYSTICKS_SOURCES
    ${QJOYSTICKS_ROOT}/src/QJoysticks.cpp
    ${QJOYSTICKS_ROOT}/src/QJoysticks/Android_Joystick.cpp
    ${QJOYSTICKS_ROOT}/src/QJoysticks/SDL_Joysticks.cpp
    ${QJOYSTICKS_ROOT}/src/QJoysticks/VirtualJoystick.cpp
    ${QJOYSTICKS_ROOT}/src/QJoysticks.h
    ${QJOYSTICKS_ROOT}/src/QJoysticks/JoysticksCommon.h
    ${QJOYSTICKS_ROOT}/src/QJoysticks/Android_Joystick.h
    ${QJOYSTICKS_ROOT}/src/QJoysticks/SDL_Joysticks.h
    ${QJOYSTICKS_ROOT}/src/QJoysticks/VirtualJoystick.h
)

add_library(QJoysticks STATIC ${QJOYSTICKS_SOURCES})

qt_add_resources(QJoysticks "qjoysticks_resources"
    FILES
        ${QJOYSTICKS_QRC}
)

set(_QJOYSTICKS_SDL_TARGET "")
if(TARGET SDL2::SDL2)
    set(_QJOYSTICKS_SDL_TARGET SDL2::SDL2)
elseif(TARGET SDL2::SDL2-static)
    set(_QJOYSTICKS_SDL_TARGET SDL2::SDL2-static)
elseif(SDL2_LIBRARIES)
    set(_QJOYSTICKS_SDL_TARGET ${SDL2_LIBRARIES})
else()
    message(FATAL_ERROR "SDL2 package did not expose link targets or library list")
endif()

target_link_libraries(QJoysticks
    PUBLIC Qt6::Core Qt6::Gui Qt6::Widgets
    PRIVATE ${_QJOYSTICKS_SDL_TARGET}
)

target_compile_definitions(QJoysticks PRIVATE SDL_SUPPORTED)

target_include_directories(QJoysticks
    PUBLIC
        ${QJOYSTICKS_ROOT}/src
        ${QJOYSTICKS_ROOT}/src/QJoysticks
    PRIVATE
        ${SDL2_INCLUDE_DIRS}
)

qt_add_executable(appAginTV
    main.cpp
)

set_source_files_properties(qml/theme/Theme.qml
    PROPERTIES QT_QML_SINGLETON_TYPE TRUE
)

qt_add_qml_module(appAginTV
    URI AginTV
    QML_FILES
        qml/Main.qml
        qml/theme/Theme.qml
        qml/components/Cover.qml
        qml/components/RoundedImage.qml
    SOURCES
        input/inputaction.h
        input/inputprovider.h input/inputprovider.cpp
        input/keyboardinputprovider.h input/keyboardinputprovider.cpp
        navigation/navigable.h navigation/navigable.cpp
        input/inputdispatcher.h input/inputdispatcher.cpp
        input/gamepadinputprovider.h input/gamepadinputprovider.cpp
        input/providershistory.h
        navigation/navigationengine.h navigation/navigationengine.cpp
        navigation/navigationmanager.h navigation/navigationmanager.cpp
        navigation/navigationnode.h
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
    PRIVATE
        Qt6::Quick
        QJoysticks
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
