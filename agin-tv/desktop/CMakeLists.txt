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

# Material Color Utilities
set(MCU_ROOT ${CMAKE_CURRENT_SOURCE_DIR}/external/material-color-utilities)

set(MCU_SOURCES
    ${MCU_ROOT}/cpp/blend/blend.cc
    ${MCU_ROOT}/cpp/blend/blend.h
    ${MCU_ROOT}/cpp/cam/cam.cc
    ${MCU_ROOT}/cpp/cam/cam.h
    ${MCU_ROOT}/cpp/cam/hct_solver.cc
    ${MCU_ROOT}/cpp/cam/hct_solver.h
    ${MCU_ROOT}/cpp/cam/hct.cc
    ${MCU_ROOT}/cpp/cam/hct.h
    ${MCU_ROOT}/cpp/cam/viewing_conditions.cc
    ${MCU_ROOT}/cpp/cam/viewing_conditions.h
    ${MCU_ROOT}/cpp/contrast/contrast.cc
    ${MCU_ROOT}/cpp/contrast/contrast.h
    ${MCU_ROOT}/cpp/dislike/dislike.cc
    ${MCU_ROOT}/cpp/dislike/dislike.h
    ${MCU_ROOT}/cpp/dynamiccolor/contrast_curve.h
    ${MCU_ROOT}/cpp/dynamiccolor/dynamic_color.cc
    ${MCU_ROOT}/cpp/dynamiccolor/dynamic_color.h
    ${MCU_ROOT}/cpp/dynamiccolor/dynamic_scheme.cc
    ${MCU_ROOT}/cpp/dynamiccolor/dynamic_scheme.h
    ${MCU_ROOT}/cpp/dynamiccolor/material_dynamic_colors.cc
    ${MCU_ROOT}/cpp/dynamiccolor/material_dynamic_colors.h
    ${MCU_ROOT}/cpp/dynamiccolor/tone_delta_pair.h
    ${MCU_ROOT}/cpp/dynamiccolor/variant.h
    ${MCU_ROOT}/cpp/palettes/core.h
    ${MCU_ROOT}/cpp/palettes/tones.cc
    ${MCU_ROOT}/cpp/palettes/tones.h
    ${MCU_ROOT}/cpp/quantize/celebi.cc
    ${MCU_ROOT}/cpp/quantize/celebi.h
    ${MCU_ROOT}/cpp/quantize/lab.cc
    ${MCU_ROOT}/cpp/quantize/lab.h
    ${MCU_ROOT}/cpp/quantize/wsmeans.cc
    ${MCU_ROOT}/cpp/quantize/wsmeans.h
    ${MCU_ROOT}/cpp/quantize/wu.cc
    ${MCU_ROOT}/cpp/quantize/wu.h
    ${MCU_ROOT}/cpp/scheme/scheme_content.cc
    ${MCU_ROOT}/cpp/scheme/scheme_content.h
    ${MCU_ROOT}/cpp/scheme/scheme_expressive.cc
    ${MCU_ROOT}/cpp/scheme/scheme_expressive.h
    ${MCU_ROOT}/cpp/scheme/scheme_fidelity.cc
    ${MCU_ROOT}/cpp/scheme/scheme_fidelity.h
    ${MCU_ROOT}/cpp/scheme/scheme_fruit_salad.cc
    ${MCU_ROOT}/cpp/scheme/scheme_fruit_salad.h
    ${MCU_ROOT}/cpp/scheme/scheme_monochrome.cc
    ${MCU_ROOT}/cpp/scheme/scheme_monochrome.h
    ${MCU_ROOT}/cpp/scheme/scheme_neutral.cc
    ${MCU_ROOT}/cpp/scheme/scheme_neutral.h
    ${MCU_ROOT}/cpp/scheme/scheme_rainbow.cc
    ${MCU_ROOT}/cpp/scheme/scheme_rainbow.h
    ${MCU_ROOT}/cpp/scheme/scheme_tonal_spot.cc
    ${MCU_ROOT}/cpp/scheme/scheme_tonal_spot.h
    ${MCU_ROOT}/cpp/scheme/scheme_vibrant.cc
    ${MCU_ROOT}/cpp/scheme/scheme_vibrant.h
    ${MCU_ROOT}/cpp/score/score.cc
    ${MCU_ROOT}/cpp/score/score.h
    ${MCU_ROOT}/cpp/temperature/temperature_cache.cc
    ${MCU_ROOT}/cpp/temperature/temperature_cache.h
    ${MCU_ROOT}/cpp/utils/utils.cc
    ${MCU_ROOT}/cpp/utils/utils.h
)

add_library(MaterialColorUtilities STATIC ${MCU_SOURCES})

target_include_directories(MaterialColorUtilities
    PUBLIC
        ${MCU_ROOT}
)

target_compile_options(MaterialColorUtilities PRIVATE -include optional)

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
        qml/components/Hero.qml
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
        MaterialColorUtilities
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
